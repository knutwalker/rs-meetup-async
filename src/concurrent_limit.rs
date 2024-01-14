use std::sync::Arc;

use eyre::Result;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    rs_meetup_async::run_async(concurrent_limit).await
}

async fn concurrent_limit(lines: Vec<&'static str>) -> Result<()> {
    let num_threads = rs_meetup_async::concurrency()?.unwrap_or(8);

    let client = Arc::new((
        tokio::sync::Semaphore::new(num_threads),
        reqwest::Client::new(),
    ));
    let mut tasks = Vec::with_capacity(lines.len());

    for word in lines {
        let client = Arc::clone(&client);
        let task = tokio::spawn(async move {
            let (permit, client) = Arc::as_ref(&client);
            let permit = permit.acquire().await?;

            let sponge = client
                .get("http://localhost:1337/sponge")
                .query(&[("word", word)])
                .send()
                .await?
                .text()
                .await?;

            drop(permit);

            rs_meetup_async::validate(word, &sponge)
        });
        tasks.push(task);
    }

    for task in tasks {
        task.await??;
    }

    Ok(())
}

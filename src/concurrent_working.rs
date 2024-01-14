use std::sync::Arc;

use eyre::Result;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    rs_meetup_async::run_async(concurrent_working).await
}

async fn concurrent_working(lines: Vec<&'static str>) -> Result<()> {
    let client = Arc::new(reqwest::Client::new());
    let mut tasks = Vec::with_capacity(lines.len());

    for word in lines {
        let client = Arc::clone(&client);
        let task = tokio::spawn(async move {
            let sponge = client
                .get("http://localhost:1337/sponge")
                .query(&[("word", word)])
                .send()
                .await?
                .text()
                .await?;
            rs_meetup_async::validate(word, &sponge)
        });
        tasks.push(task);
    }

    for task in tasks {
        task.await??;
    }

    Ok(())
}

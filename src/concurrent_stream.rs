use eyre::Result;
use futures::{stream, StreamExt as _, TryStreamExt as _};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    rs_meetup_async::run_async(concurrent_stream).await
}

async fn concurrent_stream(lines: Vec<&'static str>) -> Result<()> {
    let num_threads = rs_meetup_async::concurrency()?;

    let client = reqwest::Client::new();

    stream::iter(lines)
        .map(Ok)
        .try_for_each_concurrent(num_threads, {
            let client = &client;
            move |word| async move {
                let sponge = client
                    .get("http://localhost:1337/sponge")
                    .query(&[("word", word)])
                    .send()
                    .await?
                    .text()
                    .await?;
                rs_meetup_async::validate(word, &sponge)
            }
        })
        .await
}

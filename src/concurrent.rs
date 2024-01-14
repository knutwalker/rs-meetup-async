use std::future::Future;

use eyre::Result;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    rs_meetup_async::run_async(concurrent).await
}

async fn concurrent(lines: Vec<&str>) -> Result<()> {
    let client = reqwest::Client::new();
    for word in lines {
        let sponge = client
            .get("http://localhost:1337/sponge")
            .query(&[("word", word)])
            .send()
            .await?
            .text()
            .await?;
        rs_meetup_async::validate(word, &sponge)?;
    }
    Ok(())
}

#[allow(clippy::manual_async_fn)]
fn _concurrent_desugared(lines: Vec<&str>) -> impl Future<Output = Result<()>> + '_ {
    async move {
        let client = reqwest::Client::new();
        for word in lines {
            let sponge = client
                .get("http://localhost:1337/sponge")
                .query(&[("word", word)])
                .send()
                .await?
                .text()
                .await?;
            rs_meetup_async::validate(word, &sponge)?;
        }
        Ok(())
    }
}

use eyre::Result;

fn main() -> Result<()> {
    rs_meetup_async::run(sync)
}

fn sync(lines: Vec<&str>) -> Result<()> {
    let client = reqwest::blocking::Client::new();

    lines.into_iter().try_for_each(|word| -> Result<_> {
        let sponge = client
            .get("http://localhost:1337/sponge")
            .query(&[("word", word)])
            .send()?
            .text()?;
        rs_meetup_async::validate(word, &sponge)
    })
}

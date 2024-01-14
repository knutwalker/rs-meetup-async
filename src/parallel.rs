use eyre::Result;
use rayon::iter::{IntoParallelIterator as _, ParallelIterator as _};

fn main() -> Result<()> {
    rs_meetup_async::run(parallel)
}

fn parallel(lines: Vec<&str>) -> Result<()> {
    let client = reqwest::blocking::Client::new();

    lines.into_par_iter().try_for_each(|word| -> Result<_> {
        let sponge = client
            .get("http://localhost:1337/sponge")
            .query(&[("word", word)])
            .send()?
            .text()?;
        rs_meetup_async::validate(word, &sponge)
    })
}

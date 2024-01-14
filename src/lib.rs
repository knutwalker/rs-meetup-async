use std::{future::Future, time::Instant};

use eyre::Result;
use progress_bar::{
    inc_progress_bar, init_progress_bar, print_progress_bar_final_info, print_progress_bar_info,
    set_progress_bar_action, Color, Style,
};
use rand::seq::SliceRandom;

pub fn spongeify(input: &str) -> String {
    input
        .chars()
        .enumerate()
        .map(|(i, c)| match c {
            'a'..='z' | 'A'..='Z' => {
                if i % 2 == 0 {
                    c.to_ascii_uppercase()
                } else {
                    c.to_ascii_lowercase()
                }
            }
            _ => c,
        })
        .collect()
}

pub fn validate(word: &str, actual: &str) -> Result<()> {
    let sponge = spongeify(word);
    if sponge != actual {
        print_progress_bar_info(
            "Failed",
            &format!(
                "spongeify(\"{}\") returned \"{}\" but expected \"{}\"",
                word, actual, sponge
            ),
            Color::Red,
            Style::Bold,
        );
    }
    inc_progress_bar();
    Ok(())
}

pub fn run(body: impl FnOnce(Vec<&str>) -> Result<()>) -> Result<()> {
    let lines = prepare();

    let timer = timer(lines.len());

    let res = body(lines);

    timer(res)
}

pub async fn run_async<F: Future<Output = Result<()>>>(
    body: impl FnOnce(Vec<&'static str>) -> F,
) -> Result<()> {
    let lines = prepare();

    let timer = timer(lines.len());

    let res = body(lines).await;

    timer(res)
}

fn prepare() -> Vec<&'static str> {
    let limit = std::env::args()
        .nth(1)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(100_000);

    let mut lines = include_str!("../words.txt")
        .lines()
        .take(limit)
        .map(str::trim)
        .collect::<Vec<_>>();

    while lines.len() < limit {
        let extra = (limit - lines.len()).min(lines.len());
        lines.extend_from_within(0..extra);
    }

    assert_eq!(lines.len(), limit);

    lines.shuffle(&mut rand::thread_rng());

    init_progress_bar(limit);
    set_progress_bar_action("Running", Color::LightCyan, Style::Normal);

    lines
}

fn timer(words: usize) -> impl FnOnce(Result<()>) -> Result<()> {
    let start = Instant::now();
    move |res| finish(words, res, start)
}

fn finish(words: usize, res: Result<()>, start: Instant) -> Result<()> {
    if let Ok(()) = res {
        let elapsed = start.elapsed();
        let thrpt = words as f64 / elapsed.as_secs_f64();
        print_progress_bar_final_info(
            "Success",
            &format!("Took: {elapsed:?} ~~ {thrpt:.2} req/s"),
            Color::Green,
            Style::Normal,
        );
    }

    res
}

pub fn concurrency() -> Result<Option<usize>> {
    Ok(std::env::var("RAYON_RS_NUM_CPUS")
        .ok()
        .map(|s| s.parse::<usize>())
        .transpose()?
        .or_else(|| std::thread::available_parallelism().ok().map(|n| n.get())))
}

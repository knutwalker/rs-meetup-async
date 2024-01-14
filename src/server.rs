use axum::{extract::Query, response::IntoResponse, routing::get, Router};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .route("/sponge", get(spongeify));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:1337")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> impl IntoResponse {
    "Call /sponge?word=foo to spongeify foo"
}

#[derive(Deserialize)]
struct Sponge {
    word: String,
}

async fn spongeify(input: Query<Sponge>) -> String {
    tokio::time::sleep(std::time::Duration::from_micros(1337)).await;
    rs_meetup_async::spongeify(&input.word)
}

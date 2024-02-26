use awc::{Client, ws};
use futures::StreamExt;
use std::thread;
use std::time::{Duration, Instant};
use std::env;

#[actix_rt::main]
async fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let (_resp, mut connection) = Client::new()
        .ws("ws://localhost:8080/frontend")
        .connect()
        .await
        .expect("Failed to connect");

    loop {
        // connection
        // .send(ws::Message::Text("Echo".into()))
        // .await
        // .expect("Failed to send message");

        let response = connection.next().await.expect("Failed to receive message").expect("Failed to receive message");

        println!("{:?}", response);

        thread::sleep(Duration::from_secs(1));
    }
}
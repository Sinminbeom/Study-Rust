use awc::{Client, ws};
use futures::StreamExt;
use std::thread;
use std::time::{Duration, Instant};

#[actix_rt::main]
async fn main() {
    let (_resp, mut connection) = Client::new()
        .ws("ws://localhost:8080/vpu1")
        .connect()
        .await
        .expect("Failed to connect");

    loop {
        // connection
        // .send(ws::Message::Text("Echo".into()))
        // .await
        // .expect("Failed to send message");

        let response = connection.next().await.unwrap().unwrap();

        println!("{:?}", response);

        thread::sleep(Duration::from_secs(1));
    }
}
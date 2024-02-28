use awc::{Client, ws};
use futures::{SinkExt, StreamExt};
use std::thread;
use std::time::{Duration, Instant};
use std::env;

#[actix_rt::main]
async fn main() {
    // env::set_var("RUST_BACKTRACE", "1");

    let (_resp, mut connection) = Client::new()
        .ws("ws://localhost:8080/vpu3")
        .connect()
        .await
        .expect("Failed to connect");

    loop {
        // 서버로부터 메시지 수신
        if let Some(Ok(response)) = connection.next().await {

            println!("{:?}", response);

            // 수신한 메시지 출력
            connection
                .send(ws::Message::Ping("vpu3".into()))
                .await
                .expect("Failed to send message");

            thread::sleep(Duration::from_secs(1));

        } else {
            // 만약 수신한 메시지가 없거나 오류가 발생한 경우 처리
            println!("Failed to receive message or connection closed!");
            break; // 혹은 원하는 처리 수행 후 루프를 종료할 수 있음
        }
    }
}
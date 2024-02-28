use std::thread;
use std::time::Duration;
use awc::{Client, ws};
use futures::{SinkExt, StreamExt};

#[actix_rt::main]
async fn main() {
    // 웹소켓 클라이언트 생성 및 서버와의 연결 설정
    let (_resp, mut connection) = Client::new()
        .ws("ws://localhost:8080/vpu2")
        .connect()
        .await
        .expect("Failed to connect");

    // 메시지 수신을 위한 무한 루프
    loop {
        // 서버로부터 메시지 수신
        if let Some(Ok(response)) = connection.next().await {

            println!("{:?}", response);

            // 수신한 메시지 출력
            connection
                .send(ws::Message::Ping("vpu2".into()))
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

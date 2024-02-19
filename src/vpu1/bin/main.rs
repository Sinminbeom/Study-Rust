use awc::{Client, error::WsProtocolError};
use actix_rt::System;
use futures::stream::StreamExt;

#[actix_web::main]
async fn main() -> Result<(), WsProtocolError> {
    // Actix 웹 클라이언트 생성
    let client = Client::new();

    // WebSocket URL 설정
    let url = "ws://localhost:8080/client";

    // Actix 웹 소켓 연결
    let (response, mut connection) = client.ws(url).connect().await?;

    println!("Connected to: {}", response.url());

    // 웹 소켓 메시지 수신 및 처리
    let _ = System::new().block_on(async {
        // 소켓 연결이 유지되는 동안 메시지 수신
        while let Some(msg) = connection.next().await {
            println!("Received: {:?}", msg);
        }
    });

    Ok(())
}
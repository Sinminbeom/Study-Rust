use awc::{Client, ws};
use futures::SinkExt;
use futures::StreamExt;

#[actix_rt::main]
async fn main() {
    let (_resp, mut connection) = Client::new()
        .ws("ws://localhost:8080/vpu1")
        .connect()
        .await
        .unwrap();

    connection
        .send(ws::Message::Text("Echo".into()))
        .await
        .unwrap();

    let response = connection.next().await.unwrap().unwrap();

    println!("{:?}", response);
}
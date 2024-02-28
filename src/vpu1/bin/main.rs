mod messages;

use awc::{Client, ws};
use futures::{SinkExt, StreamExt};
use std::thread;
use std::time::{Duration, Instant};
use std::env;
use std::error::Error;
use std::path::Path;
use actix_web_actors::ws::{Frame};
use crate::messages::ClientActorMessage;
use pcap::*;
use std::error;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketOwned {
    pub header: PacketHeader,
    pub data: Box<[u8]>,
}

pub struct Codec;

impl PacketCodec for Codec {
    type Item = PacketOwned;

    fn decode(&mut self, packet: Packet) -> Self::Item {
        PacketOwned {
            header: *packet.header,
            data: packet.data.into(),
        }
    }
}

fn test() -> Result<(), Box<dyn error::Error>> {
    let device = Device::lookup()?.ok_or("no device available")?;

    // get the default Device
    println!("Using device {}", device.name);

    let cap = Capture::from_device(device)?.immediate_mode(true).open()?;

    for packet in cap.iter(Codec) {
        let packet = packet?;

        println!("{:?}", packet);
    }

    Ok(())
}

#[actix_rt::main]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_BACKTRACE", "1");

    let (_resp, mut connection) = Client::new()
        .ws("ws://localhost:8080/vpu1")
        .connect()
        .await
        .expect("Failed to connect");

    // 연결이 성공적으로 이루어지면 웹소켓 서버로부터 메시지를 지속적으로 수신합니다.
    while let Some(msg) = connection.next().await {
        // 메시지가 도착하면 처리합니다.
        match msg {
            Ok(Frame::Text(text)) => {
                // Bytes를 &str로 변환
                let text_str = std::str::from_utf8(&text).unwrap();
                if let Ok(clientActorMessage) = serde_json::from_str::<ClientActorMessage>(&text_str) {
                    // self.lobby_addr.do_send(ClientActorMessage{
                    //     device: clientActorMessage.device,
                    //     command: clientActorMessage.command
                    // })

                    // let msg_json = serde_json::to_string(&msg).unwrap(); // 예시로 serde_json 사용
                    //
                    // connection
                    //     .send(ws::Message::Text())
                    //     .await
                    //     .expect("Failed to send message");
                    println!("Received message: {:?}", clientActorMessage);


                    // let file_path = "D:\\dt\\cams\\one_min\\am20.pcap";

                    // let _ = test();


                    let mut cap = pcap::Capture::from_device("any").unwrap().open().unwrap();

                    // filter out all packets that don't have 127.0.0.1 as a source or destination.
                    cap.filter("host 127.0.0.1", false).unwrap();

                    while let Ok(packet) = cap.next() {
                        println!("got packet! {:?}", packet);
                    }

                } else {
                    println!("Failed to decode JSON message.");
                }
                // 여기서 메시지를 처리하는 로직을 추가할 수 있습니다.
            },
            Ok(Frame::Ping(text)) => {
                // 수신한 메시지 출력
                connection
                    .send(ws::Message::Ping("vpu1".into()))
                    .await
                    .expect("Failed to send message");
            },
            Ok(Frame::Close(text)) => {
                println!("fdfdfd");
            }
            Err(e) => {
                eprintln!("Error receiving message: {:?}", e);
                // 오류 처리 로직을 추가할 수 있습니다.
            },
            _ => {
                println!("fdfdfd");
            }
        }
    }

}

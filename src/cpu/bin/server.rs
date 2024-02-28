use actix::{Actor, ActorContext, AsyncContext, StreamHandler, Handler, Message, Addr, WrapFuture, ActorFutureExt, fut, ContextFutureSpawner, Running};
use actix_web_actors::ws;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::lobby::Lobby;
use crate::messages::{ClientActorMessage, Connect, Disconnect, WsMessage};


const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(1);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// Define HTTP actor
pub struct WsConn {
    device: String,
    lobby_addr: Addr<Lobby>,
    hb: Instant,
    id: Uuid
}

impl Actor for WsConn {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();
        self.lobby_addr
            .send(Connect {
                addr: addr.recipient(),
                device: self.device.clone(),
                self_id: self.id,
            })
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(_res) => (),
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.lobby_addr.do_send(Disconnect {
            id: self.id,
            device: self.device.clone()
        });
        Running::Stop
    }
}

impl WsConn {
    pub fn new(device: String, lobby: Addr<Lobby>) -> WsConn {
        WsConn {
            id: Uuid::new_v4(),
            device,
            hb: Instant::now(),
            lobby_addr: lobby
        }
    }

    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Disconnecting failed heartbeat");
                ctx.stop();
                return;
            }
            ctx.ping(b"hi");
        });
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConn {

    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                // ctx.pong(&msg);
            },
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            },
            Ok(ws::Message::Text(text)) => {

                if let Ok(clientActorMessage) = serde_json::from_str::<ClientActorMessage>(&text) {
                    self.lobby_addr.do_send(ClientActorMessage{
                        device: clientActorMessage.device,
                        command: clientActorMessage.command
                    })
                } else {
                    println!("Failed to decode JSON message.");
                }


            },
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

impl Handler<ClientActorMessage> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: ClientActorMessage, ctx: &mut Self::Context) {
        let msg_json = serde_json::to_string(&msg).unwrap(); // 예시로 serde_json 사용

        // JSON 문자열을 ctx.text 함수에 전달
        ctx.text(msg_json);
    }
}

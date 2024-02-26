use actix::{Actor, ActorContext, AsyncContext, StreamHandler, Handler, Message, Addr, WrapFuture, ActorFutureExt, fut, ContextFutureSpawner};
use actix_web_actors::ws;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::lobby::Lobby;
use crate::messages::{Connect, WsMessage};


const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(100);
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
}

impl WsConn {
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
    fn handle_message(&self, text: fn()) {
        text();
    }
}

fn test() {
    println!("fdfd");
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConn {

    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            // Ok(ws::Message::Text(text)) => WsConn.handle_message(test),
            _ => (),
        }
    }
}

impl Handler<WsMessage> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

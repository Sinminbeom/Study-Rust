use actix::{Actor, ActorContext, AsyncContext, StreamHandler, Handler, Message};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, get, web::Payload };
use actix_web_actors::ws;
use std::time::{Duration};
use serde::{Deserialize, Serialize};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(3);
// const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// Define HTTP actor
struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.health_check(ctx);
    }
}

impl MyWs {
    fn handle_message(&self, text: fn()) {
        text();

    }
    fn health_check(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            ctx.text("센서 헬스 체크");
        });
    }
}

fn test() {
    println!("fdfd");
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {

    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let addr = ctx.address();
        let addr_re = addr.recipient();
        addr_re.do_send(WsMessage {
            id: "adfsa".to_string(),
            name: "test".to_string()
        });
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            // Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Text(text)) => MyWs.handle_message(test),
            _ => (),
        }
    }
}
//WsConn responds to this to pipe it through to the actual client
#[derive(Message, Debug, Deserialize, Serialize)]
#[rtype(result = "()")]
pub struct WsMessage {
    pub id: String,
    pub name: String,
}
impl WsMessage {
    fn new(id: String, name: String) -> Self {
        WsMessage { id, name }
    }
}
impl Handler<WsMessage> for MyWs {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.name);
    }
}


#[get("/client")]
pub async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}
#[get("/vpu1")]
pub async fn vpu1(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}
#[get("/vpu2")]
pub async fn vpu2(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}
#[get("/vpu3")]
pub async fn vpu3(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}


// #[get("/ws")]
// pub async fn start_connection(
//     req: HttpRequest,
//     stream: Payload,
//     // Path(group_id): Path<Uuid>,
//     // srv: Data<Addr<Lobby>>,
// ) -> Result<HttpResponse, Error> {
//     // let ws = WsConn::new(
//     //     group_id,
//     //     srv.get_ref().clone(),
//     // );
//
//     // let resp = ws::start(ws, &req, stream)?;
//     // Ok(resp)
// }

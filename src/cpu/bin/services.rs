use actix::{Actor, Addr, StreamHandler};
use actix_web::{Error, get, HttpRequest, HttpResponse};
use actix_web::web::{Data, Path, Payload};
use actix_web_actors::ws;
use uuid::Uuid;
use crate::devices::{VPU1, VPU2, VPU3};
use crate::lobby::Lobby;
use crate::server::WsConn;

#[get("/frontend")]
pub async fn frontend(req: HttpRequest, stream: Payload, srv: Data<Addr<Lobby>>) -> Result<HttpResponse, Error> {
    let ws_conn = WsConn::new("frontend".to_string(), srv.get_ref().clone());
    let resp = ws::start(ws_conn, &req, stream)?;
    println!("{:?}", resp);
    Ok(resp)
}

#[get("/{device_name}")]
pub async fn device_names(
    req: HttpRequest,
    stream: Payload,
    path: Path<String>,
    srv: Data<Addr<Lobby>>,
) -> Result<HttpResponse, Error> {
    let device_name = path.into_inner();
    let ws = WsConn::new(
        device_name,
        srv.get_ref().clone(),
    );

    let resp = ws::start(ws, &req, stream)?;
    Ok(resp)
}

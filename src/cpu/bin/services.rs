use actix::Addr;
use actix_web::{Error, get, HttpRequest, HttpResponse};
use actix_web::web::{Data, Path, Payload};
use actix_web_actors::ws;
use crate::lobby::Lobby;
use crate::server::WsConn;

#[get("/frontend")]
pub async fn frontend(req: HttpRequest, stream: Payload, srv: Data<Addr<Lobby>>,) -> Result<HttpResponse, Error> {
    let ws_conn = WsConn::new("frontend".to_string(), srv.get_ref().clone()); // 필요한 정보를 받는 생성자 함수를 사용하여 새로운 WsConn 인스턴스 생성
    let resp = ws::start(ws_conn, &req, stream);
    println!("{:?}", resp);
    resp
}

#[get("/{device}")]
pub async fn device(
    req: HttpRequest,
    stream: Payload,
    path: Path<String>,
    srv: Data<Addr<Lobby>>,
) -> Result<HttpResponse, Error> {
    let device = path.into_inner(); // Path 구조체의 into_inner() 메서드를 사용하여 필요한 값 추출
    let ws = WsConn::new(
        device,
        srv.get_ref().clone(),
    );

    let resp = ws::start(ws, &req, stream)?;
    Ok(resp)
}

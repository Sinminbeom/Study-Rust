use actix::Addr;
use actix_web::dev::Payload;
use actix_web::{get, HttpRequest};
use actix_web::web::{Data, Path};
use uuid::Uuid;
use crate::lobby::Lobby;
use crate::ws::WsConn;

#[get("/{group_id")]
pub async fn start_connection(
    req: HttpRequest,
    stream: Payload,
    Path(group_id): Path<Uuid>,
    srv: Data<Addr<Lobby>>
) -> Result<HttpResponse, Error> {
    let ws = WsConn::new(group_id, srv.get_ref().clone());

    let resp = ws::start(ws, &req, stream);
    Ok(resp)
}
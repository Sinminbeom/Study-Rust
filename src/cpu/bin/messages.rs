use actix::{Handler, Message, Recipient};
use serde::{Deserialize, Serialize};
use crate::server::WsConn;
use uuid::Uuid;

//WsConn responds to this to pipe it through to the actual client
#[derive(Message, Debug, Deserialize, Serialize)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<WsMessage>,
    pub device: String,
    pub self_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: Uuid,
    pub device: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientActorMessage {
    pub id: Uuid,
    pub msg: String,
    pub device: String
}
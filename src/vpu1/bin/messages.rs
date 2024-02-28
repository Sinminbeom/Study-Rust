use actix::{Handler, Message, Recipient};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

//WsConn responds to this to pipe it through to the actual client
#[derive(Message, Debug, Deserialize, Serialize)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<ClientActorMessage>,
    pub device: String,
    pub self_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: Uuid,
    pub device: String,
}

#[derive(Message, Debug, Deserialize, Serialize)]
#[rtype(result = "()")]
pub struct ClientActorMessage {
    // pub id: Uuid,
    pub device: String,
    pub command: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientRecvMessage {
    pub device: String,
    pub msg: String,
}
use crate::messages::{ClientActorMessage, Connect, Disconnect, WsMessage};
use actix::prelude::{Actor, Context, Handler, Recipient};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use uuid::Uuid;


type Socket = Recipient<ClientActorMessage>;

pub struct Lobby {
    sessions: HashMap<Uuid, Socket>, //self id to self
    // devices: HashMap<String, HashSet<Uuid>>,      //room id  to list of users id
    devices: HashMap<String, Device>      //room id  to list of users id
}
#[derive(Debug)]
pub struct Device {
    id: Uuid,
    sensors: Vec<Sensor>
}

#[derive(Debug)]
pub struct Sensor {
    name: String,
    sensor_type: String,
    logging_status: String,
    sensor_status: String
}

impl Default for Lobby {
    fn default() -> Lobby {
        Lobby {
            sessions: HashMap::new(),
            devices: HashMap::new()
        }
    }
}

impl Lobby {
    // fn send_message(&self, message: &str, id_to: &Uuid) {
    //     if let Some(socket_recipient) = self.sessions.get(id_to) {
    //         let _ = socket_recipient
    //             .do_send(WsMessage(message.to_owned()));
    //     } else {
    //         println!("attempting to send message but couldn't find user id.");
    //     }
    // }
    fn send_message1(&self, message: ClientActorMessage, id_to: &Uuid) {
        if let Some(socket_recipient) = self.sessions.get(id_to) {
            let _ = socket_recipient
                .do_send(message);
        } else {
            println!("attempting to send message but couldn't find user id.");
        }
    }
}

impl Actor for Lobby {
    type Context = Context<Self>;
}

impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        if self.sessions.remove(&msg.id).is_some() {
            // self.devices
            //     .get(&msg.device)
            //     .unwrap()
            //     .iter()
            //     .filter(|conn_id| *conn_id.to_owned() != msg.id)
            //     .for_each(|user_id| self.send_message(&format!("{} disconnected.", &msg.id), user_id));
            if let Some(lobby) = self.devices.get_mut(&msg.device) {
                // if lobby.len() > 1 {
                //     lobby.remove(&msg.id);
                // } else {
                //     //only one in the lobby, remove it entirely
                //     self.devices.remove(&msg.device);
                // }
            }
        }
    }
}




impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        // self.devices
        //     .entry(msg.device.clone())
        //     .or_insert_with(HashSet::new).insert(msg.self_id);



        self.devices
            .entry(msg.device.clone())
            .and_modify(|device| {
                device.id = msg.self_id;
            })
            .or_insert_with(|| Device {
                id: msg.self_id,
                sensors: vec![],
            });
        println!("{:?}", self.devices);



        // self
        //     .devices
        //     .get(&msg.device)
        //     .unwrap()
        //     .iter()
        //     .filter(|conn_id| *conn_id.to_owned() != msg.self_id)
        //     .for_each(|conn_id| self.send_message(&format!("{} just joined!", msg.self_id), conn_id));

        self.sessions.insert(
            msg.self_id,
            msg.addr,
        );
        println!("{:?}", self.sessions);
        // self.send_message(&format!("your id is {}", msg.self_id), &msg.self_id);
    }
}

impl Handler<ClientActorMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: ClientActorMessage, _ctx: &mut Context<Self>) -> Self::Result {
        // if msg.msg.starts_with("\\w") {
        //     if let Some(id_to) = msg.msg.split(' ').collect::<Vec<&str>>().get(1) {
        //         self.send_message(&msg.msg, &Uuid::parse_str(id_to).unwrap());
        //     }
        // } else {
        //     // self.devices.get(&msg.device).unwrap().iter().for_each(|client| self.send_message(&msg.msg, client));
        //     let id = self.devices.get(&msg.device).unwrap().id;
        //     self.send_message("ddd", &id);
        // }
        // self.devices.get(&msg.device).unwrap().iter().for_each(|client| self.send_message(&msg.msg, client));
        let id = self.devices.get(&msg.device).unwrap().id;
        self.send_message1(msg, &id);
    }
}


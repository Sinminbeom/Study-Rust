mod server;
mod services;
mod devices;
mod lobby;
mod messages;

use actix::Actor;
use actix_web::{App, HttpServer};
use crate::lobby::Lobby;
use crate::services::{frontend, device};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = Lobby::default().start(); //create and spin up a lobby

    HttpServer::new(move || {
        let server_ref = server.clone();
        App::new()
            .service(frontend)
            .service(device)
            .app_data(server_ref)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await

}


mod server;
mod services;
mod devices;
mod lobby;
mod messages;

use actix::Actor;
use actix_web::{App, HttpServer, web::Data};
use crate::lobby::Lobby;
use crate::devices::{CPU, VPU1, VPU2, VPU3};
use crate::services::{device_names, frontend};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = Lobby::default().start();

    HttpServer::new(move || {
        App::new()
            .service(frontend)
            .service(device_names)
            .app_data(Data::new(server.clone()))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await

}


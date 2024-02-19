// mod ws;
// mod message;
// mod lobby;
// mod start_connection;

mod server;

use actix::Actor;
use actix_web::{App, HttpServer, web};
// use crate::lobby::Lobby;
use crate::server::index;
use crate::server::vpu1;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // HttpServer::new(|| App::new().route("/ws/", web::get().to(index)))
    //     .bind(("127.0.0.1", 8080))?
    //     .run()
    //     .await

    HttpServer::new(|| App::new().service(index).service(vpu1))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await

}


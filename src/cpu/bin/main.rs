mod server;

use actix::Actor;
use actix_web::{App, HttpServer, web};
use crate::server::{index, vpu1, vpu2, vpu3};
// use crate::lobby::Lobby;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // HttpServer::new(|| App::new().route("/ws/", web::get().to(index)))
    //     .bind(("127.0.0.1", 8080))?
    //     .run()
    //     .await

    HttpServer::new(|| App::new().service(index).service(vpu1).service(vpu2).service(vpu3))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await

}


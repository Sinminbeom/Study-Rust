use actix::{Actor, Context};
use actix_web_actors::{ws::WebsocketContext};

pub struct CPU;

impl Actor for CPU {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // self.health_check(ctx);
        println!("CPU");
    }
}

pub struct VPU1;

impl Actor for VPU1 {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("VPU1");
    }
}

pub struct VPU2;

impl Actor for VPU2 {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("VPU2");
    }
}

pub struct VPU3;

impl Actor for crate::devices::VPU3 {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("VPU3");
    }
}

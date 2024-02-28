use std::collections::HashMap;
use actix::{Actor, Context};
use actix_web_actors::{ws::WebsocketContext};
use crate::lobby::Lobby;

pub struct CPU;

impl Actor for CPU {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // self.health_check(ctx);
        println!("CPU");
    }
}

impl Default for CPU {
    fn default() -> CPU {
        CPU {}
    }
}

pub struct VPU1;

impl Actor for VPU1 {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("VPU1");
    }
}

impl Default for VPU1 {
    fn default() -> VPU1 {
        VPU1 {}
    }
}

pub struct VPU2;

impl Actor for VPU2 {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("VPU2");
    }
}

impl Default for VPU2 {
    fn default() -> VPU2 {
        VPU2 {}
    }
}

pub struct VPU3;

impl Actor for crate::devices::VPU3 {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("VPU3");
    }
}

impl Default for VPU3 {
    fn default() -> VPU3 {
        VPU3 {}
    }
}

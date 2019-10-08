extern crate actix;

use actix::{Actor, Context, System};

pub struct MyActor;

impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
       println!("I am alive!");
       System::current().stop(); // <- stop system
    }
}
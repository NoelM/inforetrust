<<<<<<< Updated upstream
fn main() {
    println!("Hello, world!");
}
=======
extern crate actix;

mod server;

use actix::{Actor, System};

fn main() {
    let system = System::new("test");

    let _addr = server::MyActor.start();

    system.run();
}
>>>>>>> Stashed changes

#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate rocket;
extern crate meiro_server;

use meiro_server::ws_server;

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

fn main() {
    ws_server::start_ws_server();
    rocket::ignite().mount("/", routes![hello]).launch();
}

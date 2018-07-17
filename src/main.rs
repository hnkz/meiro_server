#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate rocket;
extern crate meiro_server;

use meiro_server::ws_server;
use std::thread;

static status: bool = true;

#[get("/status")]
fn get_status() -> String {
    // format!("{}", status.load(Ordering::Relaxed))
    format!("{}", status)
}

fn main() {
    thread::spawn(move || {
        rocket::ignite().mount("/", routes![get_status]).launch();
    });
    ws_server::start_ws_server();
}

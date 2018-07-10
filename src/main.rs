#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate rocket;
extern crate meiro_server;

use meiro_server::ws_server;
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};

// static mut status: AtomicBool = AtomicBool::new(true);
static status: bool = true;

#[get("/status")]
fn get_status() -> String {
    // format!("{}", status.load(Ordering::Relaxed))
    format!("{}", status)
}

fn main() {
    ws_server::start_ws_server();
    
    thread::spawn(move || {
        rocket::ignite().mount("/", routes![get_status]).launch();
    });
}

#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate lazy_static;
extern crate rocket;
extern crate meiro_server;

use meiro_server::ws_server;
use std::thread;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref SERVER_STATUS: Arc<Mutex<bool>> = Arc::new(Mutex::new(true));
}
// static status: bool = true;

#[get("/status")]
fn get_status() -> String {
    format!("{}", SERVER_STATUS.lock().unwrap())
    // format!("{}", status)
}

fn main() {
    thread::spawn(move || {
        rocket::ignite().mount("/", routes![get_status]).launch();
    });
    ws_server::start_ws_server(SERVER_STATUS.clone());
}

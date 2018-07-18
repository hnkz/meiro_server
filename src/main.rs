#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate lazy_static;
extern crate rocket;
extern crate meiro_server;

use meiro_server::ws_server;
use std::thread;
use std::sync::{Arc, Mutex};

// Shared static variable
lazy_static! {
    // Server status
    static ref SERVER_STATUS: Arc<Mutex<bool>> = Arc::new(Mutex::new(true));
}

#[get("/status")]
fn get_status() -> String {
    let status = match SERVER_STATUS.lock() {
        Ok(status) => status,
        Err(err) => {
            println!("status lock error: {}", err);
            return format!("{}", false);
        }
    };

    format!("{}", status)
}

fn main() {
    thread::spawn(move || {
        rocket::ignite().mount("/", routes![get_status]).launch();
    });
        ws_server::start_ws_server(SERVER_STATUS.clone());
}

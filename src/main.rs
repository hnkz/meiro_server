#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate lazy_static;
extern crate rocket;
extern crate meiro_server;
extern crate rocket_contrib;

use rocket_contrib::Template;

use meiro_server::ws_server;
use std::thread;
use std::sync::{Arc, Mutex};
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;


// Shared static variable
lazy_static! {
    // Server status
    static ref SERVER_STATUS: Arc<Mutex<bool>> = Arc::new(Mutex::new(true));
}

// http://localhost:8000/status
#[get("/status")]
fn get_status() -> String {
    let status = match SERVER_STATUS.try_lock() {
        Ok(status) => status,
        Err(err) => {
            println!("status lock error: {}", err);
            return format!("{}", true);
        }
    };

    format!("{}", status)
}

#[get("/<path..>", rank = 5)]
fn all(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("templates").join(path)).ok()
}

fn main() {
    thread::spawn(move || {
        // Web サーバ起動
        rocket::ignite()
            .attach(Template::fairing())
            .mount("/", routes![get_status, all])
            .launch();
    });
        // WebSocket サーバ起動
        ws_server::start_ws_server(SERVER_STATUS.clone());
}

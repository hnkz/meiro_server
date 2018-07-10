#![feature(extern_prelude)]
#[macro_use]
extern crate serde_json;
extern crate websocket;

pub mod ws_server;
mod game;
mod user;
mod map;
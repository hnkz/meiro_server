#![feature(extern_prelude)]
extern crate serde_json;
extern crate websocket;
extern crate rand;

pub mod ws_server;
mod game;
mod user;
mod map;
mod item;
use websocket::sync::Client;
use websocket::OwnedMessage;
use std::net::TcpStream;
use user::User;
use map::Map;
use serde_json::{Value, Error};

pub struct Game {
    max_users: i32,
    map: Map,
    users: Vec<User>,
}

impl Game {
    pub fn new(max_users: i32) -> Game{
        Game {
            max_users: max_users,
            map: Map::new(),
            users: Vec::new()
        }
    }

    pub fn add_user(&mut self, stream: Client<TcpStream>) {
        // set non blocking
        stream.set_nonblocking(true);

        self.users.push(User::new(stream));
        self.check_closed();
    }

    pub fn check_closed(&mut self) {
        let mut i = 0i32;
        for _ in 0..self.users.len() {
            println!("{}", i);
            let message = match self.users[i as usize].get_stream_mut().recv_message() {
                Ok(message) => message,
                Err(err) => {
                    println!("message err: {}", err);
                    i += 1;
                    continue;
                }    
            };

            match message {
                OwnedMessage::Close(_) => {
                    let message = OwnedMessage::Close(None);
                    self.users[i as usize].get_stream_mut().send_message(&message).unwrap();
                    self.users.remove(i as usize);
                    i -= 1;
                }
                _ => {},
            }
            i += 1;
        }
    }

    pub fn start(&mut self) {
        // non blocking & マップ送信
        for i in 0..self.users.len() {
            self.users[i].get_stream_mut().set_nonblocking(false);
            self.users[i].get_stream_mut().send_message(&OwnedMessage::Text(self.map.to_string()));
        }

        // main
        loop {
            for i in 0..self.users.len() {
                let mut raw = match self.users[i].get_stream_mut().recv_message() {
                    Ok(raw) => raw,
                    Err(err) => {
                        println!("message err: {}", err);
                        continue;
                    }
                };

                let message = match raw {
                    OwnedMessage::Text(message) => message,
                    _ => continue
                };

                let v: Value = serde_json::from_str(&message).expect("json parse error");

                for j in 0..self.users.len() {
                    self.users[j].get_stream_mut().send_message(&OwnedMessage::Text(v["player"]["player01"].to_string()));
                }
            }
        }
    }

    pub fn get_user_count(&self) -> usize {
        self.users.len()
    }
}
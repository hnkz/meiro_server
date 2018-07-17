use websocket::sync::Client;
use websocket::OwnedMessage;
use std::net::TcpStream;
use user::User;
use map::Map;

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
        match stream.set_nonblocking(true) {
            Ok(_) => {},
            Err(err) => {
                println!("stream error: {}", err);
            }
        };

        let user_count = self.get_user_count();
        self.users.push(User::new(stream, user_count));
        self.check_closed();
    }

    pub fn set_user_pos(&mut self, i: usize, pos: (f64, f64, f64)) {
        self.users[i as usize].set_pos(pos);
    }

    pub fn remove_item(&mut self, i: usize) {
        self.map.remove_item(i);
    }

    fn set_user_nonblocking(&mut self, flag: bool) {
        for i in 0..self.max_users {
            match self.users[i as usize].get_stream_mut().set_nonblocking(flag) {
                Ok(_) => {},
                Err(err) => {
                    println!("set_nonblocking error : {}", err);
                }   
            };
        }
    }

    pub fn send_json(&mut self, i: usize, map: bool, pos: bool, item: bool) {
        let mut json = "{\n".to_string();
        json.push_str(format!("\"id\": {} \n", i).as_str());

        if map {
            json.push_str(self.map.to_string().as_str());
        }
        if pos {
            json.push_str(",\"player\": [");
            for i in 0..self.max_users {
                json.push_str(self.users[i as usize].to_string().as_str());
                json.push_str(",");
            }
            json.pop();
            json.push_str("]\n");
        }
        if item {
            json.push_str(self.map.item_to_string().as_str());
        }
        json.push_str("}\n");

        match self.users[i].get_stream_mut().send_message(&OwnedMessage::Text(json)) {
            Ok(_) => {},
            Err(err) => {
                println!("ws send_message error: {}", err);
            }   
        };
    }

    fn check_closed(&mut self) {
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

    pub fn get_user(&mut self, i: usize) -> &mut User {
        &mut self.users[i]
    }

    pub fn get_user_count(&self) -> usize {
        self.users.len()
    }
}
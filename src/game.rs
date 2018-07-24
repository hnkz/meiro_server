use std::net::TcpStream;
use websocket::sync::{ Client, Server };
use serde_json::Value;
use user::User;
use map::Map;
use item::{ Item, ItemType };
use rand::prelude::*;
use num::FromPrimitive;

pub struct Game {
    max_users: usize,
    map: Map,
    users: Vec<User>,
    items: Vec<Item>,
}

impl Game {
    // create new Game instance
    pub fn new(max_users: usize) -> Game {
        let mut map = Map::new(33, 33);
        let mut items = Vec::new();
        items.push(Item::new(ItemType::GOAL, map.get_goal_pos()));

        // good unwrap
        for _ in 0..20 {
            items.push(Item::new(ItemType::from_u64(random::<u64>() % 3 + 1).unwrap(), map.get_random_pos()));
        }

        Game {
            max_users: max_users,
            map: map,
            users: Vec::new(),
            items: items
        }
    }

    // start wait state
    pub fn wait(&mut self) -> bool {
        let server = Server::bind("0.0.0.0:2794").unwrap();

        for request in server.filter_map(Result::ok) {
            if !request.protocols().contains(&"rust-websocket".to_string()) {
                request.reject().unwrap();
                return true;
            }

            let mut client = match request.use_protocol("rust-websocket").accept() {
                Ok(client) => client,
                Err(err) => {
                    println!("wait error: {}", err.1);
                    continue;
                }
            };
            let ip = match client.peer_addr() {
                Ok(ip) => ip,
                Err(err) => {
                    println!("peer_addr error: {}", err);
                    continue;
                }   
            };
            println!("add user: {}", ip);

            self.check_closed();
            self.add_user(client);

            if self.users.len() == self.max_users as usize {
                println!("start game");

                return false;
            }
        }

        false
    }

    // start game state
    pub fn start(&mut self, i: usize) {
        if self.users[i].get_closed_flag() {
            return;
        }

        let message = self.users[i].recv_message();

        if message == "".to_string() {
            return;
        }

        let v: Value = serde_json::from_str(&message).expect("json parse error");

        // chat
        match v.get("chat") {
            Some(_chat) => {
                for i in 0..self.users.len() {
                    match self.users[i].send_message(message.clone()) {
                        Ok(_) => {},
                        Err(err) => {
                            println!("user {} is closed ? : {}", i, err);
                            self.users[i].set_closed(true);
                            return;
                        }
                    };
                }
            },
            None => {}
        }

        // get item
        match v.get("get") {
            Some(get) => {
                let id = match get.as_i64() {
                    Some(id) => id as usize,
                    None => {
                        return;
                    }
                };

                let mut json_part: String;

                // if get goal
                if id == 0 {
                    json_part = format!("\"chat\": {{ \"id\": {}, \"content\": \"ゴールしました！\"}}", i);
                } else {
                    json_part = format!("\"get\": {} \n", id);
                }

                for i in 0..self.users.len() {
                    let json = format!("{{\n \"id\": {}\n ,{}}}", i, json_part);
                    match self.users[i].send_message(json) {
                        Ok(_) => {},
                        Err(err) => {
                            println!("user {} is closed ? : {}", i, err);
                            self.users[i].set_closed(true);
                            return;
                        }
                    };
                }
            },
            None => {}
        };

        // pos
        match v.get("pos") {
            Some(pos) => {
                let x = match pos[0].as_f64() {
                    Some(x) => x,
                    None => {
                        return;
                    }
                };
                let y = match pos[1].as_f64() {
                    Some(y) => y,
                    None => {
                        return;
                    }
                };
                let z = match pos[2].as_f64() {
                    Some(z) => z,
                    None => {
                        return;
                    }
                };

                self.set_user_pos(i, (x, y, z));

                let mut json = format!("{{\n \"id\": {}\n", i);
                json.push_str(",\"player\": [");
                for i in 0..self.users.len() {
                    json.push_str(self.users[i].to_string().as_str());
                    json.push_str(",");
                }
                json.pop();
                json.push_str("]\n");
                json.push_str("}\n");
                match self.users[i].send_message(json) {
                    Ok(_) => {},
                    Err(err) => {
                        println!("user {} is closed ? : {}", i, err);
                        self.users[i].set_closed(true);
                        return;
                    }
                };
            },
            None => {}
        }
    }

    // add user to vec
    pub fn add_user(&mut self, stream: Client<TcpStream>) {
        // set non blocking
        match stream.set_nonblocking(true) {
            Ok(_) => {},
            Err(err) => {
                println!("stream error: {}", err);
            }
        };

        let user_count = self.users.len();
        self.users.push(User::new(stream, user_count, self.map.get_start_pos()));

        // create init json
        let mut json = format!("{{\n \"id\": {}\n ,{}", user_count , self.map.to_string());

        json.push_str(",\"player\": [");
        for i in 0..(user_count + 1) {
            json.push_str(self.users[i as usize].to_string().as_str());
            json.push_str(",");
        }
        json.pop();
        json.push_str("]\n");

        json.push_str(",\"item\": [\n");
        for item in &self.items {
            json.push_str(item.to_string().as_str());
            json.push_str(",");
        }
        json.pop();
        json.push_str("]\n");

        json.push_str("}\n");
        match self.users[user_count].send_message(json) {
            Ok(_) => {},
            Err(err) => {
                println!("user {} is closed ? : {}", user_count, err);
            }
        };
    }

    pub fn set_user_pos(&mut self, i: usize, pos: (f64, f64, f64)) {
        self.users[i as usize].set_pos(pos);
    }

    fn check_closed(&mut self) {
        let mut i = 0usize;
        for _ in 0..self.users.len() {
            if i >= self.users.len() {
                return;
            }

            let flag = self.users[i].is_closed();

            if flag {
                println!("remove user: {}", self.users[i].get_name());
                self.users.remove(i);
            }

            i += 1;
        }
    }

    pub fn get_user_len(&self) -> usize {
        self.users.len()
    }
}
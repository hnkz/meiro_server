use std::net::TcpStream;
use websocket::sync::{ Client, Server };
use serde_json::Value;
use user::User;
use map::Map;
use item::Item;

pub struct Game {
    max_users: usize,
    map: Map,
    users: Vec<User>,
    items: Vec<Item>,
}

impl Game {
    // create new Game instance
    pub fn new(max_users: usize) -> Game {
        Game {
            max_users: max_users,
            map: Map::new(33, 33),
            users: Vec::new(),
            items: Item::init_items()
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

            let mut client = request.use_protocol("rust-websocket").accept().unwrap();
            let ip = client.peer_addr().unwrap();
            println!("add user: {}", ip);

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
        let message = self.users[i].recv_message();

        if message == "".to_string() {
            return;
        }

        println!("{}", message);

        let v: Value = serde_json::from_str(&message).expect("json parse error");

        // chat
        match v.get("chat") {
            Some(_chat) => {
                for i in 0..self.max_users {
                    self.users[i].send_message(message.clone());
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

                for i in 0..self.max_users {
                    let json = format!("{{\n \"id\": {}\n ,{}}}", i, json_part);
                    self.users[i].send_message(json);
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
                for i in 0..self.max_users {
                    json.push_str(self.users[i as usize].to_string().as_str());
                    json.push_str(",");
                }
                json.pop();
                json.push_str("]\n");
                json.push_str("}\n");
                self.users[i].send_message(json);
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
        self.users.push(User::new(stream, user_count));

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
        self.users[user_count].send_message(json);

        self.check_closed();
    }

    pub fn set_user_pos(&mut self, i: usize, pos: (f64, f64, f64)) {
        self.users[i as usize].set_pos(pos);
    }

    fn check_closed(&mut self) {
        let mut i = 0i32;
        for _ in 0..self.users.len() {
            println!("{}", i);
            let flag = self.users[i as usize].is_closed();
            if flag {
                self.users.remove(i as usize);
            }

            i += 1;
        }
    }
}
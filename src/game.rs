use std::net::TcpStream;
use websocket::sync::{ Client, Server };
use serde_json::Value;
use user::User;
use map::Map;

pub struct Game {
    max_users: usize,
    map: Map,
    users: Vec<User>,
}

impl Game {
    // create new Game instance
    pub fn new(max_users: usize) -> Game{
        Game {
            max_users: max_users,
            map: Map::new(33, 33),
            users: Vec::new()
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
        let mut item_flag = false;
        let x = match v["pos"][0].as_f64() {
            Some(x) => x,
            None => {
                return;
            }
        };
        let y = match v["pos"][1].as_f64() {
            Some(y) => y,
            None => {
                return;
            }
        };
        let z = match v["pos"][2].as_f64() {
            Some(z) => z,
            None => {
                return;
            }
        };

        self.set_user_pos(i, (x, y, z));

        match v.get("get") {
            Some(get) => {
                item_flag = true;

                let id = match get.as_i64() {
                    Some(id) => id as usize,
                    None => {
                        return;
                    }
                };

                self.remove_item(id);
            },
            None => {

            }
        };
        self.send_json(i, false, true, item_flag);
    }

    // 
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
        self.check_closed();
    }

    pub fn set_user_pos(&mut self, i: usize, pos: (f64, f64, f64)) {
        self.users[i as usize].set_pos(pos);
    }

    // remove i th item
    pub fn remove_item(&mut self, i: usize) {
        self.map.remove_item(i);
    }

    // send json
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

        self.users[i].send_message(json)
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
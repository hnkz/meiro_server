use websocket::sync::Client;
use std::net::TcpStream;
use websocket::OwnedMessage;

pub struct User {
    stream: Client<TcpStream>,
    name: String,
    x: f64,
    y: f64,
    z: f64
}

impl User {
    pub fn new(stream: Client<TcpStream>, i: usize, pos: (f64, f64, f64)) -> User {
        User {
            stream: stream,
            name: format!("player{}", i),
            x: pos.0,
            y: pos.1,
            z: pos.2
        }
    }

    pub fn recv_message(&mut self) -> String {
        let raw = match self.stream.recv_message() {
            Ok(raw) => {
                raw
            },
            Err(_err) => {
                return "".to_string()
            }
        };

        let message = match raw {
            OwnedMessage::Text(message) => message,
            _ => "".to_string()
        };

        message
    }

    pub fn send_message(&mut self, message: String) {
        match self.stream.send_message(&OwnedMessage::Text(message)) {
            Ok(_) => {},
            Err(err) => {
                println!("ws send_message error: {}", err);
            }   
        };
    }

    // set user's position
    pub fn set_pos(&mut self, pos: (f64, f64, f64)) {
        self.x = pos.0;
        self.y = pos.1;
        self.z = pos.2;
    }

    // User's connection is closed ?
    pub fn is_closed(&mut self) -> bool {
        let message = match self.stream.recv_message() {
            Ok(message) => message,
            Err(err) => {
                println!("message err: {}", err);
                return false;
            }    
        };

        match message {
            OwnedMessage::Close(_) => {
                let message = OwnedMessage::Close(None);
                self.stream.send_message(&message).unwrap();
                return true;
            }
            _ => {},
        }

        return false;
    }
}

impl ToString for User {
    #[inline]
    fn to_string(&self) -> String {
        format!("{{\"name\": \"{}\", \"x\": {}, \"y\": {}, \"z\": {} }}", self.name, self.x, self.y, self.z)
    }
}
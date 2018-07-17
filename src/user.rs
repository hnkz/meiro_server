use websocket::sync::Client;
use std::net::TcpStream;

pub struct User {
    stream: Client<TcpStream>,
    name: String,
    x: i32,
    y: i32,
    z: i32
}

impl User {
    pub fn new(stream: Client<TcpStream>, i: usize) -> User {
        User {
            stream: stream,
            name: format!("player{}", i),
            x: 0i32,
            y: 0i32,
            z: 0i32
        }
    }

    pub fn get_stream_mut(&mut self) -> &mut Client<TcpStream> {
        &mut self.stream
    }

    pub fn set_pos(&mut self, pos: (i32, i32, i32)) {
        self.x = pos.0;
        self.y = pos.1;
        self.z = pos.2;
    }
}

impl ToString for User {
    #[inline]
    fn to_string(&self) -> String {
        format!("{{\"name\": \"{}\", \"x\": {}, \"y\": {}, \"z\": {} }}", self.name, self.x, self.y, self.z)
    }
}
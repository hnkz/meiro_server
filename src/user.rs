use websocket::sync::Client;
use std::net::TcpStream;

pub struct User {
    stream: Client<TcpStream>,
    name: String,
    x: f64,
    y: f64,
    z: f64
}

impl User {
    pub fn new(stream: Client<TcpStream>, i: usize) -> User {
        User {
            stream: stream,
            name: format!("player{}", i),
            x: 0f64,
            y: 0f64,
            z: 0f64
        }
    }

    pub fn get_stream_mut(&mut self) -> &mut Client<TcpStream> {
        &mut self.stream
    }
}

impl ToString for User {
    #[inline]
    fn to_string(&self) -> String {
        format!("{{\"name\": \"{}\", \"x\": {}, \"y\": {}, \"z\": {} }}", self.name, self.x, self.y, self.z)
    }
}
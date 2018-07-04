use websocket::sync::Client;
use std::net::TcpStream;

pub struct User {
    stream: Client<TcpStream>,
    x: i32,
    y: i32,
    z: i32
}

impl User {
    pub fn new(stream: Client<TcpStream>) -> User {
        User {
            stream: stream,
            x: 0,
            y: 0,
            z: 0
        }
    }

    pub fn get_stream_mut(&mut self) -> &mut Client<TcpStream> {
        &mut self.stream
    }
}
use websocket::sync::Server;
use game::Game;
use std::sync::atomic::{AtomicBool, Ordering};

const MAX_USER: usize = 1;

pub fn start_ws_server() {
	let mut game = Game::new(MAX_USER as i32);
    let server = Server::bind("127.0.0.1:2794").unwrap();

	// wait state
	for request in server.filter_map(Result::ok) {
		if !request.protocols().contains(&"rust-websocket".to_string()) {
			request.reject().unwrap();
			return;
		}

		let mut client = request.use_protocol("rust-websocket").accept().unwrap();
		let ip = client.peer_addr().unwrap();
		println!("add user: {}", ip);

		game.add_user(client);

		if game.get_user_count() == MAX_USER {
			break;
		}
	}

	// start state
	game.start();
}
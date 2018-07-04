use websocket::sync::Server;
use game::Game;

pub fn start_ws_server() {
	let mut game = Game::new(4);
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

		if game.get_user_count() == 4 {
			break;
		}
	}

	// start state
	game.start();
}
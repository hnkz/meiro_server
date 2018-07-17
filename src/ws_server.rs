use websocket::sync::Server;
use game::Game;
use std::thread;
use std::sync::{Arc, Mutex};
use serde_json::Value;
use websocket::OwnedMessage;

const MAX_USER: usize = 2;

pub fn start_ws_server() {
	let game = Arc::new(Mutex::new(Game::new(MAX_USER as i32)));
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

		game.lock().unwrap().add_user(client);

		if game.lock().unwrap().get_user_count() == MAX_USER {
			println!("start game");
			break;
		}
	}

	// start state
	let mut th = Vec::new();
	for i in 0..MAX_USER {
		game.lock().unwrap().send_json(i, false, true, true);

		let g = game.clone();
		th.push(thread::spawn(move || {
		    // loop of main game
		    loop {
				let raw = match g.lock() {
					Ok(mut g) => {
						match g.get_user(i).get_stream_mut().recv_message() {
							Ok(raw) => {
								raw
							},
							Err(_err) => {
								continue;
							}
						}
					},
					Err(e) => {
						println!("thread lock error: {}", e);
						continue;
					}
				};

				let message = match raw {
                    OwnedMessage::Text(message) => message,
                    _ => continue
                };

                let v: Value = serde_json::from_str(&message).expect("json parse error");
				let mut item_flag = false;
				match g.lock() {
					Ok(mut g) => {
						g.set_user_pos(i, (v["pos"][0].as_i64().unwrap() as i32, v["pos"][1].as_i64().unwrap() as i32, v["pos"][2].as_i64().unwrap() as i32));

						match v.get("get") {
							Some(get) => {
								item_flag = true;
								g.remove_item(get.as_i64().unwrap() as usize);
							},
							None => {

							}
						};
					},
					Err(e) => {
						println!("{}", e);
					}
				};
				match g.lock() {
					Ok(mut g) => {
						g.send_json(i, false, true, item_flag);
					},
					Err(e) => {
						println!("{}", e);
					}
				};
			}
		}));
	}

	for t in th {
		t.join();
	}
}
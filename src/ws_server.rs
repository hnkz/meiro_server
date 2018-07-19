use game::Game;
use std::thread;
use std::sync::{Arc, Mutex};

const MAX_USER: usize = 2;

pub fn start_ws_server(status: Arc<Mutex<bool>>) {
	let game = Arc::new(Mutex::new(Game::new(MAX_USER)));

	// wait state
	let mut status = status.lock().unwrap();
	*status = match game.lock() {
		Ok(mut game) => {
			game.wait()
		},
		Err(err) => {
			println!("game lock error: {}", err);
			false
		}
	};

	// start state
	let mut th = Vec::new();
	for i in 0..MAX_USER {
		let g = game.clone();
		th.push(thread::spawn(move || {
		    // loop of main game
			loop {
				match g.lock() {
					Ok(mut g) => {
						g.start(i);
					},
					Err(err) => {
						println!("g lock error: {}", err);
					}
				}
			}
		}));
	}

	for t in th {
		match t.join() {
			Ok(_) => {},
			Err(err) => {
				println!("thread join error: {:?}", err);
			}
		};
	}
}
mod reizen;
mod cards;
mod game;

use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::accept;

fn main () {
    let server = TcpListener::bind("0.0.0.0:39153").unwrap();
    for stream in server.incoming() {
        spawn (move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read().unwrap();

                // We do not want to send back ping/pong messages.
                if msg.is_binary() || msg.is_text() {
                    websocket.send(msg).unwrap();
                }
            }
        });
    }
}


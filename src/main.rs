mod manet;

use std::net::TcpListener;
use tungstenite::accept;

fn main() {
    let server = TcpListener::bind("127.0.0.1:9001").expect("Failed to bind to address");

    println!("WebSocket server listening on ws://127.0.0.1:9001");

    for stream in server.incoming() {
        match stream {
            Ok(stream) => {
                // Accept the WebSocket connection.
                let mut websocket = accept(stream).expect("Error accepting connection");

                println!("New WebSocket connection");

                // Handle each incoming message.
                loop {
                    let msg = websocket.read_message().expect("Error reading message");

                    if msg.is_text() || msg.is_binary() {
                        println!("Received message: {}", msg);
                        // Echo the message back to the client.
                        websocket.write_message(msg).expect("Error writing message");
                    }
                }
            }
            Err(e) => {
                eprintln!("Connection error: {}", e);
            }
        }
    }
}


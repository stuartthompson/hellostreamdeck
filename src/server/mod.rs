use std::io::Read;
use std::net::TcpListener;
use std::sync::mpsc::{Receiver, TryRecvError};
use std::thread;
use std::time::Duration;

use websocket::sync::Server;
use websocket::OwnedMessage;

const IP: &str = "127.0.0.1";
const MSG_SIZE: usize = 512;

pub fn start_server(
    port: String,
    plugin_uuid: String,
    register_event: String,
    info: String,
    rx: Receiver<bool>
) {
    // Build address
    let addr = format!("{0}:{1}", IP, port);

    // Create server
    let server = Server::bind(addr).unwrap();

    for request in server.filter_map(Result::ok) {
        // Spawn a new thread for each connection
        thread::spawn(|| {
            if !request.protocols().contains(&"rust-websocket".to_string()) {
                request.reject().unwrap();
                return;
            }

            let mut client = request.use_protocol("rust-websocket").accept().unwrap();

            let ip = client.peer_addr().unwrap();

            println!("Connection from {}", ip);

            let message = OwnedMessage::Text("Hello".to_string());
            client.send_message(&message).unwrap();

            let (mut receiver, mut sender) = client.split().unwrap();

            for message in receiver.incoming_messages() {
                let message = message.unwrap();

                match message {
                    OwnedMessage::Close(_) => {
                        let message = OwnedMessage::Close(None);
                        sender.send_message(&message).unwrap();
                        println!("Client {} disconnected.", ip);
                        return;
                    }
                    OwnedMessage::Ping(ping) => {
                        let message = OwnedMessage::Pong(ping);
                        sender.send_message(&message).unwrap();
                        println!("Client {} ping.", ip);
                    }
                    _ => sender.send_message(&message).unwrap(),
                }
            }
        });
    }
}

pub fn start_server_old(
    port: String,
    plugin_uuid: String,
    register_event: String,
    info: String,
    rx: Receiver<bool>,
) {
    println!("Port: {0}", port);
    println!("PluginUUID: {0}", plugin_uuid);
    println!("RegisterEvent: {0}", register_event);
    println!("Info: {0}", info);

    let addr = format!("{0}:{1}", IP, port);

    let server = TcpListener::bind(addr).expect("Listener failed to bind.");

    server
        .set_nonblocking(true)
        .expect("Failed to initialize server in non-blocking mode.");

    let mut server_running: bool = true;

    while server_running {
        // Check for termination signal
        match rx.try_recv() {
            Ok(_) | Err(TryRecvError::Disconnected) => {
                println!("[Server] Terminating server.");
                server_running = false;
            }
            Err(TryRecvError::Empty) => {}
        }

        // Check for client connection
        if let Ok((mut socket, addr)) = server.accept() {
            println!("[Server] Client {} connected", addr);

            let mut buff = vec![0; MSG_SIZE];

            // Read from socket
            match socket.read(&mut buff) {
                Ok(_) => {
                    let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                    let msg = String::from_utf8(msg).expect("Invalid uft8 message.");

                    println!("[Server] Client {} says: {:?}", addr, msg);
                }
                Err(e) => {
                    println!("[Server] Error reading message from client: {:?}", e);
                }
            }
        }

        // Sleep for one second
        thread::sleep(Duration::from_millis(100));
    }

    println!("[Server] Stopped.");
}

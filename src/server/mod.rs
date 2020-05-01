use std::io::Read;
use std::net::TcpListener;
use std::sync::mpsc::{Receiver, TryRecvError};
use std::thread;
use std::time::Duration;

const IP: &str = "127.0.0.1";
const MSG_SIZE: usize = 512;

pub fn start_server(
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

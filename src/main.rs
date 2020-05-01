extern crate websocket;

mod server;

use std::env;
use std::sync::mpsc;



fn main() {
    let args: Vec<String> = env::args().collect();

    let mut port: String = String::from("");
    let mut plugin_uuid: String = String::from("");
    let mut register_event: String = String::from("");
    let mut info: String = String::from("");

    for ix in 0..args.len() {
        if args[ix] == "-port" {
            port = args[ix+1].clone();
        }
        if args[ix] == "-pluginUUID" {
            plugin_uuid = args[ix+1].clone();
        }
        if args[ix] == "-registerEvent" {
            register_event = args[ix+1].clone();
        }
        if args[ix] == "-info" {
            info = args[ix+1].clone();
        }
    }

    let (_tx, rx) = mpsc::channel();
    server::start_server(port, plugin_uuid, register_event, info, rx);
}
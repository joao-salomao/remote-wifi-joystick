mod servers;

use enigo::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use std::str::from_utf8;
use std::thread;
use std::time::Duration;

use crate::servers::BroadcastServer;

const SERVICE_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
const SERVICE_PORT: u16 = 1080;

const BROADCAST_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(255, 255, 255, 255));
const BROADCAST_PORT: u16 = 9001;
const BROADCAST_MESSAGE_INTERVAL: Duration = Duration::from_secs(5);
const BROADCAST_READ_TIMEOUT: Duration = Duration::from_secs(5);

fn main() {
    let service_socket_addr = SocketAddr::new(SERVICE_IP, SERVICE_PORT);

    let mut handles = Vec::with_capacity(2);

    handles.push(thread::spawn(move || {
        BroadcastServer::new(
            BROADCAST_IP,
            BROADCAST_PORT,
            BROADCAST_MESSAGE_INTERVAL,
            BROADCAST_READ_TIMEOUT,
            service_socket_addr,
        )
        .run();
    }));

    thread::spawn(move || {
        key_press_server(service_socket_addr);
    });

    for handle in handles {
        handle.join().expect("couldn't join thread");
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct KeyPressMessage {
    key: char,
}

fn key_press_server(service_socket_addr: SocketAddr) {
    let mut enigo = Enigo::new();

    let socket = UdpSocket::bind(service_socket_addr).expect("couldn't bind to address");
    loop {
        let mut buffer = [0; 50];
        let socket_clone = socket.try_clone().expect("couldn't clone socket");

        let filled_buffer = match socket_clone.recv_from(&mut buffer) {
            Ok((number_of_bytes, source_address)) => {
                println!("Recebido {} bytes de {}", number_of_bytes, source_address);
                &mut buffer[..number_of_bytes]
            }
            Err(e) => {
                println!("Algo deu errado: {}", e);
                continue;
            }
        };

        let message: KeyPressMessage = match serde_json::from_slice(&filled_buffer) {
            Ok(v) => v,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        dbg!(&message);
        // enigo.key_click(Key::Layout(message.key));
    }
}

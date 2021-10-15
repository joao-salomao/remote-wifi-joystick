use enigo::{Enigo, Key};
use serde_json::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use std::str::from_utf8;
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;
use serde::{Deserialize, Serialize};


const SERVER_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
const SERVER_PORT: u16 = 1080;

fn main() {
    let service_socket_addr = SocketAddr::new(SERVER_IP, SERVER_PORT);

    let mut handles = Vec::with_capacity(2);

    handles.push(thread::spawn(move || {
        broadcast_server(service_socket_addr);
    }));

    println!("aquiiii");

    thread::spawn(move || {
        key_press_server(service_socket_addr);
    });

    for handle in handles {
        handle.join().unwrap();
    }
}

fn key_press_server(service_socket_addr: SocketAddr) {
    let socket = UdpSocket::bind(service_socket_addr).expect("couldn't bind to address");
    loop {
        let mut buf = [0 as u8; 32];

        let socket_clone = socket.try_clone().unwrap();
        let (number_of_bytes, src_addr) = socket_clone
            .recv_from(&mut buf)
            .expect("Didn't receive data");

        let msg = from_utf8(&buf).unwrap();
        println!(
            "Msg: {} | Bytes: {} | addr: {}",
            msg, number_of_bytes, src_addr
        );

        // socket_clone
        //     .send_to(b"alou", &src_addr)
        //     .expect("error sending");
    }
}

// https://stackoverflow.com/questions/61045602/how-do-you-broadcast-a-udp-datagram-and-receive-the-responses-in-rust
// https://github.com/andrewdavidmackenzie/simpdiscover
const BROADCAST_ADDRESS: &str = "255.255.255.255:9001";


#[derive(Serialize, Deserialize, Debug)]
struct BroadcastMessage {
    ip: String,
    port: u16,
}

fn broadcast_server(server_addr: SocketAddr) {
    let socket = UdpSocket::bind("127.0.0.1:0").expect("couldn't bind to address");

    let timeout_duration = Duration::from_secs(5);

    socket
        .set_read_timeout(Some(timeout_duration))
        .expect("couldn't set read timeout");

    socket
        .set_broadcast(true)
        .expect("couldn't set broadcast to true");

    let message = BroadcastMessage {
        ip: server_addr.ip().to_string(),
        port: server_addr.port(),
    };

    let data = json!(message).to_string();
    println!("message: {}", data);

    let bytes = data.as_bytes();
    let bytes_len = bytes.len();
    println!("bytes_len: {}", bytes_len);

    loop {
        match socket.send_to(bytes, BROADCAST_ADDRESS) {
            Ok(n) => {
                if n != bytes_len {
                    println!("Sent the wrong number of bytes");
                } else {
                    println!("Broadcast sent");
                }
            }
            Err(e) => println!("Sent the wrong: {}", e),
        }

        thread::sleep(timeout_duration);
    }

    println!("broadcast_server {}", server_addr);
}

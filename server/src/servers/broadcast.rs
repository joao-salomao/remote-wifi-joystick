// https://stackoverflow.com/questions/61045602/how-do-you-broadcast-a-udp-datagram-and-receive-the-responses-in-rust
// https://github.com/andrewdavidmackenzie/simpdiscover

use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::thread;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
struct BroadcastMessage {
    ip: String,
    port: u16,
}

pub struct BroadcastServer {
    broadcast_socket_addr: SocketAddr,
    read_timeout: Duration,
    message_interval: Duration,
    key_press_server_ip: IpAddr,
    key_press_server_port: u16,
}

impl BroadcastServer {
    pub fn new(
        ip: IpAddr,
        port: u16,
        message_interval: Duration,
        read_timeout: Duration,
        key_press_server_ip: IpAddr,
        key_press_server_port: u16,
    ) -> Self {
        Self {
            read_timeout,
            message_interval,
            key_press_server_ip,
            key_press_server_port,
            broadcast_socket_addr: SocketAddr::new(ip, port),
        }
    }

    pub fn run(&self) {
        let socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");

        socket
            .set_read_timeout(Some(self.read_timeout))
            .expect("couldn't set read timeout");

        socket
            .set_broadcast(true)
            .expect("couldn't set broadcast to true");

        let message = BroadcastMessage {
            ip: self.key_press_server_ip.to_string(),
            port: self.key_press_server_port,
        };

        let data = json!(message).to_string();
        println!("message: {}", data);

        let bytes = data.as_bytes();
        let bytes_len = bytes.len();
        println!("bytes_len: {}", bytes_len);

        loop {
            match socket.send_to(bytes, self.broadcast_socket_addr) {
                Ok(n) => {
                    if n != bytes_len {
                        println!("Sent the wrong number of bytes");
                    } else {
                        println!("Broadcast sent");
                    }
                }
                Err(e) => println!("Sent the wrong: {}", e),
            }
            thread::sleep(self.message_interval);
        }
    }
}

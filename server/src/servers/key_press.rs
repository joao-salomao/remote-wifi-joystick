use enigo::*;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, SocketAddr, UdpSocket};

#[derive(Serialize, Deserialize, Debug)]
struct ReceivedMessage {
    key: char,
}

pub struct KeyPressServer {
    socket_addr: SocketAddr,
}

impl KeyPressServer {
    pub fn new(ip: IpAddr, port: u16) -> Self {
        Self {
            socket_addr: SocketAddr::new(ip, port),
        }
    }

    pub fn run(&self) {
        let mut enigo = Enigo::new();

        let socket = UdpSocket::bind(self.socket_addr).expect("couldn't bind to address");
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
            let message: ReceivedMessage = match serde_json::from_slice(&filled_buffer) {
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
}

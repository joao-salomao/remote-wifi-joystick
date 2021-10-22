use rand::prelude::*;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use serde_json;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use std::str::from_utf8;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
struct KeyPressMessage {
    key: char,
}

fn main() {
    let server_socket_addr = find_server();
    run_key_press_sender(server_socket_addr);
}

fn run_key_press_sender(server_socket_addr: SocketAddr) {
    let mut rng = thread_rng();
    let socket = UdpSocket::bind("127.0.0.1:0").expect("couldn't bind to address");

    let mut acc = 0;

    loop {
        let options = "qwertyuiopasdfghjklçzxcvbnm".chars();
        let key = options.choose(&mut rng).expect("couldn't get a random key");
        let message = KeyPressMessage { key };
        let serialized = serde_json::to_string(&message).unwrap();
        let bytes = serialized.as_bytes();

        match socket.send_to(bytes, server_socket_addr) {
            Ok(_) => {
                acc += 1;
                // thread::sleep(Duration::from_secs(1));
            }
            Err(e) => println!("Erro: {}", e),
        }

        println!("Mensagens enviadas: {}", acc);
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ServerInfo {
    ip: IpAddr,
    port: u16,
}

fn find_server() -> SocketAddr {
    let socket = UdpSocket::bind("0.0.0.0:9001").unwrap();

    loop {
        let mut buffer = [0; 50];

        let filled_buffer = match socket.recv_from(&mut buffer) {
            Ok((number_of_bytes, source_address)) => {
                println!("Recebido {} bytes de {}", number_of_bytes, source_address);
                &mut buffer[..number_of_bytes]
            }
            Err(e) => {
                println!("Algo deu errado: {}", e);
                continue;
            }
        };

        dbg!(from_utf8(&filled_buffer).unwrap());

        let server_info: ServerInfo = match serde_json::from_slice(&filled_buffer) {
            Ok(v) => v,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        let socket_addr = SocketAddr::new(server_info.ip, server_info.port);

        println!("Serviço encontrado: {}", socket_addr);

        return socket_addr;
    }
}

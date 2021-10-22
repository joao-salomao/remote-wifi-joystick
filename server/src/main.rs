mod servers;

use crate::servers::BroadcastServer;
use crate::servers::KeyPressServer;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::thread;
use std::time::Duration;

// KeyPressServer constants
const KEY_PRESS_SERVER_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
const KEY_PRESS_SERVER_PORT: u16 = 1080;

// BroadcastServer constants
const BROADCAST_SERVER_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(255, 255, 255, 255));
const BROADCAST_SERVER_PORT: u16 = 9001;
const BROADCAST_SERVER_MESSAGE_INTERVAL: Duration = Duration::from_secs(2);
const BROADCAST_SERVER_READ_TIMEOUT: Duration = Duration::from_secs(5);

fn main() {
    let service_socket_addr = SocketAddr::new(KEY_PRESS_SERVER_IP, KEY_PRESS_SERVER_PORT);

    let mut handles = Vec::with_capacity(2);

    handles.push(thread::spawn(move || {
        BroadcastServer::new(
            BROADCAST_SERVER_IP,
            BROADCAST_SERVER_PORT,
            BROADCAST_SERVER_MESSAGE_INTERVAL,
            BROADCAST_SERVER_READ_TIMEOUT,
            service_socket_addr,
        )
        .run();
    }));

    thread::spawn(move || {
        KeyPressServer::new(KEY_PRESS_SERVER_IP, KEY_PRESS_SERVER_PORT).run();
    });

    for handle in handles {
        handle.join().expect("couldn't join thread");
    }
}

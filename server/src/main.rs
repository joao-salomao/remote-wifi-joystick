mod servers;

use crate::servers::BroadcastServer;
use crate::servers::KeyPressServer;

use std::net::{IpAddr, Ipv4Addr};
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
    let key_press_server = KeyPressServer::new(KEY_PRESS_SERVER_IP, KEY_PRESS_SERVER_PORT);
    let broadcast_server = BroadcastServer::new(
        BROADCAST_SERVER_IP,
        BROADCAST_SERVER_PORT,
        BROADCAST_SERVER_MESSAGE_INTERVAL,
        BROADCAST_SERVER_READ_TIMEOUT,
        KEY_PRESS_SERVER_IP,
        KEY_PRESS_SERVER_PORT,
    );

    let broadcast_handler = thread::spawn(move || {
        broadcast_server.run();
    });

    let key_press_handler = thread::spawn(move || {
        key_press_server.run();
    });

    broadcast_handler
        .join()
        .expect("The broadcast thread has panicked");

    key_press_handler
        .join()
        .expect("The key press thread has panicked");
}

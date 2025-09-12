// No processing middlewares, just a barebones server example with packet parsing

use log::{debug, info};
use rusty_enet as enet;
use std::time::Duration;

use godot_enet as gd_enet;

#[tokio::main]
async fn main() {
    let mut clog = colog::default_builder();
    clog.filter(None, log::LevelFilter::Trace);
    clog.init();

    let mut server = gd_enet::Server::builder().build().unwrap();

    server.open().unwrap();

    let host = server.get_mut_host().unwrap();

    loop {
        while let Some(event) = host.service().unwrap() {
            match event {
                enet::Event::Connect { peer, data } => {
                    info!("Peer {:?} connected with {:?}", peer.id().0, data);
                }
                enet::Event::Disconnect { peer, data } => {
                    info!("Peer {:?} disconnected with {:?}", peer.id().0, data);
                }
                enet::Event::Receive {
                    peer,
                    channel_id,
                    packet,
                } => {
                    let parsed_packet = gd_enet::packet::parse_packet(packet.data()).unwrap();
                    if let Ok(message) = str::from_utf8(packet.data()) {
                        debug!(
                            "Received packet: {:?}\nFrom: {:?}\nOn: {:?}",
                            message,
                            peer.id().0,
                            channel_id
                        );
                        debug!("Parsed: {:?}", parsed_packet);
                    }
                }
            }
        }

        std::thread::sleep(Duration::from_millis(10));
    }
}

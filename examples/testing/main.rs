use log::{debug, info};
use rusty_enet as enet;
use std::{
    net::{SocketAddr, UdpSocket},
    str::{self, FromStr},
    time::Duration,
};

use godot_enet as godot;

fn main() {
    let mut clog = colog::default_builder();
    clog.filter(None, log::LevelFilter::Trace);
    clog.init();

    let socket = UdpSocket::bind(SocketAddr::from_str("127.0.0.1:55556").unwrap()).unwrap();
    let mut host = enet::Host::new(
        socket,
        enet::HostSettings {
            peer_limit: 32,
            channel_limit: 2,
            compressor: None,
            checksum: None,
            ..Default::default()
        },
    )
    .unwrap();

    loop {
        while let Some(event) = host.service().unwrap() {
            match event {
                enet::Event::Connect { peer, data } => {
                    info!("Peer {} connected with {}", peer.id().0, data);
                }
                enet::Event::Disconnect { peer, data } => {
                    info!("Peer {} disconnected with {}", peer.id().0, data);
                }
                enet::Event::Receive {
                    peer,
                    channel_id,
                    packet,
                } => {
                    let parsed_packet = godot::packet::parse_packet(packet.data()).unwrap();
                    if let Ok(message) = str::from_utf8(packet.data()) {
                        debug!(
                            "Received packet: {:?}\nFrom: {:?}\nOn: {:?}",
                            message,
                            peer.id().0,
                            channel_id
                        );
                        debug!("Parsed Header: {:?}", parsed_packet);
                    }

                    if let godot::packet::GodotENetPacket::NetworkCommandSys(sys_packet) =
                        parsed_packet
                    {
                        if let godot::packet::sys::SysCommand::SysCommandRelay { content } =
                            sys_packet.sys_cmd
                        {
                            debug!(
                                "Sending packet: {:?}\nTo: {:?}\nOn: {:?}\n",
                                content,
                                peer.id().0,
                                channel_id
                            );
                            debug!("Parsed Header: {:?}", godot::packet::parse_packet(&content));
                            _ = peer.send(channel_id, &enet::Packet::reliable(content));
                        }
                    }
                }
            }
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

use godot_enet::{
    self as gd_enet, AsyncLayer, LayerResult,
    packet::{Packet, outgoing},
};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut clog = colog::default_builder();
    clog.filter(None, log::LevelFilter::Trace);
    clog.init();

    let mut builder = gd_enet::Server::builder();

    builder = builder
        .layer(gd_enet::layers::AutoParseLayer)
        .layer(AsyncLayer::build(testing));

    let mut server = builder.build().unwrap();

    server.open().unwrap();

    loop {
        server.service().await.unwrap();

        std::thread::sleep(Duration::from_millis(10));
    }
}

async fn testing(event: gd_enet::event::Event) -> LayerResult {
    if let Some(parsed_packet) = event.data_pile.get::<Packet>() {
        if let Packet::NetworkCommandSys(packet) = parsed_packet {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;

            if let gd_enet::packet::sys::SysCommand::SysCommandRelay { content } = &packet.sys_cmd {
                let outgoing_packet = outgoing::OutgoingPacket {
                    peer_id: outgoing::PeerID(0),
                    channel_id: 0,
                    packet: outgoing::Packet::reliable((*content).clone()),
                };

                if let Err(e) = event.tx_outgoing.send(outgoing_packet) {
                    return Err(format!("Failed to transmit outgoing packet: {:?}", e));
                }

                return Ok(Some(event));
            }

            return Ok(None);
        }
    }

    return Ok(Some(event));
}

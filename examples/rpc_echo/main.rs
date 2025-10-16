use godot_enet::{
    self as gd_enet, AsyncLayer, ENetPeerID, GDPeerID, LayerResult, fn_layer_err, name_id,
    packet::{Packet, outgoing, rpc::smart_send_packet},
    sort_names,
};
use std::{sync::Arc, time::Duration, vec};

const NAMES: [&str; 2] = sort_names!["rpc_testing", "abc"];

#[tokio::main]
async fn main() {
    let mut clog = colog::default_builder();
    clog.filter(None, log::LevelFilter::Trace);
    clog.init();

    let mut builder = gd_enet::Server::builder();

    let mut path_cache_layer = gd_enet::layers::PathCacheLayer::default();
    path_cache_layer.consume_simplify_path = false;

    let router = gd_enet::routers::RPCPathRouter::new();
    let function_router = gd_enet::routers::RPCFunctionNameRouter::new();
    function_router.register_name_id(
        name_id!("rpc_testing", NAMES),
        Arc::new(AsyncLayer::build(echo)),
    );
    function_router.register_name_id(
        name_id!("abc", NAMES),
        Arc::new(AsyncLayer::build(send_abc)),
    );
    router.register_path("NetworkButtons".to_string(), Arc::new(function_router));

    builder = builder
        .layer(gd_enet::layers::AutoParseLayer)
        .layer(gd_enet::layers::PeerMapLayer::default())
        .layer(path_cache_layer)
        .layer(gd_enet::layers::RPCParseLayer)
        .layer(router);

    let mut server = builder.build().unwrap();

    server.open().unwrap();

    loop {
        server.service().await.unwrap();

        std::thread::sleep(Duration::from_millis(10));
    }
}

async fn echo(event: gd_enet::event::Event) -> LayerResult {
    if let Some(parsed_packet) = event.data_pile.get::<Packet>() {
        if let Packet::NetworkCommandSimplifyPath { .. } = parsed_packet {
            let godot_enet::event::EventType::Receive { raw_packet, .. } = &event.event else {
                return Err(fn_layer_err!(
                    "Echo",
                    "Expected Receive event type".to_string()
                ));
            };

            let outgoing_packet = outgoing::OutgoingPacket {
                peer_id: ENetPeerID(0),
                channel_id: 0,
                packet: outgoing::Packet::reliable(raw_packet.data()),
            };

            if let Err(e) = event.tx_outgoing.send(outgoing_packet) {
                return Err(fn_layer_err!(
                    "Echo",
                    "Failed to transmit outgoing packet: {:?}",
                    e
                ));
            }

            return Ok(None);
        } else if let Packet::NetworkCommandConfirmPath {
            remote_cache_id, ..
        } = parsed_packet
        {
            log::info!(
                "Received Confirm Path for Remote Cache ID: {}",
                remote_cache_id
            );

            return Ok(None);
        } else if let Some(rpc_command) = event.data_pile.get::<gd_enet::packet::rpc::RPCCommand>()
        {
            log::info!("Received RPC Command For Node: {:?}", rpc_command.path);
            for (i, arg) in rpc_command.args.iter().enumerate() {
                log::info!("Arg {}: {:?}", i, arg);
            }

            let godot_enet::event::EventType::Receive { raw_packet, .. } = &event.event else {
                return Err(fn_layer_err!("Echo", "Expected Receive event type",));
            };

            let outgoing_packet = outgoing::OutgoingPacket {
                peer_id: ENetPeerID(0),
                channel_id: 0,
                packet: outgoing::Packet::reliable(raw_packet.data()),
            };

            if let Err(e) = event.tx_outgoing.send(outgoing_packet) {
                return Err(fn_layer_err!(
                    "Echo",
                    "Failed to transmit outgoing packet: {:?}",
                    e
                ));
            }

            log::info!(
                "Predicted Hash {:?}",
                gd_enet::routers::hash_function_set(&[
                    "rpc_testing".to_string(),
                    "abc".to_string()
                ])
            );

            const NAMES: [&str; 2] = sort_names!["rpc_testing", "abc"];

            log::info!(
                "Predicted ID for 'rpc_testing' {:?}",
                name_id!("rpc_testing", NAMES)
            );
        }
    }

    return Ok(Some(event));
}

async fn send_abc(event: gd_enet::event::Event) -> LayerResult {
    let Some(enet_peer_id) = event.data_pile.get::<gd_enet::ENetPeerID>() else {
        return Err(fn_layer_err!(
            "SendABC",
            "send_abc called without ENetPeerID in DataPile, requires PeerMapLayer".to_string()
        ));
    };

    let Some(gd_peer_id) = event.data_pile.get::<GDPeerID>() else {
        return Err(fn_layer_err!(
            "SendABC",
            "send_abc called without Godot Peer ID in DataPile, requires PeerMapLayer".to_string()
        ));
    };

    let Some(outgoing_cache) = event.data_pile.get::<gd_enet::layers::OutgoingCache>() else {
        return Err(fn_layer_err!(
            "SendABC",
            "send_abc called without OutgoingCache in DataPile, requires PathCacheLayer"
                .to_string()
        ));
    };

    let path = "NetworkButtons".to_string();

    log::info!(
        "Sending 'abc' RPC to Node at Path: {} for Peer ID: {:?}",
        path,
        enet_peer_id
    );

    let args = vec![];

    if let Err(e) = smart_send_packet(
        path,
        gd_enet::routers::hash_function_set(&["rpc_testing".to_string(), "abc".to_string()]),
        args,
        name_id!("abc", NAMES),
        outgoing_cache,
        &event.tx_outgoing,
        gd_peer_id,
        enet_peer_id,
    ) {
        return Err(fn_layer_err!(
            "SendABC",
            "Failed to transmit outgoing packet: {:?}",
            e
        ));
    }

    return Ok(Some(event));
}

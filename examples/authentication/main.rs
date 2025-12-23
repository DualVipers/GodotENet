use godot_enet::{self as gd_enet, DataPile, ENetPeerID};
use log::info;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut clog = colog::default_builder();
    clog.filter(None, log::LevelFilter::Trace);
    clog.init();

    let mut builder = gd_enet::Server::builder();

    builder = builder
        .layer(gd_enet::layers::AutoParseLayer)
        .layer(gd_enet::layers::AuthenticationLayer::build(
            auto_authenticate,
        ))
        .layer(gd_enet::layers::PeerMapLayer::default());

    let mut server = builder.build().unwrap();

    server.open().unwrap();

    loop {
        server.service().await.unwrap();

        std::thread::sleep(Duration::from_millis(10));
    }
}

async fn auto_authenticate(peer: ENetPeerID, content: Box<[u8]>, _data_pile: DataPile) -> bool {
    info!("Authenticating peer {} with content: {:?}", peer.0, content);

    true
}

use std::time::Duration;

use godot_enet as gd_enet;

#[tokio::main]
async fn main() {
    let mut clog = colog::default_builder();
    clog.filter(None, log::LevelFilter::Trace);
    clog.init();

    let mut builder = gd_enet::GodotENetServer::builder();

    builder = builder
        .layer(gd_enet::AutoParseLayer)
        .layer(gd_enet::testing::TestingLayer);

    let mut server = builder.build().unwrap();

    server.open().unwrap();

    loop {
        server.service().await.unwrap();

        std::thread::sleep(Duration::from_millis(10));
    }
}

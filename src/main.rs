pub mod config;
pub mod core;
pub mod auto_reload;
pub mod navigation;
pub mod server;

use core::dorcs::Dorcs;

#[tokio::main]
async fn main() {
    let mut dorcs = Dorcs::new();
    dorcs.init();

    if dorcs.config.server.enabled {
        dorcs.serve().await;
    }
}

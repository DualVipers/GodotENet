mod data_pile;
pub mod event;
mod layer;
pub mod layers;
pub mod packet;
mod server;

pub use data_pile::*;
pub use layer::*;
pub use server::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// The peer id within Godot
///
/// -1 for all but server, <=0 for all & server, >=1 for single client or server
pub struct GDPeerID(pub i32);

impl From<u32> for GDPeerID {
    fn from(value: u32) -> Self {
        // There is some weird s*** going on with SceneMultiplayer::_process_sys, but this seems to work
        GDPeerID(value as i32)
    }
}

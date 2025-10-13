mod data_pile;
mod dyn_helpers;
pub mod event;
mod layer;
pub mod layers;
pub mod packet;
pub mod routers;
mod server;
pub mod utils;
pub mod variant;

pub use data_pile::*;
pub use dyn_helpers::*;
pub use layer::*;
pub use server::*;

use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// The peer id within Godot
///
/// -x for all but x,
/// 0 for all,
/// 1 for server,
/// 2+ for single peer
pub struct GDPeerID(pub i32);

impl From<u32> for GDPeerID {
    fn from(value: u32) -> Self {
        // There is some weird s*** going on with SceneMultiplayer::_process_sys, but this seems to work
        GDPeerID(value as i32)
    }
}

impl Into<i32> for GDPeerID {
    fn into(self) -> i32 {
        self.0
    }
}

impl Deref for GDPeerID {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// The peer id within ENet
pub struct ENetPeerID(pub usize);

impl From<rusty_enet::PeerID> for ENetPeerID {
    fn from(value: rusty_enet::PeerID) -> Self {
        ENetPeerID(value.0)
    }
}

impl From<usize> for ENetPeerID {
    fn from(value: usize) -> Self {
        ENetPeerID(value)
    }
}

impl Into<rusty_enet::PeerID> for ENetPeerID {
    fn into(self) -> rusty_enet::PeerID {
        rusty_enet::PeerID(self.0)
    }
}

impl Into<usize> for ENetPeerID {
    fn into(self) -> usize {
        self.0
    }
}

impl Deref for ENetPeerID {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

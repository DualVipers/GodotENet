use std::{
    hash::Hash,
    ops::{Deref, DerefMut},
};

/// Unsafe if the slice is less than 4 bytes
pub fn parse_f32(raw_bytes: &[u8]) -> f32 {
    f32::from_le_bytes([raw_bytes[0], raw_bytes[1], raw_bytes[2], raw_bytes[3]])
}

/// Unsafe if the slice is less than 8 bytes
pub fn parse_f64(raw_bytes: &[u8]) -> f64 {
    f64::from_le_bytes([
        raw_bytes[0],
        raw_bytes[1],
        raw_bytes[2],
        raw_bytes[3],
        raw_bytes[4],
        raw_bytes[5],
        raw_bytes[6],
        raw_bytes[7],
    ])
}

/// Unsafe if the slice is less than 4 bytes
pub fn parse_i32(raw_bytes: &[u8]) -> i32 {
    i32::from_le_bytes([raw_bytes[0], raw_bytes[1], raw_bytes[2], raw_bytes[3]])
}

/// Unsafe if the slice is less than 8 bytes
pub fn parse_i64(raw_bytes: &[u8]) -> i64 {
    i64::from_le_bytes([
        raw_bytes[0],
        raw_bytes[1],
        raw_bytes[2],
        raw_bytes[3],
        raw_bytes[4],
        raw_bytes[5],
        raw_bytes[6],
        raw_bytes[7],
    ])
}

/// Unsafe if the slice is less than 4 bytes
pub fn parse_u32(raw_bytes: &[u8]) -> u32 {
    u32::from_le_bytes([raw_bytes[0], raw_bytes[1], raw_bytes[2], raw_bytes[3]])
}

/// Unsafe if the slice is less than 8 bytes
pub fn parse_u64(raw_bytes: &[u8]) -> u64 {
    u64::from_le_bytes([
        raw_bytes[0],
        raw_bytes[1],
        raw_bytes[2],
        raw_bytes[3],
        raw_bytes[4],
        raw_bytes[5],
        raw_bytes[6],
        raw_bytes[7],
    ])
}

#[derive(Debug, Copy, Clone)]
pub struct WrappedF64(pub f64);

impl PartialEq for WrappedF64 {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

impl Eq for WrappedF64 {}

impl Hash for WrappedF64 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let bits = self.0.to_bits();
        bits.hash(state);
    }
}

impl From<f64> for WrappedF64 {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl Deref for WrappedF64 {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for WrappedF64 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Copy, Clone)]
pub struct WrappedF32(pub f32);

impl PartialEq for WrappedF32 {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

impl Eq for WrappedF32 {}

impl Hash for WrappedF32 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let bits = self.0.to_bits();
        bits.hash(state);
    }
}

impl From<f32> for WrappedF32 {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl Deref for WrappedF32 {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for WrappedF32 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

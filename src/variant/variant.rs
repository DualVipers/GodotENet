use std::any::Any;
use std::fmt::Debug;

// TODO: Custom Variant Decode Error?

pub trait Variant: Any + Send + Sync + Debug + crate::DynEq + crate::DynHash {
    fn encode(&self) -> Result<Vec<u8>, String>;

    /// Raw Bytes does not include the header
    fn decode(header: u32, raw_bytes: &[u8]) -> Result<super::DecodingResult<dyn Variant>, String>
    where
        Self: Sized;
}

impl dyn Variant {
    pub fn as_any(&self) -> &dyn Any {
        self
    }

    pub fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl std::hash::Hash for dyn Variant {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.dyn_hash(state);
    }
}

impl PartialEq for dyn Variant {
    fn eq(&self, other: &Self) -> bool {
        return self.dyn_eq(other);
    }
}

impl Eq for dyn Variant {}

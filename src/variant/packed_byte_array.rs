use super::{DecodingResult, Variant, helpers};
use std::vec::Vec;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct PackedByteArray(pub Vec<u8>);

impl Variant for PackedByteArray {
    // Replicated from encode_variant in marshalls.cpp
    fn encode(&self) -> Result<Vec<u8>, String> {
        let header = 29u32;

        let mut encoded = Vec::new();

        encoded.extend(header.to_le_bytes());

        encoded.extend((self.len() as u32).to_le_bytes());
        encoded.extend(&self.0);

        Ok(encoded)
    }

    // Replicated from decode_variant in marshalls.cpp
    fn decode(_header: u32, raw_bytes: &[u8]) -> Result<DecodingResult<dyn Variant>, String>
    where
        Self: Sized,
    {
        if raw_bytes.len() < 4 {
            return Err("Not Enough Bytes to Decode PackedByteArray Variant".to_string());
        }

        let count = helpers::parse_u32(raw_bytes) as usize;

        let mut consumed = 4;

        if raw_bytes.len() < count + consumed {
            return Err("Not Enough Bytes to Decode PackedByteArray Variant".to_string());
        }

        let data = raw_bytes[consumed..(consumed + count)].to_vec();

        consumed += count as usize;

        Ok(DecodingResult {
            consumed: 4 + consumed,

            variant: Box::new(Self(data)),
        })
    }
}

impl From<Vec<u8>> for PackedByteArray {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for PackedByteArray {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

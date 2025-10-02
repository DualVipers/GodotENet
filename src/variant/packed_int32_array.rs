use super::{DecodingResult, Variant, helpers};
use std::vec::Vec;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct PackedInt32Array(pub Vec<i32>);

impl Variant for PackedInt32Array {
    // Replicated from encode_variant in marshalls.cpp
    fn encode(&self) -> Result<Vec<u8>, String> {
        let header = 30u32;

        let mut encoded = Vec::new();

        encoded.extend(header.to_le_bytes());

        encoded.extend((self.len() as u32).to_le_bytes());

        for &value in &self.0 {
            encoded.extend(value.to_le_bytes());
        }

        Ok(encoded)
    }

    // Replicated from decode_variant in marshalls.cpp
    fn decode(_header: u32, raw_bytes: &[u8]) -> Result<DecodingResult<dyn Variant>, String>
    where
        Self: Sized,
    {
        if raw_bytes.len() < 4 {
            return Err("Not Enough Bytes to Decode PackedInt32Array Variant".to_string());
        }

        let count = helpers::parse_u32(raw_bytes) as usize;

        let mut consumed = 4;

        if raw_bytes.len() < (4 * count) + consumed {
            return Err("Not Enough Bytes to Decode PackedInt32Array Variant".to_string());
        }

        let mut data = vec![0i32; count];

        for i in 0..count {
            data[i] = helpers::parse_i32(&raw_bytes[consumed..(consumed + 4)]);

            consumed += 4;
        }

        Ok(DecodingResult {
            consumed: 4 + consumed,

            variant: Box::new(Self(data)),
        })
    }
}

impl From<Vec<i32>> for PackedInt32Array {
    fn from(value: Vec<i32>) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for PackedInt32Array {
    type Target = Vec<i32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

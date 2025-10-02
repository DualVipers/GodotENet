use super::{DecodingResult, Variant, helpers};
use std::vec::Vec;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct PackedFloat64Array(pub Vec<helpers::WrappedF64>);

impl Variant for PackedFloat64Array {
    // Replicated from encode_variant in marshalls.cpp
    fn encode(&self) -> Result<Vec<u8>, String> {
        let header = 33u32;

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
            return Err("Not Enough Bytes to Decode PackedFloat64Array Variant".to_string());
        }

        let count = helpers::parse_u32(raw_bytes) as usize;

        let mut consumed = 4;

        if raw_bytes.len() < (8 * count) + consumed {
            return Err("Not Enough Bytes to Decode PackedFloat64Array Variant".to_string());
        }

        let mut data = vec![0f64.into(); count];

        for i in 0..count {
            data[i] = helpers::parse_f64(&raw_bytes[consumed..(consumed + 8)]).into();

            consumed += 8;
        }

        Ok(DecodingResult {
            consumed: 4 + consumed,

            variant: Box::new(Self(data)),
        })
    }
}

impl From<Vec<helpers::WrappedF64>> for PackedFloat64Array {
    fn from(value: Vec<helpers::WrappedF64>) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for PackedFloat64Array {
    type Target = Vec<helpers::WrappedF64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

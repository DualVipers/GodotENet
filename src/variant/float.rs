use super::{DecodingResult, Variant, helpers};
use std::hash::Hash;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Float(pub helpers::WrappedF64);

impl Variant for Float {
    // Replicated from encode_variant in marshalls.cpp
    fn encode(&self) -> Result<Vec<u8>, String> {
        let mut header = 2u32;

        if *self.0 as f32 as f64 != *self.0 {
            header |= super::HEADER_DATA_FLAG_64;
        }

        let mut encoded = header.to_le_bytes().to_vec();

        if header & super::HEADER_DATA_FLAG_64 != 0 {
            encoded.extend(self.0.to_le_bytes());
        } else {
            encoded.extend((*self.0 as f32).to_le_bytes());
        }

        Ok(encoded)
    }

    // Replicated from decode_variant in marshalls.cpp
    fn decode(header: u32, raw_bytes: &[u8]) -> Result<DecodingResult<dyn Variant>, String>
    where
        Self: Sized,
    {
        if (header & super::HEADER_DATA_FLAG_64) != 0 {
            if raw_bytes.len() < 8 {
                return Err("Not Enough Bytes to Decode 64-bit Float Variant".to_string());
            }

            return Ok(DecodingResult {
                consumed: 4 + 8,
                variant: Box::new(Self::from(helpers::parse_f64(raw_bytes))),
            });
        } else {
            if raw_bytes.len() < 4 {
                return Err("Not Enough Bytes to Decode 32-bit Float Variant".to_string());
            }

            return Ok(DecodingResult {
                consumed: 4 + 4,
                variant: Box::new(Self::from(helpers::parse_f32(raw_bytes) as f64)),
            });
        }
    }
}

impl From<f64> for Float {
    fn from(value: f64) -> Self {
        Self(value.into())
    }
}

impl std::ops::Deref for Float {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

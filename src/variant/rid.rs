use super::{DecodingResult, Variant, helpers};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Rid(pub u64);

impl Variant for Rid {
    // Replicated from encode_variant in marshalls.cpp
    fn encode(&self) -> Result<Vec<u8>, String> {
        let header = 23u32;

        let mut encoded = header.to_le_bytes().to_vec();

        encoded.extend(self.0.to_le_bytes());

        Ok(encoded)
    }

    // Replicated from decode_variant in marshalls.cpp
    fn decode(_header: u32, raw_bytes: &[u8]) -> Result<DecodingResult<dyn Variant>, String>
    where
        Self: Sized,
    {
        if raw_bytes.len() < 8 {
            return Err("Not Enough Bytes to Decode RID Variant".to_string());
        }

        return Ok(DecodingResult {
            consumed: 4 + 8,
            variant: Box::new(Self::from(helpers::parse_u64(raw_bytes))),
        });
    }
}

impl From<u64> for Rid {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for Rid {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

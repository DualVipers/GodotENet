use super::{DecodingResult, Variant};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Bool(pub bool);

impl Variant for Bool {
    fn encode(&self) -> Result<Vec<u8>, String> {
        let mut encoded = 1u32.to_le_bytes().to_vec();

        encoded.extend(if self.0 {
            1u32.to_le_bytes()
        } else {
            0u32.to_le_bytes()
        });

        Ok(encoded)
    }

    // Replicated from decode_variant in marshalls.cpp
    fn decode(_header: u32, raw_bytes: &[u8]) -> Result<DecodingResult<dyn Variant>, String>
    where
        Self: Sized,
    {
        if raw_bytes.len() < 4 {
            return Err("Not Enough Bytes to Decode Bool Variant".to_string());
        }

        return Ok(DecodingResult {
            consumed: 4 + 4,
            variant: Box::new(Self(
                u32::from_le_bytes([raw_bytes[0], raw_bytes[1], raw_bytes[2], raw_bytes[3]]) != 0,
            )),
        });
    }
}

impl Bool {
    pub fn encode_compressed(&self) -> Result<Vec<u8>, String> {
        return if self.0 {
            Ok(vec![super::VARIANT_META_BOOL_MASK])
        } else {
            Ok(vec![0u8])
        };
    }

    pub fn decode_compressed(raw_bytes: &[u8]) -> Result<DecodingResult<dyn Variant>, String>
    where
        Self: Sized,
    {
        return if raw_bytes[0] & super::VARIANT_META_TYPE_MASK == 1 {
            Ok(DecodingResult {
                consumed: 1,
                variant: Box::new(Self((raw_bytes[0] & super::VARIANT_META_BOOL_MASK) > 0)),
            })
        } else {
            Err("Invalid Header for Nil Variant".to_string())
        };
    }
}

impl From<bool> for Bool {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for Bool {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

use super::{DecodingResult, Variant};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Nil;

impl Variant for Nil {
    fn encode(&self) -> Result<Vec<u8>, String> {
        Ok(0u32.to_le_bytes().to_vec())
    }

    // Replicated from decode_variant in marshalls.cpp
    fn decode(header: u32, _raw_bytes: &[u8]) -> Result<DecodingResult<dyn Variant>, String>
    where
        Self: Sized,
    {
        return if header & super::HEADER_TYPE_MASK == 0 {
            Ok(DecodingResult {
                variant: Box::new(Self),
                consumed: 4,
            })
        } else {
            Err("Invalid Header for Nil Variant".to_string())
        };
    }
}

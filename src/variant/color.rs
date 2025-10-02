use super::{DecodingResult, Variant, helpers};
use std::hash::Hash;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Color {
    pub r: helpers::WrappedF32,
    pub g: helpers::WrappedF32,
    pub b: helpers::WrappedF32,
    pub a: helpers::WrappedF32,
}

impl Variant for Color {
    // Replicated from encode_variant in marshalls.cpp
    fn encode(&self) -> Result<Vec<u8>, String> {
        let header = 20u32;

        let mut encoded = header.to_le_bytes().to_vec();

        encoded.extend(self.r.to_le_bytes());
        encoded.extend(self.g.to_le_bytes());
        encoded.extend(self.b.to_le_bytes());
        encoded.extend(self.a.to_le_bytes());

        Ok(encoded)
    }

    // Replicated from decode_variant in marshalls.cpp
    fn decode(_header: u32, raw_bytes: &[u8]) -> Result<DecodingResult<dyn Variant>, String>
    where
        Self: Sized,
    {
        let mut color = [0.0; 4];
        let mut consumed = 0;

        if raw_bytes.len() < 4 * 4 {
            return Err("Not Enough Bytes to Decode 32-bit Color Variant".to_string());
        }

        for i in 0..4 {
            color[i] = helpers::parse_f32(&raw_bytes[consumed..(consumed + 4)]);
            consumed += 4;
        }

        return Ok(DecodingResult {
            consumed: 4 + consumed,
            variant: Box::new(Self {
                r: color[0].into(),
                g: color[1].into(),
                b: color[2].into(),
                a: color[3].into(),
            }),
        });
    }
}

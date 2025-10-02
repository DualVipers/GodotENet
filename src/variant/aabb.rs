use super::{DecodingResult, Variant, helpers};
use std::hash::Hash;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct AABB {
    pub position: [helpers::WrappedF64; 3],
    pub size: [helpers::WrappedF64; 3],
}

impl Variant for AABB {
    // Replicated from encode_variant in marshalls.cpp
    fn encode(&self) -> Result<Vec<u8>, String> {
        let mut header = 16u32;

        // Replaces Compile Time Behavior of Godot
        if *self.position[0] as f32 as f64 != *self.position[0]
            || *self.position[1] as f32 as f64 != *self.position[1]
            || *self.position[2] as f32 as f64 != *self.position[2]
            || *self.size[0] as f32 as f64 != *self.size[0]
            || *self.size[1] as f32 as f64 != *self.size[1]
            || *self.size[2] as f32 as f64 != *self.size[2]
        {
            header |= super::HEADER_DATA_FLAG_64;
        }

        let mut encoded = header.to_le_bytes().to_vec();

        if header & super::HEADER_DATA_FLAG_64 != 0 {
            for i in 0..3 {
                encoded.extend(self.position[i].to_le_bytes());
            }
            for i in 0..3 {
                encoded.extend(self.size[i].to_le_bytes());
            }
        } else {
            for i in 0..3 {
                encoded.extend((*self.position[i] as f32).to_le_bytes());
            }
            for i in 0..3 {
                encoded.extend((*self.size[i] as f32).to_le_bytes());
            }
        }

        Ok(encoded)
    }

    // Replicated from decode_variant in marshalls.cpp
    fn decode(header: u32, raw_bytes: &[u8]) -> Result<DecodingResult<dyn Variant>, String>
    where
        Self: Sized,
    {
        let mut position = [helpers::WrappedF64(0.0); 3];
        let mut size = [helpers::WrappedF64(0.0); 3];
        let mut consumed = 0;

        if (header & super::HEADER_DATA_FLAG_64) != 0 {
            if raw_bytes.len() < consumed + 6 * 8 {
                return Err("Not Enough Bytes to Decode 64-bit AABB Variant".to_string());
            }

            for i in 0..3 {
                position[i] = helpers::parse_f64(&raw_bytes[consumed..(consumed + 8)]).into();
                consumed += 8;
            }

            for i in 0..3 {
                size[i] = helpers::parse_f64(&raw_bytes[consumed..(consumed + 8)]).into();
                consumed += 8;
            }
        } else {
            if raw_bytes.len() < consumed + 6 * 4 {
                return Err("Not Enough Bytes to Decode 32-bit AABB Variant".to_string());
            }

            for i in 0..3 {
                position[i] =
                    (helpers::parse_f32(&raw_bytes[consumed..(consumed + 4)]) as f64).into();
                consumed += 4;
            }

            for i in 0..3 {
                size[i] = (helpers::parse_f32(&raw_bytes[consumed..(consumed + 4)]) as f64).into();
                consumed += 4;
            }
        }

        return Ok(DecodingResult {
            consumed: 4 + consumed,
            variant: Box::new(Self { position, size }),
        });
    }
}

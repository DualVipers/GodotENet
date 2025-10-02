use super::{DecodingResult, Variant, helpers};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Projection(pub [[helpers::WrappedF64; 4]; 4]);

impl Variant for Projection {
    // Replicated from encode_variant in marshalls.cpp
    fn encode(&self) -> Result<Vec<u8>, String> {
        let mut header = 19u32;

        // Replaces Compile Time Behavior of Godot
        for i in 0..4 {
            for j in 0..4 {
                if header & super::HEADER_DATA_FLAG_64 == 0
                    && *self.0[i][j] as f32 as f64 != *self.0[i][j]
                {
                    header |= super::HEADER_DATA_FLAG_64;
                }
            }
        }

        let mut encoded = header.to_le_bytes().to_vec();

        if header & super::HEADER_DATA_FLAG_64 != 0 {
            for i in 0..4 {
                for j in 0..4 {
                    encoded.extend(self.0[i][j].to_le_bytes());
                }
            }
        } else {
            for i in 0..4 {
                for j in 0..4 {
                    encoded.extend((*self.0[i][j] as f32).to_le_bytes());
                }
            }
        }

        Ok(encoded)
    }

    // Replicated from decode_variant in marshalls.cpp
    fn decode(header: u32, raw_bytes: &[u8]) -> Result<DecodingResult<dyn Variant>, String>
    where
        Self: Sized,
    {
        let mut projection = [[helpers::WrappedF64(0.0); 4]; 4];
        let mut consumed = 0;

        if (header & super::HEADER_DATA_FLAG_64) != 0 {
            if raw_bytes.len() < consumed + 16 * 8 {
                return Err("Not Enough Bytes to Decode 64-bit Projection Variant".to_string());
            }

            for i in 0..4 {
                for j in 0..4 {
                    projection[i][j] =
                        helpers::parse_f64(&raw_bytes[consumed..(consumed + 8)]).into();
                    consumed += 8
                }
            }
        } else {
            if raw_bytes.len() < consumed + 16 * 4 {
                return Err("Not Enough Bytes to Decode 32-bit Projection Variant".to_string());
            }

            for i in 0..4 {
                for j in 0..4 {
                    projection[i][j] =
                        (helpers::parse_f32(&raw_bytes[consumed..(consumed + 4)]) as f64).into();
                    consumed += 4;
                }
            }
        }

        return Ok(DecodingResult {
            consumed: 4 + consumed,
            variant: Box::new(Self(projection)),
        });
    }
}

impl From<[[f64; 4]; 4]> for Projection {
    fn from(value: [[f64; 4]; 4]) -> Self {
        let mut wrapped = [[helpers::WrappedF64(0.0); 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                wrapped[i][j] = value[i][j].into();
            }
        }
        Self(wrapped)
    }
}

impl std::ops::Deref for Projection {
    type Target = [[helpers::WrappedF64; 4]; 4];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

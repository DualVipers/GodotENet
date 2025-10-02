use super::{DecodingResult, Variant, helpers};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Transform2D(pub [[helpers::WrappedF64; 2]; 3]);

impl Variant for Transform2D {
    // Replicated from encode_variant in marshalls.cpp
    fn encode(&self) -> Result<Vec<u8>, String> {
        let mut header = 11u32;

        // Replaces Compile Time Behavior of Godot
        for i in 0..3 {
            for j in 0..2 {
                if header & super::HEADER_DATA_FLAG_64 == 0
                    && *self.0[i][j] as f32 as f64 != *self.0[i][j]
                {
                    header |= super::HEADER_DATA_FLAG_64;
                }
            }
        }

        let mut encoded = header.to_le_bytes().to_vec();

        if header & super::HEADER_DATA_FLAG_64 != 0 {
            for i in 0..3 {
                for j in 0..2 {
                    encoded.extend(self.0[i][j].to_le_bytes());
                }
            }
        } else {
            for i in 0..3 {
                for j in 0..2 {
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
        let mut transform = [[helpers::WrappedF64(0.0); 2]; 3];
        let mut consumed = 0;

        if (header & super::HEADER_DATA_FLAG_64) != 0 {
            if raw_bytes.len() < consumed + 6 * 8 {
                return Err("Not Enough Bytes to Decode 64-bit Transform2D Variant".to_string());
            }

            for i in 0..3 {
                for j in 0..2 {
                    transform[i][j] =
                        helpers::parse_f64(&raw_bytes[consumed..(consumed + 8)]).into();
                    consumed += 8
                }
            }
        } else {
            if raw_bytes.len() < consumed + 6 * 4 {
                return Err("Not Enough Bytes to Decode 32-bit Transform2D Variant".to_string());
            }

            for i in 0..3 {
                for j in 0..2 {
                    transform[i][j] =
                        (helpers::parse_f32(&raw_bytes[consumed..(consumed + 4)]) as f64).into();
                    consumed += 4;
                }
            }
        }

        return Ok(DecodingResult {
            consumed: 4 + consumed,
            variant: Box::new(Self(transform)),
        });
    }
}

impl From<[[f64; 2]; 3]> for Transform2D {
    fn from(value: [[f64; 2]; 3]) -> Self {
        let mut wrapped = [[helpers::WrappedF64(0.0); 2]; 3];

        for i in 0..3 {
            for j in 0..2 {
                wrapped[i][j] = value[i][j].into();
            }
        }

        Self(wrapped)
    }
}

impl std::ops::Deref for Transform2D {
    type Target = [[helpers::WrappedF64; 2]; 3];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

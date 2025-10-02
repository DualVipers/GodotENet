use super::{DecodingResult, Variant, helpers};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Transform3D {
    pub basis: [[helpers::WrappedF64; 3]; 3],
    pub origin: [helpers::WrappedF64; 3],
}

impl Variant for Transform3D {
    // Replicated from encode_variant in marshalls.cpp
    fn encode(&self) -> Result<Vec<u8>, String> {
        let mut header = 18u32;

        // Replaces Compile Time Behavior of Godot
        for i in 0..3 {
            for j in 0..3 {
                if header & super::HEADER_DATA_FLAG_64 == 0
                    && *self.basis[i][j] as f32 as f64 != *self.basis[i][j]
                {
                    header |= super::HEADER_DATA_FLAG_64;
                }
            }
        }

        for i in 0..3 {
            if header & super::HEADER_DATA_FLAG_64 == 0
                && *self.origin[i] as f32 as f64 != *self.origin[i]
            {
                header |= super::HEADER_DATA_FLAG_64;
            }
        }

        let mut encoded = header.to_le_bytes().to_vec();

        if header & super::HEADER_DATA_FLAG_64 != 0 {
            for i in 0..3 {
                for j in 0..3 {
                    encoded.extend(self.basis[i][j].to_le_bytes());
                }
            }

            for i in 0..3 {
                encoded.extend(self.origin[i].to_le_bytes());
            }
        } else {
            for i in 0..3 {
                for j in 0..3 {
                    encoded.extend((*self.basis[i][j] as f32).to_le_bytes());
                }
            }

            for i in 0..3 {
                encoded.extend((*self.origin[i] as f32).to_le_bytes());
            }
        }

        Ok(encoded)
    }

    // Replicated from decode_variant in marshalls.cpp
    fn decode(header: u32, raw_bytes: &[u8]) -> Result<DecodingResult<dyn Variant>, String>
    where
        Self: Sized,
    {
        let mut basis = [[helpers::WrappedF64(0.0); 3]; 3];
        let mut origin = [helpers::WrappedF64(0.0); 3];
        let mut consumed = 0;

        if (header & super::HEADER_DATA_FLAG_64) != 0 {
            if raw_bytes.len() < consumed + 12 * 8 {
                return Err("Not Enough Bytes to Decode 64-bit Transform3D Variant".to_string());
            }

            for i in 0..3 {
                for j in 0..3 {
                    basis[i][j] = helpers::parse_f64(&raw_bytes[consumed..(consumed + 8)]).into();
                    consumed += 8
                }
            }

            for i in 0..3 {
                origin[i] = helpers::parse_f64(&raw_bytes[consumed..(consumed + 8)]).into();
                consumed += 8;
            }
        } else {
            if raw_bytes.len() < consumed + 12 * 4 {
                return Err("Not Enough Bytes to Decode 32-bit Transform3D Variant".to_string());
            }

            for i in 0..3 {
                for j in 0..3 {
                    basis[i][j] =
                        (helpers::parse_f32(&raw_bytes[consumed..(consumed + 4)]) as f64).into();
                    consumed += 4;
                }
            }

            for i in 0..3 {
                origin[i] =
                    (helpers::parse_f32(&raw_bytes[consumed..(consumed + 4)]) as f64).into();
                consumed += 4;
            }
        }

        return Ok(DecodingResult {
            consumed: 4 + consumed,
            variant: Box::new(Self { basis, origin }),
        });
    }
}

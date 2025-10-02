use super::{DecodingResult, Variant, helpers};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Int(pub i64);

impl Variant for Int {
    // Replicated from encode_variant in marshalls.cpp
    fn encode(&self) -> Result<Vec<u8>, String> {
        let mut header = 2u32;

        if self.0 < i32::MIN as i64 || self.0 > i32::MAX as i64 {
            header |= super::HEADER_DATA_FLAG_64;
        }

        let mut encoded = header.to_le_bytes().to_vec();

        if header & super::HEADER_DATA_FLAG_64 != 0 {
            encoded.extend(self.0.to_le_bytes());
        } else {
            encoded.extend((self.0 as i32).to_le_bytes());
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
                return Err("Not Enough Bytes to Decode 64-bit Int Variant".to_string());
            }

            return Ok(DecodingResult {
                consumed: 4 + 8,
                variant: Box::new(Self::from(helpers::parse_i64(raw_bytes))),
            });
        } else {
            if raw_bytes.len() < 4 {
                return Err("Not Enough Bytes to Decode 32-bit Int Variant".to_string());
            }

            return Ok(DecodingResult {
                consumed: 4 + 4,
                variant: Box::new(Self::from(helpers::parse_i32(raw_bytes) as i64)),
            });
        }
    }
}

impl Int {
    pub fn encode_compressed(&self) -> Result<Vec<u8>, String> {
        // TODO: Encode Compressed Int Variants

        return Err("Can Not Encode Compressed Int Variants".to_string());
    }

    // Replicated from decode_and_decompress_variant in multiplayer_api.cpp
    pub fn decode_compressed(raw_bytes: &[u8]) -> Result<DecodingResult<dyn Variant>, String>
    where
        Self: Sized,
    {
        let encode_mode = raw_bytes[0] & super::VARIANT_META_EMODE_MASK;

        if encode_mode == 0 << 6 {
            if raw_bytes.len() < 1 {
                return Err("Not Enough Bytes to Decode Compressed 8-bit Int Variant".to_string());
            }

            return Ok(DecodingResult {
                consumed: 1 + 1,
                variant: Box::new(Self::from(raw_bytes[1] as i64)),
            });
        } else if encode_mode == 1 << 6 {
            if raw_bytes.len() < 2 {
                return Err("Not Enough Bytes to Decode Compressed 16-bit Int Variant".to_string());
            }

            return Ok(DecodingResult {
                consumed: 1 + 2,
                variant: Box::new(Self::from(
                    u16::from_le_bytes([raw_bytes[1], raw_bytes[2]]) as i64
                )),
            });
        } else if encode_mode == 2 << 6 {
            if raw_bytes.len() < 4 {
                return Err("Not Enough Bytes to Decode Compressed 32-bit Int Variant".to_string());
            }

            return Ok(DecodingResult {
                consumed: 1 + 4,
                variant: Box::new(Self::from(helpers::parse_u32(raw_bytes) as i64)),
            });
        } else {
            if raw_bytes.len() < 1 {
                return Err("Not Enough Bytes to Decode Compressed 64-bit Int Variant".to_string());
            }

            return Ok(DecodingResult {
                consumed: 1 + 8,
                variant: Box::new(Self::from(helpers::parse_u64(raw_bytes) as i64)),
            });
        }
    }
}

impl From<i64> for Int {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for Int {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

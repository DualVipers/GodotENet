use super::{DecodingResult, Variant, helpers};
use std::vec::Vec;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct PackedStringArray(pub Vec<String>);

impl Variant for PackedStringArray {
    // Replicated from encode_variant in marshalls.cpp
    fn encode(&self) -> Result<Vec<u8>, String> {
        let header = 34u32;

        let mut encoded = Vec::new();

        encoded.extend(header.to_le_bytes());

        encoded.extend((self.len() as u32).to_le_bytes());

        for value in &self.0 {
            encoded.extend(value.as_bytes());

            if (value.len() % 4) != 0 {
                // Padding
                let padding = 4 - (value.len() % 4);
                encoded.extend(vec![0u8; padding as usize]);
            }
        }

        Ok(encoded)
    }

    // Replicated from decode_variant in marshalls.cpp
    fn decode(_header: u32, raw_bytes: &[u8]) -> Result<DecodingResult<dyn Variant>, String>
    where
        Self: Sized,
    {
        if raw_bytes.len() < 4 {
            return Err("Not Enough Bytes to Decode PackedStringArray Variant".to_string());
        }

        let count = helpers::parse_u32(raw_bytes) as usize;

        let mut consumed = 4;

        let mut data = vec!["".into(); count];

        for _ in 0..count {
            let str_len = helpers::parse_u32(raw_bytes) as usize;
            consumed += 4;

            if raw_bytes.len() < consumed + str_len {
                return Err(
                    "Not Enough Bytes to Decode String in PackedStringArray Variant".to_string(),
                );
            }

            let string_data =
                match String::from_utf8(raw_bytes[consumed..(consumed + str_len)].to_vec()) {
                    Ok(s) => s,
                    Err(_) => {
                        return Err(
                            "Invalid UTF-8 in String in PackedStringArray Variant".to_string()
                        );
                    }
                };
            consumed += str_len;

            if (str_len % 4) != 0 {
                let padding = 4 - (str_len % 4);
                if raw_bytes.len() < consumed + padding {
                    return Err(
                        "Not Enough Bytes to Decode String Padding in PackedStringArray Variant"
                            .to_string(),
                    );
                }
                consumed += padding;
            }

            data.push(string_data);
        }

        Ok(DecodingResult {
            consumed: 4 + consumed,

            variant: Box::new(Self(data)),
        })
    }
}

impl From<Vec<String>> for PackedStringArray {
    fn from(value: Vec<String>) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for PackedStringArray {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

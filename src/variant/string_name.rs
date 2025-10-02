use super::{DecodingResult, Variant, helpers};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct StringName(pub String);

impl Variant for StringName {
    // Replicated from _encode_string in marshalls.cpp
    fn encode(&self) -> Result<Vec<u8>, String> {
        let header = 21u32;

        let mut encoded = header.to_le_bytes().to_vec();

        let len = self.0.len() as u32;
        encoded.extend(len.to_le_bytes());

        encoded.extend(self.0.as_bytes()); // UTF-8 Encoding By Default

        if (len % 4) != 0 {
            // Padding
            let padding = 4 - (len % 4);
            encoded.extend(vec![0u8; padding as usize]);
        }

        Ok(encoded)
    }

    // Replicated from _decode_string in marshalls.cpp
    fn decode(_header: u32, raw_bytes: &[u8]) -> Result<DecodingResult<dyn Variant>, String>
    where
        Self: Sized,
    {
        if raw_bytes.len() < 4 {
            return Err("Not Enough Bytes to Decode String Name Length".to_string());
        }

        let str_len = helpers::parse_u32(raw_bytes) as usize;
        let mut consumed = 4;

        if raw_bytes.len() < consumed + str_len {
            return Err("Not Enough Bytes to Decode String Name Data".to_string());
        }

        let string_data =
            match String::from_utf8(raw_bytes[consumed..(consumed + str_len)].to_vec()) {
                Ok(s) => s,
                Err(_) => return Err("Invalid UTF-8 in String Name Variant".to_string()),
            };
        consumed += str_len;

        if (str_len % 4) != 0 {
            let padding = 4 - (str_len % 4);
            if raw_bytes.len() < consumed + padding {
                return Err("Not Enough Bytes to Decode String Name Padding".to_string());
            }
            consumed += padding;
        }

        return Ok(DecodingResult {
            consumed: 4 + consumed,
            variant: Box::new(Self::from(string_data)),
        });
    }
}

impl From<String> for StringName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for StringName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

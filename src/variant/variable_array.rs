use super::{DecodingResult, Variant, helpers};
use std::{ops::Deref, sync::Arc};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct VariableArray(Vec<Arc<Box<dyn Variant>>>);

impl Variant for VariableArray {
    // Replicated from encode_variant in marshalls.cpp
    fn encode(&self) -> Result<Vec<u8>, String> {
        let header = 28u32;

        let mut encoded = header.to_le_bytes().to_vec();

        let count = self.len() as u32;
        encoded.extend((count & 0x7FFFFFFF).to_le_bytes());

        for r in self.iter() {
            let encoded_variant = r.encode()?;
            encoded.extend(encoded_variant);
        }

        Ok(encoded)
    }

    // Replicated from decode_variant in marshalls.cpp
    fn decode(_header: u32, raw_bytes: &[u8]) -> Result<DecodingResult<dyn Variant>, String>
    where
        Self: Sized,
    {
        let mut offset = 0;

        let count = helpers::parse_u32(&raw_bytes[offset..]) & 0x7FFFFFFF;
        offset += 4;

        // Somehow setup typing for DashMap
        let mut array = Vec::new();

        for i in 0..count {
            let decoding_result = match crate::variant::decode_variant(&raw_bytes[offset..]) {
                Ok(result) => result,
                Err(e) => {
                    return Err(format!(
                        "Failed to Decode Key {} of {} in Dictionary Variant: \n{}",
                        i + 1,
                        count,
                        e
                    ));
                }
            };

            offset += decoding_result.consumed;

            array.push(Arc::new(decoding_result.variant));
        }

        Ok(DecodingResult {
            consumed: 4 + offset,
            variant: Box::new(Self(array)),
        })
    }
}

impl From<Vec<Arc<Box<dyn Variant>>>> for VariableArray {
    fn from(value: Vec<Arc<Box<dyn Variant>>>) -> Self {
        Self(value)
    }
}

impl Deref for VariableArray {
    type Target = Vec<Arc<Box<dyn Variant>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

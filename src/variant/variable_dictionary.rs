use super::{DecodingResult, Variant, helpers};
use dashmap::DashMap;
use std::{ops::Deref, sync::Arc};

#[derive(Clone, Debug)]
pub struct VariableDictionary(Arc<DashMap<Box<dyn Variant>, Box<dyn Variant>>>);

impl Variant for VariableDictionary {
    // Replicated from encode_variant in marshalls.cpp
    fn encode(&self) -> Result<Vec<u8>, String> {
        let header = 27u32;

        let mut encoded = header.to_le_bytes().to_vec();

        let count = self.len() as u32;
        encoded.extend((count & 0x7FFFFFFF).to_le_bytes());

        for r in self.iter() {
            let key_encoded = r.key().encode()?;
            encoded.extend(key_encoded);

            let value_encoded = r.value().encode()?;
            encoded.extend(value_encoded);
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
        let map = DashMap::<Box<dyn Variant>, Box<dyn Variant>>::new();

        for i in 0..count {
            let key_decoding_result = match crate::variant::decode_variant(&raw_bytes[offset..]) {
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

            offset += key_decoding_result.consumed;

            let value_decoding_result = match crate::variant::decode_variant(&raw_bytes[offset..]) {
                Ok(result) => result,
                Err(e) => {
                    return Err(format!(
                        "Failed to Decode Value {} of {} in Dictionary Variant: \n{}",
                        i + 1,
                        count,
                        e
                    ));
                }
            };

            offset += value_decoding_result.consumed;

            map.insert(key_decoding_result.variant, value_decoding_result.variant);
        }

        Ok(DecodingResult {
            consumed: 4 + offset,
            variant: Box::new(Self(Arc::new(map))),
        })
    }
}

impl PartialEq for VariableDictionary {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        for r in self.iter() {
            match other.get(r.key()) {
                Some(v) => {
                    return r.value().deref().eq(&**v.deref()); // woof, that is ugly
                }
                None => {
                    return false;
                }
            }
        }

        return true;
    }
}

impl Eq for VariableDictionary {}

impl std::hash::Hash for VariableDictionary {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for r in self.iter() {
            r.key().hash(state);
            r.value().hash(state);
        }
    }
}

impl From<DashMap<Box<dyn Variant>, Box<dyn Variant>>> for VariableDictionary {
    fn from(value: DashMap<Box<dyn Variant>, Box<dyn Variant>>) -> Self {
        Self(Arc::new(value))
    }
}

impl From<Arc<DashMap<Box<dyn Variant>, Box<dyn Variant>>>> for VariableDictionary {
    fn from(value: Arc<DashMap<Box<dyn Variant>, Box<dyn Variant>>>) -> Self {
        Self(value)
    }
}

impl Deref for VariableDictionary {
    type Target = Arc<DashMap<Box<dyn Variant>, Box<dyn Variant>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

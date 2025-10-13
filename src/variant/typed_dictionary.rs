use super::{DecodingResult, Variant, helpers};
use dashmap::DashMap;
use std::{any::TypeId, hash::Hash, ops::Deref, sync::Arc};

#[derive(Clone, Debug)]
pub struct TypedDictionary<K, V>(Arc<DashMap<K, V>>)
where
    K: Variant + Eq + Hash + Clone,
    V: Variant + Eq + Hash + Clone;

impl<K, V> Variant for TypedDictionary<K, V>
where
    K: Variant + Eq + Hash + Clone,
    V: Variant + Eq + Hash + Clone,
{
    // Replicated from encode_variant in marshalls.cpp
    fn encode(&self) -> Result<Vec<u8>, String> {
        let mut header = 27u32;

        header |= 1 << super::HEADER_DATA_FIELD_TYPED_DICTIONARY_KEY_SHIFT;
        header |= 1 << super::HEADER_DATA_FIELD_TYPED_DICTIONARY_VALUE_SHIFT;

        let key_type: u32;
        let value_type: u32;

        if TypeId::of::<K>() == TypeId::of::<super::Bool>() {
            key_type = 1;
        } else if TypeId::of::<K>() == TypeId::of::<super::Int>() {
            key_type = 2;
        } else if TypeId::of::<K>() == TypeId::of::<super::Float>() {
            key_type = 3;
        } else if TypeId::of::<K>() == TypeId::of::<super::VariantString>() {
            key_type = 4;
        } else {
            return Err("Cannot Encode Typed Dictionary with Unknown Key Type".to_string());
        }

        if TypeId::of::<V>() == TypeId::of::<super::Bool>() {
            value_type = 1;
        } else if TypeId::of::<V>() == TypeId::of::<super::Int>() {
            value_type = 2;
        } else if TypeId::of::<V>() == TypeId::of::<super::Float>() {
            value_type = 3;
        } else if TypeId::of::<V>() == TypeId::of::<super::VariantString>() {
            value_type = 4;
        } else {
            return Err("Cannot Encode Typed Dictionary with Unknown Value Type".to_string());
        }

        let mut encoded = header.to_le_bytes().to_vec();

        encoded.extend(key_type.to_le_bytes());
        encoded.extend(value_type.to_le_bytes());

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
        let map = DashMap::<K, V>::new();

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

            if key_decoding_result
                .variant
                .as_any()
                .downcast_ref::<K>()
                .is_none()
            {
                return Err(format!(
                    "Failed to Downcast Key {} of {} in Dictionary Variant to Expected Type",
                    i + 1,
                    count
                ));
            }

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

            if value_decoding_result
                .variant
                .as_any()
                .downcast_ref::<V>()
                .is_none()
            {
                return Err(format!(
                    "Failed to Downcast Value {} of {} in Dictionary Variant to Expected Type",
                    i + 1,
                    count
                ));
            }

            offset += value_decoding_result.consumed;

            map.insert(
                key_decoding_result
                    .variant
                    .as_any()
                    .downcast_ref::<K>()
                    .unwrap()
                    .clone(),
                value_decoding_result
                    .variant
                    .as_any()
                    .downcast_ref::<V>()
                    .unwrap()
                    .clone(),
            );
        }

        Ok(DecodingResult {
            consumed: 4 + offset,
            variant: Box::new(Self(Arc::new(map))),
        })
    }
}

impl<K, V> PartialEq for TypedDictionary<K, V>
where
    K: Variant + Eq + Hash + Clone,
    V: Variant + Eq + Hash + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        for r in self.iter() {
            match other.get(r.key()) {
                Some(v) => {
                    return r.value().eq(&*v.deref());
                }
                None => {
                    return false;
                }
            }
        }

        return true;
    }
}

impl<K, V> Eq for TypedDictionary<K, V>
where
    K: Variant + Eq + Hash + Clone,
    V: Variant + Eq + Hash + Clone,
{
}

impl<K, V> std::hash::Hash for TypedDictionary<K, V>
where
    K: Variant + Eq + Hash + Clone,
    V: Variant + Eq + Hash + Clone,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for r in self.iter() {
            r.key().hash(state);
            r.value().hash(state);
        }
    }
}

impl<K, V> From<DashMap<K, V>> for TypedDictionary<K, V>
where
    K: Variant + Eq + Hash + Clone,
    V: Variant + Eq + Hash + Clone,
{
    fn from(value: DashMap<K, V>) -> Self {
        Self(Arc::new(value))
    }
}

impl<K, V> From<Arc<DashMap<K, V>>> for TypedDictionary<K, V>
where
    K: Variant + Eq + Hash + Clone,
    V: Variant + Eq + Hash + Clone,
{
    fn from(value: Arc<DashMap<K, V>>) -> Self {
        Self(value)
    }
}

impl<K, V> std::ops::Deref for TypedDictionary<K, V>
where
    K: Variant + Eq + Hash + Clone,
    V: Variant + Eq + Hash + Clone,
{
    type Target = Arc<DashMap<K, V>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

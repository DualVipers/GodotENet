use super::{DecodingResult, Variant, helpers};
use std::{hash::Hash, ops::Deref, sync::Arc};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct TypedArray<T>(Vec<Arc<T>>)
where
    T: Variant + Hash + Eq + Clone;

impl<T> Variant for TypedArray<T>
where
    T: Variant + Hash + Eq + Clone,
{
    // Replicated from encode_variant in marshalls.cpp
    fn encode(&self) -> Result<Vec<u8>, String> {
        let header = 27u32;

        // Todo: Check and Encode Key and Value Types Maybe?

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

            if decoding_result
                .variant
                .as_any()
                .downcast_ref::<T>()
                .is_none()
            {
                return Err(format!(
                    "Failed to Downcast Value {} of {} in Dictionary Array to Expected Type",
                    i + 1,
                    count
                ));
            }

            offset += decoding_result.consumed;

            array.push(Arc::new(
                decoding_result
                    .variant
                    .as_any()
                    .downcast_ref::<T>()
                    .unwrap()
                    .clone(),
            ));
        }

        Ok(DecodingResult {
            consumed: 4 + offset,
            variant: Box::new(Self(array)),
        })
    }
}

impl<T> From<Vec<Arc<T>>> for TypedArray<T>
where
    T: Variant + Hash + Eq + Clone,
{
    fn from(value: Vec<Arc<T>>) -> Self {
        Self(value)
    }
}

impl<T> Deref for TypedArray<T>
where
    T: Variant + Hash + Eq + Clone,
{
    type Target = Vec<Arc<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

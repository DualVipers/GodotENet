use super::{DecodingResult, Variant, helpers};
use std::{any::TypeId, hash::Hash, ops::Deref, sync::Arc};

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
        let mut header = 28u32;

        header |= 1 << super::HEADER_DATA_FIELD_TYPED_ARRAY_SHIFT;

        let value_type: u32;

        match TypeId::of::<T>() {
            id if id == TypeId::of::<super::Bool>() => {
                value_type = 1;
            }
            id if id == TypeId::of::<super::Int>() => {
                value_type = 2;
            }
            id if id == TypeId::of::<super::Float>() => {
                value_type = 3;
            }
            id if id == TypeId::of::<super::VariantString>() => {
                value_type = 4;
            }
            id if id == TypeId::of::<super::Vector2>() => {
                value_type = 5;
            }
            id if id == TypeId::of::<super::Vector2I>() => {
                value_type = 6;
            }
            id if id == TypeId::of::<super::Rect2>() => {
                value_type = 7;
            }
            id if id == TypeId::of::<super::Rect2I>() => {
                value_type = 8;
            }
            id if id == TypeId::of::<super::Vector3>() => {
                value_type = 9;
            }
            id if id == TypeId::of::<super::Vector3I>() => {
                value_type = 10;
            }
            id if id == TypeId::of::<super::Transform2D>() => {
                value_type = 11;
            }
            id if id == TypeId::of::<super::Vector4>() => {
                value_type = 12;
            }
            id if id == TypeId::of::<super::Vector4I>() => {
                value_type = 13;
            }
            id if id == TypeId::of::<super::Plane>() => {
                value_type = 14;
            }
            id if id == TypeId::of::<super::Quaternion>() => {
                value_type = 15;
            }
            id if id == TypeId::of::<super::AABB>() => {
                value_type = 16;
            }
            id if id == TypeId::of::<super::Basis>() => {
                value_type = 17;
            }
            id if id == TypeId::of::<super::Transform3D>() => {
                value_type = 18;
            }
            id if id == TypeId::of::<super::Projection>() => {
                value_type = 19;
            }
            id if id == TypeId::of::<super::Color>() => {
                value_type = 20;
            }
            _ => {
                return Err("Cannot Encode Typed Array with Unknown Value Type".to_string());
            }
        }

        let mut encoded = header.to_le_bytes().to_vec();

        encoded.extend(value_type.to_le_bytes());

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

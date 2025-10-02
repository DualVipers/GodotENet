use crate::variant::TypedArray;

use super::{DecodingResult, TypedDictionary, Variant, helpers};

// Replicated from decode_variant in marshalls.cpp and others
pub fn decode_array(header: u32, raw_bytes: &[u8]) -> Result<DecodingResult<dyn Variant>, String> {
    let type_kind: u8 = (((header) & super::HEADER_DATA_FIELD_TYPED_ARRAY_MASK)
        >> super::HEADER_DATA_FIELD_TYPED_ARRAY_SHIFT) as u8;
    let mut value_type = 0;

    let mut offset = 0;

    if type_kind == 1 {
        if raw_bytes.len() + offset < 4 {
            return Err("Not Enough Bytes to Decode Built In Typed Array Variant".to_string());
        }

        value_type = helpers::parse_u32(&raw_bytes[offset..]);
        offset += 4;
    } else if type_kind != 0 {
        return Err("Decoding Non-Built In Typed Array Variants not supported.".to_string());
    }

    if value_type > 0 && value_type <= 20 {
        return Ok(match value_type {
            1 => TypedArray::<super::Bool>::decode(header, &raw_bytes[offset..])?,
            2 => TypedArray::<super::Int>::decode(header, &raw_bytes[offset..])?,
            3 => TypedArray::<super::Float>::decode(header, &raw_bytes[offset..])?,
            4 => TypedArray::<super::VariantString>::decode(header, &raw_bytes[offset..])?,
            5 => TypedArray::<super::Vector2>::decode(header, &raw_bytes[offset..])?,
            6 => TypedArray::<super::Vector2I>::decode(header, &raw_bytes[offset..])?,
            7 => TypedArray::<super::Rect2>::decode(header, &raw_bytes[offset..])?,
            8 => TypedArray::<super::Rect2I>::decode(header, &raw_bytes[offset..])?,
            9 => TypedArray::<super::Vector3>::decode(header, &raw_bytes[offset..])?,
            10 => TypedArray::<super::Vector3I>::decode(header, &raw_bytes[offset..])?,
            11 => TypedArray::<super::Transform2D>::decode(header, &raw_bytes[offset..])?,
            12 => TypedArray::<super::Vector4>::decode(header, &raw_bytes[offset..])?,
            13 => TypedArray::<super::Vector4I>::decode(header, &raw_bytes[offset..])?,
            14 => TypedArray::<super::Plane>::decode(header, &raw_bytes[offset..])?,
            15 => TypedArray::<super::Quaternion>::decode(header, &raw_bytes[offset..])?,
            16 => TypedArray::<super::AABB>::decode(header, &raw_bytes[offset..])?,
            17 => TypedArray::<super::Basis>::decode(header, &raw_bytes[offset..])?,
            18 => TypedArray::<super::Transform3D>::decode(header, &raw_bytes[offset..])?,
            19 => TypedArray::<super::Projection>::decode(header, &raw_bytes[offset..])?,
            20 => TypedArray::<super::Color>::decode(header, &raw_bytes[offset..])?,
            _ => {
                return Err(
                    format! {"Decoding Typed Array Variants with Type {:?} not Fully Supported.", value_type
                    },
                );
            }
        });
    } else if value_type != 0 {
        log::warn!(
            "Decoding Typed Array Variants with Type {:?} not Fully Supported, Falling Back To Variable.",
            value_type
        );
    }

    Ok(super::VariableArray::decode(header, &raw_bytes[offset..])?)
}

// Replicated from decode_variant in marshalls.cpp and others
pub fn decode_dictionary(
    header: u32,
    raw_bytes: &[u8],
) -> Result<DecodingResult<dyn Variant>, String> {
    let key_type_kind: u8 = (((header) & super::HEADER_DATA_FIELD_TYPED_DICTIONARY_KEY_MASK)
        >> super::HEADER_DATA_FIELD_TYPED_DICTIONARY_KEY_SHIFT) as u8;
    let mut key_type = 0;
    let value_type_kind: u8 = (((header) & super::HEADER_DATA_FIELD_TYPED_DICTIONARY_VALUE_MASK)
        >> super::HEADER_DATA_FIELD_TYPED_DICTIONARY_VALUE_SHIFT)
        as u8;
    let mut value_type = 0;

    let mut offset = 0;

    if key_type_kind == 1 {
        if raw_bytes.len() + offset < 4 {
            return Err("Not Enough Bytes to Decode Built In Typed Dictionary Variant".to_string());
        }

        key_type = helpers::parse_u32(&raw_bytes[offset..]);
        offset += 4;
    } else if key_type_kind != 0 {
        return Err("Decoding Non-Built In Typed Dictionary Variants not supported.".to_string());
    }

    if value_type_kind == 1 {
        if raw_bytes.len() + offset < 4 {
            return Err("Not Enough Bytes to Decode Built In Typed Dictionary Variant".to_string());
        }

        value_type = helpers::parse_u32(&raw_bytes[offset..]);
        offset += 4;
    } else if value_type_kind != 0 {
        return Err("Decoding Non-Built In Typed Dictionary Variants not supported.".to_string());
    }

    if key_type > 0 && key_type <= 4 && value_type > 0 && value_type <= 4 {
        return Ok(match key_type {
            1 => match value_type {
                1 => TypedDictionary::<super::Bool, super::Bool>::decode(
                    header,
                    &raw_bytes[offset..],
                )?,
                2 => TypedDictionary::<super::Bool, super::Int>::decode(
                    header,
                    &raw_bytes[offset..],
                )?,
                3 => TypedDictionary::<super::Bool, super::Float>::decode(
                    header,
                    &raw_bytes[offset..],
                )?,
                4 => TypedDictionary::<super::Bool, super::VariantString>::decode(
                    header,
                    &raw_bytes[offset..],
                )?,
                _ => {
                    return Err(
                        format! {"Decoding Typed Dictionary Variants with Keys Type {:?} not Fully Supported.", key_type},
                    );
                }
            },
            2 => match value_type {
                1 => TypedDictionary::<super::Int, super::Bool>::decode(
                    header,
                    &raw_bytes[offset..],
                )?,
                2 => {
                    TypedDictionary::<super::Int, super::Int>::decode(header, &raw_bytes[offset..])?
                }
                3 => TypedDictionary::<super::Int, super::Float>::decode(
                    header,
                    &raw_bytes[offset..],
                )?,
                4 => TypedDictionary::<super::Int, super::VariantString>::decode(
                    header,
                    &raw_bytes[offset..],
                )?,
                _ => {
                    return Err(
                        format! {"Decoding Typed Dictionary Variants with Keys Type {:?} not Fully Supported.", key_type
                        },
                    );
                }
            },
            3 => match value_type {
                1 => TypedDictionary::<super::Float, super::Bool>::decode(
                    header,
                    &raw_bytes[offset..],
                )?,
                2 => TypedDictionary::<super::Float, super::Int>::decode(
                    header,
                    &raw_bytes[offset..],
                )?,
                3 => TypedDictionary::<super::Float, super::Float>::decode(
                    header,
                    &raw_bytes[offset..],
                )?,
                4 => TypedDictionary::<super::Float, super::VariantString>::decode(
                    header,
                    &raw_bytes[offset..],
                )?,
                _ => {
                    return Err(
                        format! {"Decoding Typed Dictionary Variants with Keys Type {:?} not Fully Supported.", key_type
                        },
                    );
                }
            },
            4 => match value_type {
                1 => TypedDictionary::<super::VariantString, super::Bool>::decode(
                    header,
                    &raw_bytes[offset..],
                )?,
                2 => TypedDictionary::<super::VariantString, super::Int>::decode(
                    header,
                    &raw_bytes[offset..],
                )?,
                3 => TypedDictionary::<super::VariantString, super::Float>::decode(
                    header,
                    &raw_bytes[offset..],
                )?,
                4 => TypedDictionary::<super::VariantString, super::VariantString>::decode(
                    header,
                    &raw_bytes[offset..],
                )?,
                _ => {
                    return Err(
                        format! {"Decoding Typed Dictionary Variants with Keys Type {:?} not Fully Supported.", key_type
                        },
                    );
                }
            },
            _ => {
                return Err(
                    format! {"Decoding Typed Dictionary Variants with Values Type {:?} not Fully Supported.", value_type
                    },
                );
            }
        });
    } else {
        if value_type != 0 {
            log::warn!(
                "Decoding Typed Dictionary Variants with Keys Type {:?} not Fully Supported, Falling Back To Variable.",
                value_type
            );
        }
        if key_type != 0 {
            log::warn!(
                "Decoding Typed Dictionary Variants with Values Type {:?} not Fully Supported, Falling Back To Variable.",
                key_type
            );
        }
    }

    Ok(super::VariableDictionary::decode(
        header,
        &raw_bytes[offset..],
    )?)
}

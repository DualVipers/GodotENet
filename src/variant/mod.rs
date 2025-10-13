mod aabb;
mod basis;
mod bool;
mod color;
mod float;
pub mod helpers;
mod int;
mod nil;
mod packed_byte_array;
mod packed_float32_array;
mod packed_float64_array;
mod packed_int32_array;
mod packed_int64_array;
mod packed_string_array;
mod plane;
mod projection;
mod quaternion;
mod rect2;
mod rect2i;
mod rid;
mod string;
mod string_name;
mod transform2d;
mod transform3d;
mod typed_array;
mod typed_dictionary;
mod variable_array;
mod variable_dictionary;
mod variable_typed;
mod variant;
mod vector2;
mod vector2i;
mod vector3;
mod vector3i;
mod vector4;
mod vector4i;

pub use aabb::*;
pub use basis::*;
pub use bool::*;
pub use color::*;
pub use float::*;
pub use int::*;
pub use nil::*;
pub use packed_byte_array::*;
pub use packed_float32_array::*;
pub use packed_float64_array::*;
pub use packed_int32_array::*;
pub use packed_int64_array::*;
pub use packed_string_array::*;
pub use plane::*;
pub use projection::*;
pub use quaternion::*;
pub use rect2::*;
pub use rect2i::*;
pub use rid::*;
pub use string::*;
pub use string_name::*;
pub use transform2d::*;
pub use transform3d::*;
pub use typed_array::*;
pub use typed_dictionary::*;
pub use variable_array::*;
pub use variable_dictionary::*;
pub use variable_typed::*;
pub use variant::*;
pub use vector2::*;
pub use vector2i::*;
pub use vector3::*;
pub use vector3i::*;
pub use vector4::*;
pub use vector4i::*;

// From multiplayer_api.cpp
const VARIANT_META_TYPE_MASK: u8 = 0x3F;
const VARIANT_META_EMODE_MASK: u8 = 0xC0;
const VARIANT_META_BOOL_MASK: u8 = 0x80;

// All From marshalls.cpp

const HEADER_TYPE_MASK: u32 = 0xFF;
// For `Variant::INT`, `Variant::FLOAT` and other math types.
const HEADER_DATA_FLAG_64: u32 = 1 << 16;
// For `Variant::ARRAY`.
const HEADER_DATA_FIELD_TYPED_ARRAY_MASK: u32 = 0b11 << 16;
// For `Variant::ARRAY`.
const HEADER_DATA_FIELD_TYPED_ARRAY_SHIFT: u32 = 16;
// For `Variant::DICTIONARY`.
const HEADER_DATA_FIELD_TYPED_DICTIONARY_KEY_MASK: u32 = 0b11 << 16;
// For `Variant::DICTIONARY`.
const HEADER_DATA_FIELD_TYPED_DICTIONARY_KEY_SHIFT: u32 = 16;
// For `Variant::DICTIONARY`.
const HEADER_DATA_FIELD_TYPED_DICTIONARY_VALUE_MASK: u32 = 0b11 << 18;
// For `Variant::DICTIONARY`.
const HEADER_DATA_FIELD_TYPED_DICTIONARY_VALUE_SHIFT: u32 = 18;

pub struct DecodingResult<T: ?Sized> {
    pub variant: Box<T>,

    /// How many bytes were consumed, including the header if necessary
    pub consumed: usize,
}

pub fn decode_and_decompress_variant(
    raw_bytes: &[u8],
) -> Result<DecodingResult<dyn Variant>, String> {
    // Replicated from decode_and_decompress_variant in multiplayer_api.cpp

    if raw_bytes.len() < 1 {
        return Err("Not Enough Bytes to Decode Compressed Variant".to_string());
    }

    let variant_type = raw_bytes[0] & VARIANT_META_TYPE_MASK;

    match variant_type {
        // BOOL
        1 => {
            return Bool::decode_compressed(&raw_bytes[0..]);
        }
        // INT
        2 => {
            return Int::decode_compressed(&raw_bytes[0..]);
        }
        _ => {}
    }

    return decode_variant(raw_bytes);
}

pub fn decode_variant(raw_bytes: &[u8]) -> Result<DecodingResult<dyn Variant>, String> {
    // Replicated from decode_variant in marshalls.cpp

    if raw_bytes.len() < 4 {
        return Err("Not Enough Bytes to Decode Variant".to_string());
    }

    let header: u32 = u32::from_le_bytes([raw_bytes[0], raw_bytes[1], raw_bytes[2], raw_bytes[3]]);

    let decoding_result: DecodingResult<dyn Variant>;

    // TODO: Complete All Variant Types
    // Pulled from Variant in variant.h
    match header & HEADER_TYPE_MASK {
        // NIL
        0 => {
            decoding_result = Nil::decode(header, &raw_bytes[4..])?;
        }
        // BOOL
        1 => {
            decoding_result = Bool::decode(header, &raw_bytes[4..])?;
        }
        // INT
        2 => {
            decoding_result = Int::decode(header, &raw_bytes[4..])?;
        }
        // FLOAT
        3 => {
            decoding_result = Float::decode(header, &raw_bytes[4..])?;
        }
        // STRING
        4 => {
            decoding_result = VariantString::decode(header, &raw_bytes[4..])?;
        }
        // VECTOR2
        5 => {
            decoding_result = Vector2::decode(header, &raw_bytes[4..])?;
        }
        // VECTOR2I
        6 => {
            decoding_result = Vector2I::decode(header, &raw_bytes[4..])?;
        }
        // RECT2
        7 => {
            decoding_result = Rect2::decode(header, &raw_bytes[4..])?;
        }
        // RECT2I
        8 => {
            decoding_result = Rect2I::decode(header, &raw_bytes[4..])?;
        }
        // VECTOR3
        9 => {
            decoding_result = Vector3::decode(header, &raw_bytes[4..])?;
        }
        // VECTOR3I
        10 => {
            decoding_result = Vector3I::decode(header, &raw_bytes[4..])?;
        }
        // TRANSFORM2D
        11 => {
            decoding_result = Transform2D::decode(header, &raw_bytes[4..])?;
        }
        // VECTOR4
        12 => {
            decoding_result = Vector4::decode(header, &raw_bytes[4..])?;
        }
        // VECTOR4I
        13 => {
            decoding_result = Vector4I::decode(header, &raw_bytes[4..])?;
        }
        // PLANE
        14 => {
            decoding_result = Plane::decode(header, &raw_bytes[4..])?;
        }
        // QUATERNION
        15 => {
            decoding_result = Quaternion::decode(header, &raw_bytes[4..])?;
        }
        // AABB
        16 => {
            decoding_result = AABB::decode(header, &raw_bytes[4..])?;
        }
        // BASIS
        17 => {
            decoding_result = Basis::decode(header, &raw_bytes[4..])?;
        }
        // TRANSFORM3D
        18 => {
            decoding_result = Transform3D::decode(header, &raw_bytes[4..])?;
        }
        // PROJECTION
        19 => {
            decoding_result = Projection::decode(header, &raw_bytes[4..])?;
        }
        // COLOR
        20 => {
            decoding_result = Color::decode(header, &raw_bytes[4..])?;
        }
        // STRING_NAME
        21 => {
            decoding_result = StringName::decode(header, &raw_bytes[4..])?;
        }
        // NODE_PATH
        22 => {
            return Err("Decoding Node Path Variants Is Not Supported".to_string());
        }
        // RID
        23 => {
            decoding_result = Rid::decode(header, &raw_bytes[4..])?;
        }
        // OBJECT
        24 => {
            return Err("Decoding Object Variants Is Not Supported.".to_string());
        }
        // CALLABLE
        25 => {
            return Err("Decoding Callable Variants Is Not Supported.".to_string());
        }
        // SIGNAL
        26 => {
            return Err("Decoding Signal Variants Is Not Supported.".to_string());
        }
        // DICTIONARY
        27 => {
            decoding_result = decode_dictionary(header, &raw_bytes[4..])?;
        }
        // ARRAY
        28 => {
            decoding_result = decode_array(header, &raw_bytes[4..])?;
        }
        // PACKED_BYTE_ARRAY
        29 => {
            decoding_result = PackedByteArray::decode(header, &raw_bytes[4..])?;
        }
        // PACKED_INT32_ARRAY
        30 => {
            decoding_result = PackedInt32Array::decode(header, &raw_bytes[4..])?;
        }
        // PACKED_INT64_ARRAY
        31 => {
            decoding_result = PackedInt64Array::decode(header, &raw_bytes[4..])?;
        }
        // PACKED_FLOAT32_ARRAY
        32 => {
            decoding_result = PackedFloat32Array::decode(header, &raw_bytes[4..])?;
        }
        // PACKED_FLOAT64_ARRAY
        33 => {
            decoding_result = PackedFloat64Array::decode(header, &raw_bytes[4..])?;
        }
        // PACKED_STRING_ARRAY
        34 => {
            decoding_result = PackedStringArray::decode(header, &raw_bytes[4..])?;
        }
        // PACKED_VECTOR2_ARRAY
        35 => {
            return Err("Decoding PACKED_VECTOR2_ARRAY Variants Is Not Supported.".to_string()); // TODO: Implement
        }
        // PACKED_VECTOR3_ARRAY
        36 => {
            return Err("Decoding PACKED_VECTOR3_ARRAY Variants Is Not Supported.".to_string()); // TODO: Implement
        }
        // PACKED_COLOR_ARRAY
        37 => {
            return Err("Decoding PACKED_COLOR_ARRAY Variants Is Not Supported.".to_string()); // TODO: Implement
        }
        // PACKED_VECTOR4_ARRAY
        38 => {
            return Err("Decoding PACKED_VECTOR4_ARRAY Variants Is Not Supported.".to_string()); // TODO: Implement
        }
        _ => {
            return Err(
                format! {"Decoding Variant Type {} Not Supported", header & HEADER_TYPE_MASK},
            );
        }
    }

    return Ok(decoding_result);
}

use super::{DecodingResult, Variant, helpers};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Vector4 {
    pub x: helpers::WrappedF64,
    pub y: helpers::WrappedF64,
    pub z: helpers::WrappedF64,
    pub w: helpers::WrappedF64,
}

impl Variant for Vector4 {
    // Replicated from encode_variant in marshalls.cpp
    fn encode(&self) -> Result<Vec<u8>, String> {
        let mut header = 12u32;

        // Replaces Compile Time Behavior of Godot
        if *self.x as f32 as f64 != *self.x
            || *self.y as f32 as f64 != *self.y
            || *self.z as f32 as f64 != *self.z
            || *self.w as f32 as f64 != *self.w
        {
            header |= super::HEADER_DATA_FLAG_64;
        }

        let mut encoded = header.to_le_bytes().to_vec();

        if header & super::HEADER_DATA_FLAG_64 != 0 {
            encoded.extend(self.x.to_le_bytes());
            encoded.extend(self.y.to_le_bytes());
            encoded.extend(self.z.to_le_bytes());
            encoded.extend(self.w.to_le_bytes());
        } else {
            encoded.extend((*self.x as f32).to_le_bytes());
            encoded.extend((*self.y as f32).to_le_bytes());
            encoded.extend((*self.z as f32).to_le_bytes());
            encoded.extend((*self.w as f32).to_le_bytes());
        }

        Ok(encoded)
    }

    // Replicated from decode_variant in marshalls.cpp
    fn decode(header: u32, raw_bytes: &[u8]) -> Result<DecodingResult<dyn Variant>, String>
    where
        Self: Sized,
    {
        let mut vec4 = [helpers::WrappedF64(0.0); 4];
        let mut consumed = 0;

        if (header & super::HEADER_DATA_FLAG_64) != 0 {
            if raw_bytes.len() < 4 * 8 {
                return Err("Not Enough Bytes to Decode 64-bit Vector4 Variant".to_string());
            }

            for i in 0..4 {
                vec4[i] = helpers::parse_f64(&raw_bytes[consumed..(consumed + 8)]).into();
                consumed += 8;
            }
        } else {
            if raw_bytes.len() < 4 * 4 {
                return Err("Not Enough Bytes to Decode 32-bit Vector4 Variant".to_string());
            }

            for i in 0..4 {
                vec4[i] = (helpers::parse_f32(&raw_bytes[consumed..(consumed + 4)]) as f64).into();
                consumed += 4;
            }
        }

        return Ok(DecodingResult {
            consumed: 4 + consumed,
            variant: Box::new(Self {
                x: vec4[0],
                y: vec4[1],
                z: vec4[2],
                w: vec4[3],
            }),
        });
    }
}

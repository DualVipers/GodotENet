use super::{DecodingResult, Variant, helpers};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Rect2 {
    pub pos_x: helpers::WrappedF64,
    pub pos_y: helpers::WrappedF64,
    pub size_x: helpers::WrappedF64,
    pub size_y: helpers::WrappedF64,
}

impl Variant for Rect2 {
    // Replicated from encode_variant in marshalls.cpp
    fn encode(&self) -> Result<Vec<u8>, String> {
        let mut header = 7u32;

        // Replaces Compile Time Behavior of Godot
        if *self.pos_x as f32 as f64 != *self.pos_x
            || *self.pos_y as f32 as f64 != *self.pos_y
            || *self.size_x as f32 as f64 != *self.size_x
            || *self.size_y as f32 as f64 != *self.size_y
        {
            header |= super::HEADER_DATA_FLAG_64;
        }

        let mut encoded = header.to_le_bytes().to_vec();

        if header & super::HEADER_DATA_FLAG_64 != 0 {
            encoded.extend(self.pos_x.to_le_bytes());
            encoded.extend(self.pos_y.to_le_bytes());
            encoded.extend(self.size_x.to_le_bytes());
            encoded.extend(self.size_y.to_le_bytes());
        } else {
            encoded.extend((*self.pos_x as f32).to_le_bytes());
            encoded.extend((*self.pos_y as f32).to_le_bytes());
            encoded.extend((*self.size_x as f32).to_le_bytes());
            encoded.extend((*self.size_y as f32).to_le_bytes());
        }

        Ok(encoded)
    }

    // Replicated from decode_variant in marshalls.cpp
    fn decode(header: u32, raw_bytes: &[u8]) -> Result<DecodingResult<dyn Variant>, String>
    where
        Self: Sized,
    {
        let mut rect2 = [helpers::WrappedF64(0.0); 4];
        let mut consumed = 0;

        if (header & super::HEADER_DATA_FLAG_64) != 0 {
            if raw_bytes.len() < 4 * 8 {
                return Err("Not Enough Bytes to Decode 64-bit Rect2 Variant".to_string());
            }

            for i in 0..4 {
                rect2[i] = helpers::parse_f64(&raw_bytes[consumed..(consumed + 8)]).into();
                consumed += 8;
            }
        } else {
            if raw_bytes.len() < consumed + 4 * 4 {
                return Err("Not Enough Bytes to Decode 32-bit Rect2 Variant".to_string());
            }

            for i in 0..4 {
                rect2[i] = (helpers::parse_f32(&raw_bytes[consumed..(consumed + 4)]) as f64).into();
                consumed += 4;
            }
        }

        return Ok(DecodingResult {
            consumed: 4 + consumed,
            variant: Box::new(Self {
                pos_x: rect2[0],
                pos_y: rect2[1],
                size_x: rect2[2],
                size_y: rect2[3],
            }),
        });
    }
}

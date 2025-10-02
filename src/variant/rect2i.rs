use super::{DecodingResult, Variant, helpers};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Rect2I {
    pub pos_x: i32,
    pub pos_y: i32,
    pub size_x: i32,
    pub size_y: i32,
}

impl Variant for Rect2I {
    // Replicated from encode_variant in marshalls.cpp
    fn encode(&self) -> Result<Vec<u8>, String> {
        let header = 8u32;

        let mut encoded = header.to_le_bytes().to_vec();

        encoded.extend((self.pos_x).to_le_bytes());
        encoded.extend((self.pos_y).to_le_bytes());
        encoded.extend((self.size_x).to_le_bytes());
        encoded.extend((self.size_y).to_le_bytes());

        Ok(encoded)
    }

    // Replicated from decode_variant in marshalls.cpp
    fn decode(_header: u32, raw_bytes: &[u8]) -> Result<DecodingResult<dyn Variant>, String>
    where
        Self: Sized,
    {
        if raw_bytes.len() < 4 * 4 {
            return Err("Not Enough Bytes to Decode Vector2I Variant".to_string());
        }

        let mut rect2i = [0; 4];
        let mut consumed = 0;

        for i in 0..4 {
            rect2i[i] = helpers::parse_i32(&raw_bytes[consumed..(consumed + 4)]);
            consumed += 4;
        }

        return Ok(DecodingResult {
            consumed: 4 + consumed,
            variant: Box::new(Self {
                pos_x: rect2i[0],
                pos_y: rect2i[1],
                size_x: rect2i[2],
                size_y: rect2i[3],
            }),
        });
    }
}

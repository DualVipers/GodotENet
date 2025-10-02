use super::{DecodingResult, Variant, helpers};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Vector3I {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Variant for Vector3I {
    // Replicated from encode_variant in marshalls.cpp
    fn encode(&self) -> Result<Vec<u8>, String> {
        let header = 10u32;

        let mut encoded = header.to_le_bytes().to_vec();

        encoded.extend((self.x).to_le_bytes());
        encoded.extend((self.y).to_le_bytes());
        encoded.extend((self.z).to_le_bytes());

        Ok(encoded)
    }

    // Replicated from decode_variant in marshalls.cpp
    fn decode(_header: u32, raw_bytes: &[u8]) -> Result<DecodingResult<dyn Variant>, String>
    where
        Self: Sized,
    {
        if raw_bytes.len() < 3 * 4 {
            return Err("Not Enough Bytes to Decode Vector3I Variant".to_string());
        }

        let mut vec3i = [0; 3];
        let mut consumed = 0;

        for i in 0..3 {
            vec3i[i] = helpers::parse_i32(&raw_bytes[consumed..(consumed + 4)]);
            consumed += 4;
        }

        return Ok(DecodingResult {
            consumed: 4 + consumed,
            variant: Box::new(Self {
                x: vec3i[0],
                y: vec3i[1],
                z: vec3i[2],
            }),
        });
    }
}

use std::{fmt::Display, str::FromStr};

use anyhow::anyhow;

#[derive(PartialEq, Eq, Debug)]
pub struct ChunkType {
    pub bytes: [u8; 4],
}

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid() && !(self.is_critical() && self.is_safe_to_copy())
    }

    fn is_critical(&self) -> bool {
        self.bytes[0].is_ascii_uppercase()
    }

    fn is_public(&self) -> bool {
        self.bytes[1].is_ascii_uppercase()
    }

    fn is_reserved_bit_valid(&self) -> bool {
        self.bytes[2].is_ascii_uppercase()
    }

    fn is_safe_to_copy(&self) -> bool {
        self.bytes[3].is_ascii_lowercase()
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = anyhow::Error;
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        for item in &value {
            match item {
                b'a'..=b'z' | b'A'..=b'Z' => (),
                _ => return Err(anyhow!("Invalid chunk type")),
            }
        }

        let chunk_type = ChunkType { bytes: value };
        if chunk_type.is_valid() {
            Ok(chunk_type)
        } else {
            Err(anyhow!("Invalid string"))
        }
    }
}

impl FromStr for ChunkType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            Err(anyhow!("Invalid str length"))
        } else {
            for item in s.bytes() {
                match item {
                    b'a'..=b'z' | b'A'..=b'Z' => (),
                    _ => return Err(anyhow!("Invalid chunk type")),
                }
            }

            let s: [u8; 4] = s.as_bytes().try_into()?;
            let chunk_type = ChunkType { bytes: s };

            if chunk_type.is_valid() {
                Ok(chunk_type)
            } else {
                Err(anyhow!("Invalid chunk type"))
            }
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = String::from_utf8(self.bytes.to_vec()).unwrap();
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 84];
        let actual = ChunkType::try_from([82, 117, 83, 84]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 84]).unwrap();
        let actual = ChunkType::from_str("RuST").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruST").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUST").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust");
        assert!(chunk.is_err());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert_eq!(&chunk.to_string(), "RuST");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 84]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuST").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}

use crate::chunk_type::ChunkType;
use anyhow::{Result, anyhow};
use crc::{CRC_32_ISO_HDLC, Crc};
use std::fmt::Display;

pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let png_crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let length = data.len() as u32;
        let value: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect();
        let crc = png_crc.checksum(&value);

        Chunk {
            length,
            chunk_type,
            data,
            crc,
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String> {
        let s = String::from_utf8(self.data.to_vec())?;
        Ok(s)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let length_bytes = self.length().to_be_bytes();
        let chunk_type_bytes = self.chunk_type().bytes();
        let data_bytes = self.data();
        let crc_bytes = self.crc().to_be_bytes();

        let bytes: Vec<u8> = length_bytes
            .iter()
            .chain(chunk_type_bytes.iter())
            .chain(data_bytes.iter())
            .chain(crc_bytes.iter())
            .copied()
            .collect();

        bytes
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = anyhow::Error;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < 12 {
            Err(anyhow!("not a chunk"))
        } else {
            let length = u32::from_be_bytes(value[0..4].try_into()?);

            let expected_chunk_size = (12 + length) as usize;
            if value.len() < expected_chunk_size {
                return Err(anyhow!("unexpected chunk size"));
            }

            let new_value: [u8; 4] = value[4..8].try_into()?;
            let chunk_type = ChunkType::try_from(new_value)?;

            let data_start = 8;
            let data_end = data_start + length as usize;
            let data = value[data_start..data_end].to_vec();

            let crc_start = 8 + length as usize;
            let crc_end = crc_start + 4;
            let crc = u32::from_be_bytes(value[crc_start..crc_end].try_into()?);
            let chunk = Chunk::new(chunk_type, data);

            if chunk.crc() != crc {
                Err(anyhow!("crc is invalid"))
            } else {
                Ok(chunk)
            }
        }
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Length: {}", self.length())?;
        writeln!(f, "Chunk Type: {}", self.chunk_type())?;
        // writeln!(f, "Chunk Data: {:?}", self.data())?;
        writeln!(f, "CRC: {}", self.crc())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}

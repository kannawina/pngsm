use crate::chunk_type::ChunkType;
use crate::Error;

use crc::{Crc, *};
use std::fmt;
use thiserror::Error;

//crc migh be 2187366107
pub struct Chunk {
    chunk_type: ChunkType,
    data: Vec<u8>,
}

impl Chunk {
    pub const CHUNK_METADATA: usize = 12;
    pub const CHUNK_METADATA_LEN: usize = 4;
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        Self { chunk_type, data }
    }

    pub fn length(&self) -> u32 {
        self.data.len() as u32
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn crc(&self) -> u32 {
        let bytes: Vec<u8> = self
            .chunk_type
            .bytes()
            .iter()
            .chain(self.data.iter())
            .copied()
            .collect();

        let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        crc.checksum(&bytes)
    }

    pub fn data_as_string(&self) -> crate::Result<String> {
        self.data.iter().try_fold(String::new(), |mut acc, c| {
            let char = *c as char;
            if char.is_ascii_graphic() || char.is_ascii_whitespace() {
                acc.push(char);
                Ok(acc)
            } else {
                Err("Not a readable char".into())
            }
        })
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.length()
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc().to_be_bytes().iter())
            .copied()
            .collect()
    }
}

#[derive(Debug, Error)]
enum ChunkError {
    #[error("Input value too small: atleast 12 bytes")]
    InputValueTooSmall,
    #[error("Chunk type is invalid: {0}, third character must be an uppercase")]
    InvalidChunkType(String),
    #[error("Crc is invalid, expected: {0}, actual: {1}")]
    InvalidCrc(u32, u32),
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < Chunk::CHUNK_METADATA {
            return Err(Box::new(ChunkError::InputValueTooSmall));
        }

        let (lb, value) = value.split_at(Chunk::CHUNK_METADATA_LEN);
        let length = u32::from_be_bytes(lb.try_into()?);

        let (ctb, value) = value.split_at(Chunk::CHUNK_METADATA_LEN);
        let ctb: [u8; 4] = ctb.try_into()?;
        let chunk_type = ChunkType::try_from(ctb)?;

        if !chunk_type.is_valid() {
            return Err(Box::new(ChunkError::InvalidChunkType(
                chunk_type.to_string(),
            )));
        }

        let (cd, value) = value.split_at(length as usize);
        let chunk_data: Vec<u8> = cd.try_into()?;

        let (crcb, _) = value.split_at(Chunk::CHUNK_METADATA_LEN);
        let crc = u32::from_be_bytes(crcb.try_into()?);

        let new = Self::new(chunk_type, chunk_data);
        if new.crc() != crc {
            return Err(Box::new(ChunkError::InvalidCrc(new.crc(), crc)));
        }

        Ok(new)
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data_as_string().unwrap())
    }
}

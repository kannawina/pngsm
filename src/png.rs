use std::fmt;
use std::str::FromStr;
use thiserror::Error;

use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::Error;

#[derive(Debug, Error)]
enum PngError {
    #[error("No chunk with chunk type of {0}")]
    NoChunkType(String),
    #[error("Header is wrong. Data might be not a PNG")]
    WrongHeader,
}

pub struct Png {
    chunks: Vec<Chunk>,
}

impl Png {
    pub const STANDARD_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
    pub const HEADER_LEN: usize = 8;
    pub fn from_chunks(chunks: Vec<Chunk>) -> Self {
        Self { chunks }
    }
    pub fn append_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }
    pub fn remove_chunk(&mut self, chunk_type: &str) -> crate::Result<Chunk> {
        let ct = ChunkType::from_str(chunk_type)?;
        let position = self.chunks.iter().position(|x| x.chunk_type() == &ct);
        if let Some(index) = position {
            Ok(self.chunks.remove(index))
        } else {
            Err(Box::new(PngError::NoChunkType(chunk_type.to_string())))
        }
    }

    pub fn chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    pub fn chunk_by_type(&self, chunk_type: &str) -> Option<&Chunk> {
        let ct = ChunkType::from_str(chunk_type);
        if let Ok(ref chunk_type) = ct {
            self.chunks.iter().find(|x| x.chunk_type() == chunk_type)
        } else {
            None
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let chunk_bytes = self.chunks.iter().map(|x| x.as_bytes()).flatten();
        Self::STANDARD_HEADER
            .iter()
            .copied()
            .chain(chunk_bytes)
            .collect()
    }
}

impl TryFrom<&[u8]> for Png {
    type Error = Error;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let (png_header, value) = value.split_at(Self::HEADER_LEN);
        if Self::STANDARD_HEADER != png_header {
            return Err(Box::new(PngError::WrongHeader));
        }

        let mut tmp = value;
        let mut chunks = Vec::new();
        while tmp.len() > 0 {
            let chunk_len: [u8; 4] = tmp[..Chunk::CHUNK_METADATA_LEN].try_into()?;
            let chunk_len = u32::from_be_bytes(chunk_len);
            let (chunk_bytes, value) = tmp.split_at(Chunk::CHUNK_METADATA + chunk_len as usize);
            chunks.push(Chunk::try_from(chunk_bytes)?);
            tmp = value;
        }

        Ok(Png::from_chunks(chunks))
    }
}

impl fmt::Display for Png {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = self
            .chunks
            .iter()
            .map(|x| x.data_as_string().unwrap())
            .collect::<String>();
        write!(f, "{}", message)
    }
}

use crate::Error;
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

#[derive(PartialEq, Debug)]
pub struct ChunkType {
    chars: Vec<char>,
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.chars
            .iter()
            .map(|&x| x as u8)
            .enumerate()
            .fold([0u8; 4], |mut acc, (i, x)| {
                acc[i] = x;
                acc
            })
    }

    pub fn is_valid(&self) -> bool {
        self.chars[2].is_uppercase()
    }

    pub fn is_critical(&self) -> bool {
        self.chars[0].is_uppercase()
    }

    pub fn is_public(&self) -> bool {
        self.chars[1].is_uppercase()
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        self.is_valid()
    }

    pub fn is_safe_to_copy(&self) -> bool {
        self.chars[3].is_lowercase()
    }
}

#[derive(Debug, Error)]
pub enum ChunkTypeError {
    #[error("Invalid value: {0}. Value must be Between 65-90 or 97-122")]
    InvalidValue(u8),

    #[error("Invalid Character: {0}. Character must be between A-Z or a-z")]
    InvalidChar(char),

    #[error("Invalid Length: {0}. Length must be atleaset 4 Bytes")]
    InvalidLen(usize),
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let chars =
            value
                .into_iter()
                .try_fold(Vec::with_capacity(4), |mut acc, x| match char::from(x) {
                    'A'..='Z' => {
                        acc.push(x as char);
                        Ok(acc)
                    }
                    'a'..='z' => {
                        acc.push(x as char);
                        Ok(acc)
                    }
                    _ => Err(Box::new(ChunkTypeError::InvalidValue(x))),
                })?;

        Ok(ChunkType { chars })
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.chars.iter().map(|x| *x).collect::<String>())
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(Box::from(ChunkTypeError::InvalidLen(s.len())));
        }

        let chars = s.chars().try_fold(Vec::with_capacity(4), |mut acc, x| {
            if x.is_ascii_alphabetic() {
                acc.push(x);
                Ok(acc)
            } else {
                Err(Box::new(ChunkTypeError::InvalidChar(x)))
            }
        })?;

        Ok(ChunkType { chars })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}

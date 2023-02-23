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

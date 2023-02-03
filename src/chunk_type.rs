use std::convert::TryFrom;
use std::fmt::Display;
use std::str::FromStr;
use std::str::from_utf8;

#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Debug)]
pub struct ChunkType(u8, u8, u8, u8);

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        for i in value.iter(){
            if i.is_ascii_digit(){
                return Err(r#"Digit found in Chunk type"#);
            }
        }

        Ok(ChunkType(value[0], value[1], value[2], value[3]))
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes_array = vec![self.0, self.1, self.2, self.3];
        
        write!(f, "{}", from_utf8(&bytes_array).unwrap())
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 4{
            return Err("Invalid String Size");
        }

        for i in s.chars() {
            if !i.is_alphabetic() {
                return Err("Invalid String Characters");
            }
        }

        let bytes = s.as_bytes();
        Ok(ChunkType(bytes[0], bytes[1], bytes[2], bytes[3]))
    }
}


impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        [self.0, self.1, self.2, self.3]
    }

    pub fn is_valid(&self) -> bool {
        if self.2.is_ascii_lowercase(){
            return false;
        }

        if !self.0.is_ascii_alphabetic() || !self.1.is_ascii_alphabetic() || !self.2.is_ascii_alphabetic() || !self.3.is_ascii_alphabetic(){
            return false;
        }

        true
    }


    pub fn is_critical(&self) -> bool {
        self.0.is_ascii_uppercase()
    }

    pub fn is_public(&self) -> bool {
        self.1.is_ascii_uppercase()
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        self.2.is_ascii_uppercase()
    }

    pub fn is_safe_to_copy(&self) -> bool {
        self.3.is_ascii_lowercase()
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
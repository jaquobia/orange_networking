pub mod network_interface;
pub mod packet;
pub mod network_buffer;

use std::mem::size_of;

pub use orange_networking_derive;
use packet::{PacketParseable, PacketParseError};

// Java types
pub type Byte = i8;
pub type Char = u16;
pub type Short = i16;
pub type Int = i32;
pub type Long = i64;
pub type Float = f32;
pub type Double = f64;
pub type Boolean = bool;

pub type ByteArray = Vec<u8>;
impl PacketParseable for ByteArray {
    fn to_packet_bytes(&self) -> Vec<u8> {
        [
        (self.len() as i32).to_packet_bytes(),
        self.to_vec(),
        ].concat()
    }
    fn from_packet_bytes(bytes: &[u8]) -> Result<(Self, usize), PacketParseError> where Self: Sized {
        let mut consumed = 0usize;
        let vec_size = match i32::from_packet_bytes(bytes) {
            Ok((value, size)) => { consumed += size; value },
            Err(e) => { return Err(e); },
        } as usize;
        let bytes = &bytes[consumed..];
        if bytes.len() < vec_size {
            return Err(PacketParseError::NotEnoughData);
        }
        let data = &bytes[0..vec_size];
        consumed += vec_size;
        Ok((data.to_vec(), consumed ))
    }
}

impl PacketParseable for i64 {
    fn to_packet_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
    fn from_packet_bytes(bytes: &[u8]) -> Result<(Self, usize), packet::PacketParseError> where Self: Sized {
        if bytes.len() >= size_of::<Self>() {
            let value = Self::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]]);
            Ok((value, size_of::<Self>()))
        } else {
            Err(packet::PacketParseError::NotEnoughData)
        }
    }
}
impl PacketParseable for i32 {
    fn to_packet_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
    fn from_packet_bytes(bytes: &[u8]) -> Result<(Self, usize), packet::PacketParseError> where Self: Sized {
        if bytes.len() >= size_of::<Self>() {
            let value = Self::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
            Ok((value, size_of::<Self>()))
        } else {
            Err(packet::PacketParseError::NotEnoughData)
        }
    }
}
impl PacketParseable for i16 {
    fn to_packet_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
    fn from_packet_bytes(bytes: &[u8]) -> Result<(Self, usize), packet::PacketParseError> where Self: Sized {
        if bytes.len() >= size_of::<Self>() {
            let value = Self::from_be_bytes([bytes[0], bytes[1]]);
            Ok((value, size_of::<Self>()))
        } else {
            Err(packet::PacketParseError::NotEnoughData)
        }
    }
}
impl PacketParseable for i8 {
    fn to_packet_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
    fn from_packet_bytes(bytes: &[u8]) -> Result<(Self, usize), packet::PacketParseError> where Self: Sized {
        if bytes.len() >= size_of::<Self>() {
            let value = Self::from_be_bytes([bytes[0]]);
            Ok((value, size_of::<Self>()))
        } else {
            Err(packet::PacketParseError::NotEnoughData)
        }
    }
}
impl PacketParseable for u64 {
    fn to_packet_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
    fn from_packet_bytes(bytes: &[u8]) -> Result<(Self, usize), packet::PacketParseError> where Self: Sized {
        if bytes.len() >= size_of::<Self>() {
            let value = Self::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]]);
            Ok((value, size_of::<Self>()))
        } else {
            Err(packet::PacketParseError::NotEnoughData)
        }
    }
}
impl PacketParseable for u32 {
    fn to_packet_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
    fn from_packet_bytes(bytes: &[u8]) -> Result<(Self, usize), packet::PacketParseError> where Self: Sized {
        if bytes.len() >= size_of::<Self>() {
            let value = Self::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
            Ok((value, size_of::<Self>()))
        } else {
            Err(packet::PacketParseError::NotEnoughData)
        }
    }
}
impl PacketParseable for u16 {
    fn to_packet_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
    fn from_packet_bytes(bytes: &[u8]) -> Result<(Self, usize), packet::PacketParseError> where Self: Sized {
        if bytes.len() >= size_of::<Self>() {
            let value = Self::from_be_bytes([bytes[0], bytes[1]]);
            Ok((value, size_of::<Self>()))
        } else {
            Err(packet::PacketParseError::NotEnoughData)
        }
    }
}
impl PacketParseable for u8 {
    fn to_packet_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
    fn from_packet_bytes(bytes: &[u8]) -> Result<(Self, usize), packet::PacketParseError> where Self: Sized {
        if bytes.len() >= size_of::<Self>() {
            let value = Self::from_be_bytes([bytes[0]]);
            Ok((value, size_of::<Self>()))
        } else {
            Err(packet::PacketParseError::NotEnoughData)
        }
    }
}
impl PacketParseable for f64 {
    fn to_packet_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
    fn from_packet_bytes(bytes: &[u8]) -> Result<(Self, usize), packet::PacketParseError> where Self: Sized {
        if bytes.len() >= size_of::<Self>() {
            let value = Self::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]]);
            Ok((value, size_of::<Self>()))
        } else {
            Err(packet::PacketParseError::NotEnoughData)
        }
    }
}
impl PacketParseable for f32 {
    fn to_packet_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
    fn from_packet_bytes(bytes: &[u8]) -> Result<(Self, usize), packet::PacketParseError> where Self: Sized {
        if bytes.len() >= size_of::<Self>() {
            let value = Self::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
            Ok((value, size_of::<Self>()))
        } else {
            Err(packet::PacketParseError::NotEnoughData)
        }
    }
}
impl PacketParseable for bool {
    fn to_packet_bytes(&self) -> Vec<u8> {
        let value = if *self { 1u8 } else { 0u8 };
        value.to_be_bytes().to_vec()
    }
    fn from_packet_bytes(bytes: &[u8]) -> Result<(Self, usize), packet::PacketParseError> where Self: Sized {
        if bytes.len() >= 1 {
            let value = bytes[0] > 0;
            Ok((value, 1))
        } else {
            Err(packet::PacketParseError::NotEnoughData)
        }
    }
}
impl PacketParseable for String {
    fn to_packet_bytes(&self) -> Vec<u8> {
        let b = self.len() as i16;
        let mut b1 = vec![b.to_packet_bytes()];
        for c in self.encode_utf16().into_iter() {
            b1.push(c.to_packet_bytes());
        }
        b1.concat()
    }
    fn from_packet_bytes(bytes: &[u8]) -> Result<(Self, usize), packet::PacketParseError> where Self: Sized {
        let mut consumed = 0usize;
        let str_size: usize = match i16::from_packet_bytes(bytes) {
            Ok((value, size)) => { consumed += size; value },
            Err(e) => { return Err(e); }
        } as usize;
        let mut str_vec: Vec<u16> = vec![0; str_size];
        for index in 0..str_size {
            let char = match u16::from_packet_bytes(&bytes[consumed..]) {
                Ok((value, size)) => { consumed += size; value },
                Err(e) => { return Err(e); },
            };
            str_vec[index] = char;
        }
        let str = String::from_utf16_lossy(&str_vec);
        println!("Read string: {}", str);
        Ok((str, consumed))
    }
}

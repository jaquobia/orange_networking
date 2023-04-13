// use std::io::{Cursor, Read};
//
// use byteorder::{NetworkEndian, WriteBytesExt, ReadBytesExt};
//
// use crate::{ByteArray, UnsizedByteArray};
//
// pub struct NetworkBuffer {
//     pub internal_buffer: Vec<u8>,
// }
//
// impl NetworkBuffer {
//
//     pub fn new() -> Self {
//         Self {
//             internal_buffer: Vec::with_capacity(32),
//         }
//     }
//     pub fn get_bytes(&self) -> &Vec<u8> {
//         &self.internal_buffer
//     }
//     pub fn into_reader(self) -> NetworkBufferReader {
//         NetworkBufferReader::new(self.internal_buffer)
//     }
//
//     pub fn write_i64(&mut self, i: i64) { 
//         self.internal_buffer.write_i64::<NetworkEndian>(i);
//     }
//     pub fn write_u64(&mut self, i: u64) {
//         self.internal_buffer.write_u64::<NetworkEndian>(i);
//     }
//     pub fn write_i32(&mut self, i: i32) {
//         self.internal_buffer.write_i32::<NetworkEndian>(i);
//     }
//     pub fn write_u32(&mut self, i: u32) {
//         self.internal_buffer.write_u32::<NetworkEndian>(i);
//     }
//     pub fn write_i16(&mut self, i: i16) {
//         self.internal_buffer.write_i16::<NetworkEndian>(i);
//     }
//     pub fn write_u16(&mut self, i: u16) {
//         self.internal_buffer.write_u16::<NetworkEndian>(i);
//     }
//     pub fn write_i8(&mut self, i: i8) {
//         self.internal_buffer.write_i8(i);
//     }
//     pub fn write_u8(&mut self, i: u8) {
//         self.internal_buffer.write_u8(i);
//     }
//     pub fn write_bool(&mut self, i: bool) {
//         self.internal_buffer.write_u8(if i { 1u8 } else { 0u8 });
//     }
//     pub fn write_f64(&mut self, f: f64) {
//         self.internal_buffer.write_f64::<NetworkEndian>(f);
//     }
//     pub fn write_f32(&mut self, f: f32) {
//         self.internal_buffer.write_f32::<NetworkEndian>(f);
//     }
//     pub fn write_string(&mut self, str: &String) {
//         let length: u16 = str.len().try_into().unwrap();
//         self.write_u16(length);
//         for c in str.encode_utf16() {
//             self.write_u16(c);
//         }
//     }
//     pub fn write_bytes_sized(&mut self, bytes: &ByteArray) {
//         let length: i32 = bytes.len().try_into().unwrap();
//         self.write_i32(length);
//         self.internal_buffer.extend(bytes);
//     }
//     pub fn write_bytes_unsized(&mut self, bytes: &UnsizedByteArray) {
//         self.internal_buffer.extend(bytes);
//     }
// }
//
// pub struct NetworkBufferReader {
//     internal_buffer: Cursor<Vec<u8>>,
// }
//
// impl NetworkBufferReader {
//     pub fn new(buffer: Vec<u8>) -> Self {
//         Self {
//             internal_buffer: Cursor::new(buffer),
//         }
//     }
//
//     pub fn read_i64(&mut self) -> i64 {
//         self.internal_buffer.read_i64::<NetworkEndian>().expect("Couldn't read i64 from buffer!")
//     }
//     pub fn read_u64(&mut self) -> u64 {
//         self.internal_buffer.read_u64::<NetworkEndian>().expect("Couldn't read u64 from buffer!")
//     }
//     pub fn read_i32(&mut self) -> i32 {
//         self.internal_buffer.read_i32::<NetworkEndian>().expect("Couldn't read i32 from buffer!")
//     }
//     pub fn read_u32(&mut self) -> u32 {
//         self.internal_buffer.read_u32::<NetworkEndian>().expect("Couldn't read u32 from buffer!")
//     }
//     pub fn read_i16(&mut self) -> i16 {
//         self.internal_buffer.read_i16::<NetworkEndian>().expect("Couldn't read i16 from buffer!")
//     }
//     pub fn read_u16(&mut self) -> u16 {
//         self.internal_buffer.read_u16::<NetworkEndian>().expect("Couldn't read u16 from buffer!")
//     }
//     pub fn read_i8(&mut self) -> i8 {
//         self.internal_buffer.read_i8().expect("Couldn't read i8 from buffer!")
//     }
//     pub fn read_u8(&mut self) -> u8 {
//         self.internal_buffer.read_u8().expect("Couldn't read u8 from buffer!")
//     }
//     pub fn read_bool(&mut self) -> bool {
//         self.internal_buffer.read_u8().expect("Couldn't read bool from buffer!") > 0u8
//     }
//     pub fn read_f64(&mut self) -> f64 {
//         self.internal_buffer.read_f64::<NetworkEndian>().expect("Couldn't read f64 from buffer!")
//     }
//     pub fn read_f32(&mut self) -> f32 {
//         self.internal_buffer.read_f32::<NetworkEndian>().expect("Couldn't read f32 from buffer!")
//     }
//     pub fn read_string(&mut self) -> String {
//         let str_len = self.read_u16();
//         let mut string_buffer: Vec<u16> = Vec::with_capacity((str_len * 2).into());
//
//         for i in 0..str_len {
//             let value = self.internal_buffer.read_u16::<NetworkEndian>().expect(format!("Couldn't read String from buffer! Expected {str_len} characters, found {i}!").as_str());
//             string_buffer.push(value);
//         }
//         String::from_utf16(string_buffer.as_slice()).expect("Couldn't create String from buffer data!")
//     }
//     pub fn read_bytes_sized(&mut self) -> ByteArray {
//         let length: usize = self.read_i32().try_into().unwrap();
//         let mut bytes = vec![0; length as usize];
//         self.internal_buffer.read(&mut bytes).expect("Couldn't read a sized byte array");
//         bytes
//     }
//     pub fn read_bytes_unsized(&mut self, num_bytes: usize) -> UnsizedByteArray {
//         let length: usize = num_bytes - self.internal_buffer.position() as usize;
//         let mut bytes = vec![0; length];
//         self.internal_buffer.read(&mut bytes).expect("Couldn't read an unsized byte array");
//         bytes
//     }
// }

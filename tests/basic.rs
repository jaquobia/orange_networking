#[cfg(test)]
mod tests {
    use orange_networking::packet::{PacketEnumHolder, PacketParseable, PacketParseError};
    use orange_networking_derive::PacketEnumHolder;

    #[repr(u8)]
    #[derive(Debug, Clone, PacketEnumHolder)]
    enum TestPackets {
        Packet1 = 0,
        Packet2 { data: u8, name: String, d: f64 } = 2,
        PacketLogin { protocol: i32 , username: String, seed: i64, dimension: i8 } = 3,
        Packet4 { data: u8, data2: i64, data3: String } = 4u8,
    } 
 
    // #[test]
    // fn it_works() {
    //     let mut buffer_write = NetworkBuffer::new();
    //
    //     let a = TestPackets::Packet1 { data: 78 };
    //     let a2 = TestPackets::Packet2 { data: 28, name: "Joe".to_string(), d: 99.22 };
    //     TestPackets::packet_to_bytes(a, &mut buffer_write);
    //     TestPackets::packet_to_bytes(a2, &mut buffer_write);
    //
    //     let buffer = buffer_write.get_bytes();
    //     println!("Bytes: {:?}", buffer);
    //     let mut buffer_reader = NetworkBufferReader::new(buffer.to_vec());
    //     let num_bytes = buffer.len();
    //     let b = TestPackets::bytes_to_packet(&mut buffer_reader, num_bytes);
    //     assert!(if let Ok(TestPackets::Packet1 { data }) = b { data == 78 } else { false });
    //
    //     let b2 = TestPackets::bytes_to_packet(&mut buffer_reader, num_bytes);
    //     assert!(if let Ok(TestPackets::Packet2 { data, name, d }) = b2 { data == 28 && "Joe" == name && d == 99.22} else {false});
    // } 
    //
    // #[test]
    // fn test_arrays() {
    //     let mut buffer_write = NetworkBuffer::new();
    //
    //     let a = TestPackets::PacketLogin { protocol: 0, username: "".to_string(), seed: 0, dimension: 8, bytes: vec![], leftovers: vec![] };
    //     TestPackets::packet_to_bytes(a, &mut buffer_write);
    //
    //     let buffer = buffer_write.get_bytes();
    //     let mut buffer_reader = NetworkBufferReader::new(buffer.to_vec());
    //     let num_bytes = 1 + 4 + (2 + 0) + 8 + 1 + (4 + 0) + (0);
    //     let b = TestPackets::bytes_to_packet(&mut buffer_reader, num_bytes);
    //     if let Ok(TestPackets::PacketLogin { protocol, username, seed, dimension, bytes, leftovers } ) = b {
    //         assert_eq!(bytes.len(), 0); 
    //         assert_eq!(leftovers.len(), 0);
    //     }
    //     else {
    //         assert!(false);
    //     }
    // }

    #[test]
    fn test_packet_parseable() {
        let t1 = 64u8;
        let t2 = 12298i64;
        let t3 = String::from("hhhx6");

        let packet = TestPackets::Packet4 { data: t1, data2: t2, data3: t3.clone() };
        let bytes = TestPackets::packet_to_bytes(packet); 

        println!("Bytes: ({}) {bytes:?}", bytes.len());

        match TestPackets::bytes_to_packet(&bytes) {
            Ok((packet, consumed)) => {
                match packet {
                    TestPackets::Packet4 { data, data2, data3 } => {
                        println!("Consumed: {consumed}");
                        assert_eq!(consumed, bytes.len());
                        assert_eq!(data, t1);
                        assert_eq!(data2, t2);
                        assert_eq!(data3, t3);
                    }, 
                    _ => { eprintln!("Not the expected packet!"); assert!(false); }
                }
            },
            Err(PacketParseError::NotAPacket) => { eprintln!("Not a valid packet!"); assert!(false); },
            Err(PacketParseError::NotEnoughData) => { eprintln!("Not enough bytes!"); assert!(false); },
        }
    }
}

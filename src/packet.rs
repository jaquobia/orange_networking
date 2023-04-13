pub enum PacketParseError {
    NotEnoughData,
    NotAPacket,
}

pub trait PacketEnumHolder {
    fn bytes_to_packet(reader: &[u8]) -> Result<(Self, usize), PacketParseError> where Self: Sized;
    fn packet_to_bytes(packet: Self) -> Vec<u8>;
}

pub trait PacketParseable {
    fn to_packet_bytes(&self) -> Vec<u8>;
    fn from_packet_bytes(bytes: &[u8]) -> Result<(Self, usize), PacketParseError> where Self: Sized;
}

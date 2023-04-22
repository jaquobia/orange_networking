use std::{net::{TcpStream, SocketAddr}, thread::{JoinHandle, self}, sync::{Arc, atomic::AtomicBool, mpsc::{channel, Receiver, Sender}}, io::{Read, Write, self, BufRead}, time::Duration};

use log::warn;
use crate::packet::PacketEnumHolder;

pub struct NetworkThread<G> where G: PacketEnumHolder + Sized + Send {
    read_handle:  Option<JoinHandle<()>>,
    write_handle: Option<JoinHandle<()>>,
    stop: Arc<AtomicBool>,
    recieve_packets: Receiver<G>,
    send_packet: Sender<G>,

}

impl<G: PacketEnumHolder + Sized + Send + 'static> NetworkThread<G> {
    pub fn send_packet(&self, packet: G) {
        match self.send_packet.send(packet) {
            Ok(_) => { },
            Err(_e) => {
                // warn!("{e}");
            },
        };
    }

    pub fn get_packets(&self) -> Vec<G> {
        let mut ret = vec![];
        while let Ok(value) = self.recieve_packets.try_recv() {
           ret.push(value); 
        }
        ret
    }

    pub fn connect_to_server(ip:  String, port: u32) -> Result<NetworkThread<G>, String> {
        let addr_str = format!("{}:{}", ip, port); 
        let stream = TcpStream::connect(addr_str.parse::<SocketAddr>().unwrap());

        match stream {
            Ok(stream) => {
                #[cfg(feature = "no_delay")]
                stream.set_nodelay(true);
                #[cfg(feature = "non_blocking_reads")]
                stream.set_nonblocking(true);

                let stop = Arc::new(AtomicBool::new(false));

                let (tx, recieve_packets) = channel::<G>();
                let (send_packet, rx) = channel::<G>();

                let read_handle = Self::start_read_thread(stream.try_clone().expect("Couldn't clone the tcp stream!?"), stop.clone(), tx);
                let write_handle = Self::start_write_thread(stream, stop.clone(), rx);

                return Ok(NetworkThread { read_handle, write_handle, stop, recieve_packets, send_packet });
            }
            Err(_) => {
                return Err("Couldn't connect to server".to_string());
            }
        }
    }

    fn start_read_thread(socket: TcpStream, stop: Arc<AtomicBool>, outside: Sender<G>) -> Option<JoinHandle<()>> {

        let mut reader = io::BufReader::new(socket);
        let mut partial_packet : Vec<u8> = vec![];
        Some(thread::spawn(move || { 
            while !stop.load(std::sync::atomic::Ordering::Acquire) {
            let bytes: Vec<u8> = reader.fill_buf().expect("Could not fill the buffer?").to_vec();
            let mut do_parsing = true;
            let mut current_point = 0usize;
            while do_parsing {
                // Append the partial packet onto the front of the bytes to complete the packet
                // data
                if partial_packet.len() > 0 {
                    // Disabled due to being unneeded
                    // warn!("Size of partial packet data: id {}, size {}", partial_packet[0], partial_packet.len());
                }
                match G::bytes_to_packet(&[&partial_packet, &bytes[current_point..]].concat()) {
                    Ok((packet_data, consumed)) => {
                        // The partial packet was not part of the reader's data
                        let consumed = consumed - partial_packet.len();
                        current_point += consumed;
                        reader.consume(consumed);
                        // Reset the partial packet so its not continuously building and saving old
                        // data
                        partial_packet = vec![];
                        match outside.send(packet_data) {
                            Ok(_) => {},
                            Err(e) => {
                                warn!("{e}");
                            },
                        };
                    },
                    Err(crate::packet::PacketParseError::NotAPacket) => { warn!("Invalid Packet!! No idea what to do!!"); return; },
                    Err(crate::packet::PacketParseError::NotEnoughData) => { 
                        do_parsing = false; 
                        // Consume the rest of the data and push it onto a separate buffer so
                        // fill_buf can read more data
                        partial_packet.extend_from_slice(&bytes[current_point..]);
                        reader.consume(bytes.len() - current_point);
                        continue; 
                    },
                }
            }
        } 
        }))
    }

    fn start_write_thread(mut socket: TcpStream, stop: Arc<AtomicBool>, outside: Receiver<G>) -> Option<JoinHandle<()>> {
        Some(thread::spawn(move || {

            // let mut buffer_writer = NetworkBuffer::new();
            while !stop.load(std::sync::atomic::Ordering::Acquire) {
                let timeout = Duration::from_secs_f64(0.05);
                match outside.try_recv() {
                    Ok(packet) => {
                        let bytes = G::packet_to_bytes(packet);
                        match socket.write(&bytes) {
                            Ok(bytes_written) => { },
                            Err(e) => {
                                warn!("{e}");
                                stop.store(true, std::sync::atomic::Ordering::Release);
                                socket.shutdown(std::net::Shutdown::Both);
                                return;
                            },
                        };
                        socket.flush().expect("Expected a socket flush, never got one.");
                        // buffer_writer.internal_buffer.clear();
                    },
                    Err(e) => { 
                        match e {
                            std::sync::mpsc::TryRecvError::Disconnected => { 
                                stop.store(true, std::sync::atomic::Ordering::Release);
                                socket.shutdown(std::net::Shutdown::Both);
                                return;
                            },
                            std::sync::mpsc::TryRecvError::Empty => {  },
                        }
                    },
                }
            }
        }))
    }

    pub fn stop(&mut self) {
        self.stop.swap(true, std::sync::atomic::Ordering::Release);

        match self.read_handle.take().expect("Couldn't close thread").join() {
            Ok(_) => {  },
            Err(_) => {  },
        }

        match self.write_handle.take().expect("Couldn't close thread").join() {
            Ok(_) => {  },
            Err(_) => {  },
        }
    }
}

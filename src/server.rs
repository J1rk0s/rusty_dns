use std::net::{self, UdpSocket};
use crate::{models::packets::DnsPacket, DnsHandler};
use std::io::Result;

pub struct DnsServer{
    sock: net::UdpSocket,
    addr: String
}

impl DnsServer {
    pub fn init(addr: &str, port: i32) -> DnsServer {
        let ip = format!("{}:{}", addr, port);
        let socket = UdpSocket::bind(&ip).expect("Failed to create udp socket");

        Self {
            sock: socket,
            addr: ip
        }
    }

    pub fn run(&self) -> Result<()> {
        println!("Server listening on {}", self.addr);
        
        loop {
            let mut buff: [u8; 1024] = [0; 1024];
            let (bytes_written, addr) = self.sock.recv_from(&mut buff)?;

            println!("Received {} bytes from {}", bytes_written, addr.ip());
            let packet: DnsPacket = DnsPacket::parse(&buff);
            let res: DnsPacket = DnsHandler::handle_packet(&packet);
            res.print_data();

            if let Ok(bytes) = res.to_network_bytes() {
                println!("Sending response to {}", addr.ip());
                self.sock.send_to(&bytes, addr)?;
            }
        }
    }
}
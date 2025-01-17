use std::net::{self, UdpSocket};
use crate::models::packets::{DnsAnswer, DnsPacket};
use crate::models::lookup::LOOKUP;

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

    pub fn run(&self) -> () {
        println!("Server listening on {}", self.addr);
        
        loop {
            let mut buff: [u8; 1024] = [0; 1024];
            let (bytes_written, addr) = self.sock.recv_from(&mut buff).expect("Receive error");

            println!("Received {} bytes from {}", bytes_written, addr.ip());
            let packet: DnsPacket = DnsPacket::parse(&buff);
            packet.print_data();
        }
    }

    fn get_answer(&self, packet: &DnsPacket) -> DnsAnswer {
        let ip = LOOKUP.get(packet.question.qname.as_str());

        DnsAnswer::default()
    }
}
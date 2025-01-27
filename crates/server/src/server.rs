use std::net::{self, UdpSocket};
use rusty_dns_resolver::DnsResolver;
use std::io::Result;

pub struct DnsServer{
    sock: net::UdpSocket,
    addr: String
}

impl DnsServer {
    pub fn new(addr: &str, port: u16) -> DnsServer {
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
            match DnsResolver::resolve(&buff) {
                Ok(bytes) => {
                    println!("Sending response to {}\n", addr.ip());
                    self.sock.send_to(&bytes, addr)?;
                }

                Err(e) => {
                    println!("{}", e);
                }
            }
        }
    }
}
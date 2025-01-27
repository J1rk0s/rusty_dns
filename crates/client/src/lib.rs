use std::net::UdpSocket;

pub struct DnsClient {
    sock: UdpSocket,
    dest_addr: String,
}

impl DnsClient {
    pub fn new(destination: String, port: u16) -> Self {
        let s = UdpSocket::bind(format!("{}:{}", destination, port));

        match s {
            Ok(sock) => {
                Self {
                    sock,
                    dest_addr: destination
                }
            }

            Err(_) => {
                panic!("Cannot bind to the specified ip");
            }
        }
    }
}

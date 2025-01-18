use std::net::Ipv4Addr;

use crate::models::{lookup::LOOKUP, packets::DnsPacket};

pub struct DnsHandler {}

impl DnsHandler {
    pub fn handle_packet(packet: &DnsPacket) -> DnsPacket {
        let mut cloned = packet.clone();

        // Header preparation
        cloned.header.ancount = 1;
        cloned.header.flags |= (cloned.header.flags >> 15) & 0b1;
        
        // Answer preparation
        cloned.answer.name = packet.question.qname.clone();
        cloned.answer.type_code = 0x0001;
        cloned.answer.class = 0x0001;
        cloned.answer.ttl = 59;
        cloned.answer.rdlen = 4;
        
        let ip = *LOOKUP.get(packet.question.qname.as_str()).unwrap_or(&"0.0.0.0");
        let octets: Vec<u8> = ip.split(".").map(|x| x.parse().unwrap()).collect();
        cloned.answer.rdata = Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]).octets().into_iter().collect();

        cloned
    }
}
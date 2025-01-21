use std::net::{Ipv4Addr, Ipv6Addr};

use crate::models::{lookup::LOOKUP, packets::DnsPacket};

pub struct DnsHandler {}

impl DnsHandler {

    /// Creates a new packet with the old data and adds the answer section with the requested data
    pub fn handle_packet(packet: &DnsPacket) -> DnsPacket {
        let mut cloned = packet.clone();

        // Header preparation
        cloned.header.ancount = 1;
        cloned.header.flags |= 0b10000000_00000000;
        
        // Answer preparation
        cloned.answer.name = packet.question.qname.clone();

        match cloned.question.qtype {
            1 => {
                cloned.answer.type_code = 0x0001;
                cloned.answer.class = 0x0001;
                cloned.answer.ttl = 59;
                cloned.answer.rdlen = 4;
                
                let ip = *LOOKUP.get(packet.question.qname.as_str()).unwrap_or(&("0.0.0.0", "0.0.0.0"));
                cloned.answer.rdata = ip.0.parse::<Ipv4Addr>().unwrap().octets().into_iter().collect();
            },

            28 => {
                cloned.answer.type_code = 28;
                cloned.answer.class = 0x0001;
                cloned.answer.ttl = 59;
                cloned.answer.rdlen = 16;

                let ip = *LOOKUP.get(packet.question.qname.as_str()).unwrap_or(&("0.0.0.0", "0.0.0.0"));
                cloned.answer.rdata = ip.1.parse::<Ipv6Addr>().unwrap().octets().into_iter().collect();
            }

            _ => {
                cloned.header.flags |= 1;
            }
        }

        cloned
    }
}
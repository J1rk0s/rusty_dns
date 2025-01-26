use std::net::{Ipv4Addr, Ipv6Addr};

use crate::{models::{lookup::LOOKUP, packets::{DnsAnswer, DnsPacket}}, string_utils::str_dns_bytes};

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
            1 => { // A
                DnsHandler::handle_ipv4(&mut cloned);
            },

            12 => { // PTR
                DnsHandler::handle_ptr(&mut cloned);
            }

            28 => { // AAAA
                DnsHandler::handle_ipv6(&mut cloned);
            },

            _ => {
                cloned.header.flags |= 4;
                cloned.header.ancount = 0;
                cloned.answer = DnsAnswer::default();
            }
        }

        cloned
    }

    fn handle_ipv4(packet: &mut DnsPacket) {
        packet.answer.type_code = 1;
        packet.answer.class = 1;
        packet.answer.ttl = 59;
        packet.answer.rdlen = 4;

        match LOOKUP.get(packet.question.qname.as_str()) {
            Some(ip) => {
                packet.answer.rdata = ip.0.parse::<Ipv4Addr>().unwrap().octets().into_iter().collect();
            }

            None => {
                packet.header.flags |= 3;
            }
        }
    }

    fn handle_ipv6(packet: &mut DnsPacket) {
        packet.answer.type_code = 28;
        packet.answer.class = 1;
        packet.answer.ttl = 59;
        packet.answer.rdlen = 16;

        match LOOKUP.get(packet.question.qname.as_str()) {
            Some(ip) => {
                packet.answer.rdata = ip.1.parse::<Ipv6Addr>().unwrap().octets().into_iter().collect();
            }

            None => {
                packet.header.flags |= 3;
            }
        }
    }

    fn handle_ptr(packet: &mut DnsPacket) {
        packet.answer.type_code = 12;
        packet.answer.class = 1;
        packet.answer.ttl = 59;
        
        let server_name = "rustydns.local";

        packet.answer.rdata = str_dns_bytes(&server_name).unwrap();
        packet.answer.rdlen = packet.answer.rdata.len() as u16;
    }
}
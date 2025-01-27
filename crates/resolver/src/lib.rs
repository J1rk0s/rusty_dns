use std::net::{Ipv4Addr, Ipv6Addr};

use rusty_dns_errors::protocol_errors::ProtocolError;
use rusty_dns_protocols::*;
use rusty_dns_protocols::models::lookup::LOOKUP;
use rusty_dns_utils::string_utils::str_dns_bytes;

pub struct DnsResolver {}

impl DnsResolver {

    /// Parses a packet from given buffer, resolves the packet and returns the packet as bytes 
    pub fn resolve(buff: &[u8]) -> Result<Vec<u8>, ProtocolError> {
        let p = DnsPacket::parse(buff)?;

        let resolved = DnsResolver::resolve_packet(&p);
        resolved.print_data();
        resolved.to_network_bytes().map_err(|_| ProtocolError::BytesConversionError)
    }

    /// Creates a new packet with the old data and adds the answer section with the requested data
    pub fn resolve_packet(packet: &DnsPacket) -> DnsPacket {
        let mut cloned = packet.clone();

        // Header preparation
        cloned.header.ancount = 1;
        cloned.header.flags |= 0b10000000_00000000;
        
        // Answer preparation
        cloned.answer.name = packet.question.qname.clone();

        match cloned.question.qtype {
            1 => { // A
                DnsResolver::resolve_ipv4(&mut cloned);
            },

            12 => { // PTR
                DnsResolver::resolve_ptr(&mut cloned);
            }

            28 => { // AAAA
                DnsResolver::resolve_ipv6(&mut cloned);
            },

            _ => { // Other
                cloned.header.flags |= 4;
                cloned.header.ancount = 0;
                cloned.answer = DnsAnswer::default();
            }
        }

        cloned
    }

    /// Resolves IPv4 for the packet
    fn resolve_ipv4(packet: &mut DnsPacket) {
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

    /// Resolves IPv6 for the packet
    fn resolve_ipv6(packet: &mut DnsPacket) {
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

    /// Resolves server name for the packet
    fn resolve_ptr(packet: &mut DnsPacket) {
        packet.answer.type_code = 12;
        packet.answer.class = 1;
        packet.answer.ttl = 256;
        
        let server_name = "rustydns.local";

        packet.answer.rdata = str_dns_bytes(&server_name).unwrap();
        packet.answer.rdlen = packet.answer.rdata.len() as u16;
    }
}

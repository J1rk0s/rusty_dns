use std::io::{self, Write};
use byteorder::{BigEndian, WriteBytesExt};

use rusty_dns_utils::{number_utils::*, string_utils::*};
use rusty_dns_errors::protocol_errors::ProtocolError;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DnsHeader {
    pub id: u16,
    pub flags: u16,
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16
}

#[repr(C)]
#[derive(Default, Clone)]
pub struct DnsQuestion {
    pub qname: String,
    pub qtype: u16,
    pub qclass: u16
}

#[repr(C)]
#[derive(Default, Clone)]
pub struct DnsAnswer {
    pub name: String,
    pub type_code: u16,
    pub class: u16,
    pub ttl: u32,
    pub rdlen: u16,
    pub rdata: Vec<u8>
}

#[repr(C)]
#[derive(Default, Clone)]
pub struct DnsPacket {
    pub header: DnsHeader,
    pub question: DnsQuestion,
    pub answer: DnsAnswer
}

impl DnsHeader {
    pub fn qr(&self) -> u16 {
        (self.flags >> 15) & 1
    }

    pub fn opcode(&self) -> u16 {
        (self.flags >> 11) & 0xf
    }

    pub fn aa(&self) -> u16 {
        (self.flags >> 10) & 1
    }

    pub fn tc(&self) -> u16 {
        (self.flags >> 9) & 1
    }

    pub fn rd(&self) -> u16 {
        (self.flags >> 8) & 1
    }

    pub fn ra(&self) -> u16 {
        (self.flags >> 7) & 1
    }

    pub fn z(&self) -> u16 {
        (self.flags >> 4) & 7
    }

    pub fn rcode(&self) -> u16 {
        self.flags & 0xf
    }
}

impl DnsPacket {
    /// Converts u8 buffer to a DnsPacket
    pub fn parse(data: &[u8]) -> Result<DnsPacket, ProtocolError> {
        fn parse_be_u16(slice: Option<&[u8]>) -> Result<u16, ProtocolError> {
            Ok(
                u16::from_be_bytes(
                    slice.ok_or(ProtocolError::ParseError)?
                    .try_into()
                    .map_err(|_| ProtocolError::ParseError)?
                )
            )
        }

        // Parsing network byte order to machine endian
        let id = parse_be_u16(data.get(0..2))?;
        let flags = parse_be_u16(data.get(2..4))?;
        let qdcount = parse_be_u16(data.get(4..6))?;
        let ancount = parse_be_u16(data.get(6..8))?;
        let nscount = parse_be_u16(data.get(8..10))?;
        let arcount = parse_be_u16(data.get(10..12))?;

        let header = DnsHeader {
            id,
            flags,
            qdcount,
            ancount,
            nscount,
            arcount,
        };

        // Name extraction
        let mut name = "".to_owned();
        let mut idx = 12;
        while data[idx] != 0 {
            let len = *data.get(idx).ok_or(ProtocolError::ParseError)?;

            idx += 1;

            for _ in 0..len {
                let c = *data.get(idx).ok_or(ProtocolError::ParseError)?;
                name.push(c as char);
                idx += 1;
            }

            name.push('.');
        }

        name = name.strip_suffix(".").unwrap().to_string();
        name = name.as_str().to_ascii_lowercase();

        let qtype = parse_be_u16(data.get(idx + 1..idx + 3))?;
        let qclass = parse_be_u16(data.get(idx + 3..idx + 5))?;

        let question = DnsQuestion {
            qname: name,
            qtype,
            qclass
        };

        Ok(
            DnsPacket {
                header,
                question,
                answer: DnsAnswer::default()
            }
        )
    }

    /// Prints the DNS header flags
    fn print_flags(&self) {        
        println!("  DNS flags ({}):", self.header.flags);
        println!("      QR: {}", self.header.qr());
        println!("      OPCODE: {}", self.header.opcode());
        println!("      AA: {}", self.header.aa());
        println!("      TC: {}", self.header.tc());
        println!("      RD: {}", self.header.rd());
        println!("      RA: {}", self.header.ra());
        println!("      Z: {}", self.header.z());
        println!("      RCODE: {}", self.header.rcode());
    }

    /// Prints the DNS header and question to the console
    pub fn print_data(&self) {
        println!("Response packet data:");
        println!("DNS header");
        println!("  ID: {}", self.header.id);
        self.print_flags();
        println!("  QDCOUNT: {}", self.header.qdcount);
        println!("  ANCOUNT: {}", self.header.ancount);
        println!("  NSCOUNT: {}", self.header.nscount);
        println!("  ARCOUNT: {}", self.header.arcount); 
        println!("DNS question");
        println!("  QNAME: {}", self.question.qname);
        println!("  QTYPE: {}", self.question.qtype);
        println!("  QCLASS: {}", self.question.qclass);

        if self.header.ancount != 0 || self.header.rcode() == 0 {
            println!("DNS answer");
            println!("  NAME: {}", self.answer.name);
            println!("  TYPE: {}", self.answer.type_code);
            println!("  CLASS: {}", self.answer.class);
            println!("  TTL: {}", self.answer.ttl);
            println!("  RDLEN: {}", self.answer.rdlen);
            println!("  RDATA: {:?}", self.answer.rdata);
        }

        println!();
    }

    /// Transforms the packet to an array of bytes in network byte order
    pub fn to_network_bytes(&self) -> io::Result<Vec<u8>> {
        let mut buff: Vec<u8> = vec![];

        // Header conversion
        buff.write_u16::<BigEndian>(self.header.id)?;

        buff.write_u16::<BigEndian>(self.header.flags)?;
        buff.write_u16::<BigEndian>(self.header.qdcount)?;
        buff.write_u16::<BigEndian>(self.header.ancount)?;
        buff.write_u16::<BigEndian>(self.header.nscount)?;
        buff.write_u16::<BigEndian>(self.header.arcount)?;

        // Question conversion
        let mut qname: Vec<u8> = str_dns_bytes(&self.question.qname)?;
        buff.append(&mut qname);
        buff.write_all(&u16_to_bytes(self.question.qtype))?;
        buff.write_all(&u16_to_bytes(self.question.qclass))?;

        // Answer conversion
        if self.header.ancount > 0 {
            let mut aname: Vec<u8> = str_dns_bytes(&self.answer.name)?;
            buff.append(&mut aname);
            buff.write_all(&u16_to_bytes(self.answer.type_code))?;
            buff.write_all(&u16_to_bytes(self.answer.class))?;
            buff.write_all(&u32_to_bytes(self.answer.ttl))?;
            buff.write_all(&u16_to_bytes(self.answer.rdlen))?;

            let mut octets: Vec<u8> = self.answer.rdata.clone();
            buff.append(&mut octets);
        }

        Ok(buff)
    }
    
}
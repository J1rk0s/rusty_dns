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
    pub ttl: u64,
    pub rdlen: u32,
    pub rdata: Vec<u8>
}

#[derive(Default, Clone)]
pub struct DnsPacket {
    pub header: DnsHeader,
    pub question: DnsQuestion,
    pub answer: DnsAnswer
}

impl DnsPacket {

    /// Converts u8 buffer to a DnsPacket
    pub fn parse(data: &[u8]) -> DnsPacket {
        assert!(data.len() > 12, "Data must always contain header");

        let id = u16::from_be_bytes(data[0..2].try_into().unwrap());
        let flags = u16::from_be_bytes(data[2..4].try_into().unwrap());
        let qdcount = u16::from_be_bytes(data[4..6].try_into().unwrap());
        let ancount = u16::from_be_bytes(data[6..8].try_into().unwrap());
        let nscount = u16::from_be_bytes(data[8..10].try_into().unwrap());
        let arcount = u16::from_be_bytes(data[10..12].try_into().unwrap());

        let header = DnsHeader {
            id,
            flags,
            qdcount,
            ancount,
            nscount,
            arcount,
        };

        let mut name = "".to_owned();
        let mut idx = 12;
        while data[idx] != 0 {
            let len = data[idx];

            idx += 1;

            for _ in 0..len {
                name.push(data[idx] as char);
                idx += 1;
            }

            name.push('.');
        }

        name = name.strip_suffix(".").unwrap().to_string();

        let qtype = u16::from_be_bytes(data[idx + 1..idx + 3].try_into().unwrap());
        let qclass = u16::from_be_bytes(data[idx + 3..idx + 5].try_into().unwrap());

        let question = DnsQuestion {
            qname: name,
            qtype,
            qclass
        };

        DnsPacket {
            header,
            question,
            answer: DnsAnswer::default()
        }
    }

    /// Prints the DNS header flags
    pub fn print_flags(&self) -> () {
        let qr = (self.header.flags >> 15) & 0b1;
        let opcode = (self.header.flags >> 11) & 0b1111;
        let aa = (self.header.flags >> 10) & 0b1;
        let tc = (self.header.flags >> 9) & 0b1;
        let rd = (self.header.flags >> 8) & 0b1;
        let ra = (self.header.flags >> 7) & 0b1;
        let z = (self.header.flags >> 4) & 0b111;
        let rcode = self.header.flags & 0b1111;
        
        println!("  DNS flags:");
        println!("      QR: {}", qr);
        println!("      OPCODE: {}", opcode);
        println!("      AA: {}", aa);
        println!("      TC: {}", tc);
        println!("      RD: {}", rd);
        println!("      RA: {}", ra);
        println!("      Z: {}", z);
        println!("      RCODE: {}", rcode);
    }

    /// Prints the DNS header and question to the console
    pub fn print_data(&self) {
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
        println!();
        println!();
    }
    
}
use crate::{models::dns_flags::DnsHeaderFlag, DnsHeader};

pub struct DnsHeaderBuilder {
    header: DnsHeader
}

impl DnsHeaderBuilder {
    pub fn new() -> Self {
        Self {
            header: DnsHeader::default()
        }
    }

    pub fn set_id(mut self, id: u16) -> Self {
        self.header.id = id;

        self
    }

    pub fn set_flag(mut self, flag: DnsHeaderFlag) -> Self {
        self.header.flags |= flag.to_bit();

        self
    }

    pub fn set_qdcount(mut self, count: u16) -> Self {
        self.header.qdcount = count;

        self
    }

    pub fn build(self) -> DnsHeader {
        self.header
    }
}
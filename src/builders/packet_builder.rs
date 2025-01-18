use crate::models::{dns_flags::DnsHeaderFlag, packets::{DnsAnswer, DnsHeader, DnsPacket, DnsQuestion}};

pub struct DnsPacketBuilder {
    packet: DnsPacket
}

impl DnsPacketBuilder {
    pub fn init() -> DnsPacketBuilder {
        Self {
            packet: DnsPacket::default()
        }
    }

    pub fn build(self) -> DnsPacket {
        self.packet.clone()
    }

    pub fn set_header(mut self, header: DnsHeader) -> Self {
        self.packet.header = header;

        self
    }

    pub fn set_question(mut self, question: DnsQuestion) -> Self {
        self.packet.question = question;

        self
    }

    pub fn set_answer(mut self, answer: DnsAnswer) -> Self {
        self.packet.answer = answer;

        self
    }

    pub fn set_header_id(mut self, id: u16) -> Self {
        self.packet.header.id = id;

        self
    }

    pub fn set_header_flag(mut self, flag: DnsHeaderFlag) -> Self {
        self.packet.header.flags |= flag.to_bit();

        self
    }
}
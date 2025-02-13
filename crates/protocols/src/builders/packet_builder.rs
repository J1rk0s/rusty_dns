use crate::{DnsHeader, DnsPacket, DnsQuestion};

#[derive(Default)]
pub struct DnsPacketBuilder {
    packet: DnsPacket
}

impl DnsPacketBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_header(mut self, header: DnsHeader) -> Self {
        self.packet.header = header;

        self
    }

    pub fn set_question(mut self, question: DnsQuestion) -> Self {
        self.packet.question = question;

        self
    }


    pub fn build(self) -> DnsPacket {
        self.packet
    }
}
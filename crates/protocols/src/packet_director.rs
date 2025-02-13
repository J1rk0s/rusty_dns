use crate::{builders::{DnsHeaderBuilder, DnsPacketBuilder, DnsQuestionBuilder}, models::dns_flags::{DnsHeaderFlag, DnsQuestionClass, DnsQuestionTypes}, DnsPacket};

pub struct PacketDirector {}

impl PacketDirector {
    pub fn create_basic_packet(id: u16, hostname: String, qtype: DnsQuestionTypes) -> DnsPacket {
        let header = DnsHeaderBuilder::new()
            .set_id(id)
            .set_qdcount(1)
            .set_flag(DnsHeaderFlag::QR(false))
            .set_flag(DnsHeaderFlag::RD(true))
            .build();

        let question = DnsQuestionBuilder::new()
            .set_name(hostname)
            .set_qtype(qtype)
            .seq_qclass(DnsQuestionClass::Internet)
            .build();

        let packet = DnsPacketBuilder::new()
            .set_header(header)
            .set_question(question)
            .build();

        packet
    }
}
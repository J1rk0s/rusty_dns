#[cfg(test)]
mod tests {
    use rusty_dns::models::packets::DnsPacket;

    #[test]
    fn test_parsing(){
        let data: [u8; 33] = [0xca, 0x38, 0x1, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x3, 0x77, 0x77, 0x77, 0x7, 0x79, 0x6f, 0x75, 0x74, 0x75, 0x62, 0x65, 0x3, 0x63, 0x6f, 0x6d, 0x0, 0x0, 0x1, 0x0, 0x1];
        let packet = DnsPacket::parse(&data);

        assert_eq!(packet.header.id, 51768);
        assert_eq!(packet.header.flags, 256);
        assert_eq!(packet.header.qdcount, 1);
        assert_eq!(packet.header.ancount, 0);
        assert_eq!(packet.header.nscount, 0);
        assert_eq!(packet.header.arcount, 0);

        assert_eq!(packet.question.qname, "www.youtube.com");
        assert_eq!(packet.question.qclass, 1);
        assert_eq!(packet.question.qtype, 1);
    }
}
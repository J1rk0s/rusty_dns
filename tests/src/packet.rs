#[cfg(test)]
mod packet_tests {

    use rusty_dns_protocols::DnsPacket;

    #[test]
    fn test_valid_packet(){
        let data: [u8; 33] = [
            0xca, // ID
            0x38, // ID
            0x1, // FLAGS
            0x0, // FLAGS
            0x0, // QDCOUNT 
            0x1, // QDCOUNT 
            0x0, // ANCOUNT 
            0x0, // ANCOUNT 
            0x0, // NSCOUNT 
            0x0, // NSCOUNT 
            0x0, // ARCOUNT 
            0x0, // ARCOUNT 
            0x3, // length 3
            0x77, // w
            0x77, // w
            0x77, // w
            0x7, // length 7
            0x79, // y
            0x6f, // o
            0x75, // u
            0x74, // t
            0x75, // u
            0x62, // b
            0x65, // e
            0x3, // length 3
            0x63, // c
            0x6f, // o
            0x6d, // m
            0x0, // NULL
            0x0, // QTYPE IPV4
            0x1, // QTYPE IPV4
            0x0, // QCLASS
            0x1  // QCLASS
        ];

        let parsed = DnsPacket::parse(&data);
        assert!(parsed.is_ok());
        
        let packet = parsed.unwrap();

        assert_eq!(packet.header.id, 51768);
        assert_eq!(packet.header.flags, 256);
        assert_eq!(packet.header.qdcount, 1);
        assert_eq!(packet.header.ancount, 0);
        assert_eq!(packet.header.nscount, 0);
        assert_eq!(packet.header.arcount, 0);

        assert_eq!(packet.question.qname, "www.youtube.com");
        assert_eq!(packet.question.qtype, 1);
        assert_eq!(packet.question.qclass, 1);
    }

    #[test]
    fn test_invalid_packet() {
        let data: [u8; 14] = [
            0xca, // ID
            0x38, // ID
            0x1, // FLAGS
            0x0, // FLAGS
            0x0, // QDCOUNT 
            0x1, // QDCOUNT 
            0x0, // ANCOUNT 
            0x0, // ANCOUNT 
            0x0, // NSCOUNT 
            0x0, // NSCOUNT 
            0x0, // ARCOUNT 
            0x0, // ARCOUNT 
            0x3, // length 3
            0x77, // w
        ];

        let packet = DnsPacket::parse(&data);

        assert!(packet.is_err());
    }
}
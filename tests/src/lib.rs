#[cfg(test)]
mod tests {
    use rusty_dns_server::{DnsHandler, models::DnsPacket};

    #[test]
    fn test_parsing(){
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

        let packet = DnsPacket::parse(&data);

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
    fn test_handle_packet_for_ipv4_with_valid_web(){
        let data: [u8; 28] = [
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
            0x6, // length 6
            0x67, // g
            0x6f, // o
            0x6f, // o
            0x67, // g
            0x6c, // l
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

        let packet = DnsPacket::parse(&data);
        let ans = DnsHandler::handle_packet(&packet);

        assert_eq!(ans.answer.name, "google.com");
        assert_eq!(ans.answer.class, 1);
        assert_eq!(ans.answer.type_code, 1);
        assert_eq!(ans.answer.rdlen, 4);
        assert_eq!(ans.answer.ttl, 59);
        assert_eq!(ans.answer.rdata, vec![0x8e, 0xfb, 0x24, 0x6e])
    }
}
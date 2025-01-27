#[cfg(test)]
mod resolver_tests {
    use rusty_dns_protocols::DnsPacket;
    use rusty_dns_resolver::DnsResolver;

    #[test]
    fn test_valid_ipv4(){
        let data: [u8; 29] = [
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
            0x7, // length 7
            0x65, // e
            0x78, // x
            0x61, // a
            0x6d, // m
            0x70, // p
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

        let packet = DnsPacket::parse(&data).unwrap();
        let ans = DnsResolver::resolve_packet(&packet);

        assert_eq!(ans.answer.name, "example.com");
        assert_eq!(ans.answer.class, 1);
        assert_eq!(ans.answer.type_code, 1);
        assert_eq!(ans.answer.rdlen, 4);
        assert_eq!(ans.answer.rdata, vec![0x17, 0xd7, 0x00, 0x88]);
    }

    #[test]
    fn test_valid_ipv6(){
        let data: [u8; 29] = [
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
            0x7, // length 7
            0x65, // e
            0x78, // x
            0x61, // a
            0x6d, // m
            0x70, // p
            0x6c, // l
            0x65, // e
            0x3, // length 3
            0x63, // c
            0x6f, // o
            0x6d, // m
            0x0, // NULL
            0x0, // QTYPE IPV6
            0x1c, // QTYPE IPV6
            0x0, // QCLASS
            0x1  // QCLASS
        ];

        let packet = DnsPacket::parse(&data).unwrap();
        let ans = DnsResolver::resolve_packet(&packet);

        assert_eq!(ans.answer.name, "example.com");
        assert_eq!(ans.answer.type_code, 28);
        assert_eq!(ans.answer.class, 1);
        assert_eq!(ans.answer.rdlen, 16);
        assert_eq!(ans.answer.rdata, vec![0x26, 0x0, 0x14, 0x6, 0x3a, 0x0, 0x0, 0x21, 0x0, 0x0, 0x0, 0x0, 0x17, 0x3e, 0x2e, 0x65]);
    }

    #[test]
    fn test_invalid_protocol(){
        let data: [u8; 29] = [
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
            0x7, // length 7
            0x65, // e
            0x78, // x
            0x61, // a
            0x6d, // m
            0x70, // p
            0x6c, // l
            0x65, // e
            0x3, // length 3
            0x63, // c
            0x6f, // o
            0x6d, // m
            0x0, // NULL
            0x80, // QTYPE RESERVED
            0x02, // QTYPE RESERVED
            0x0, // QCLASS
            0x1  // QCLASS
        ];

        let packet = DnsPacket::parse(&data).unwrap();
        let ans = DnsResolver::resolve_packet(&packet);

        assert_eq!(ans.question.qname, "example.com");
        assert_eq!(ans.question.qtype, 32770);
        assert_eq!(ans.question.qclass, 1);
        
        assert_eq!(ans.header.rcode(), 4);
        assert_eq!(ans.header.ancount, 0);
    }
}
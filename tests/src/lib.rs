mod packet_tests;

#[cfg(test)]
mod tests {
    use rusty_dns_protocols::DnsPacket;
    use rusty_dns_resolver::DnsResolver;

    #[test]
    fn test_valid_ipv4(){
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

        let packet = DnsPacket::parse(&data).unwrap();
        let ans = DnsResolver::resolve_packet(&packet);

        assert_eq!(ans.answer.name, "google.com");
        assert_eq!(ans.answer.class, 1);
        assert_eq!(ans.answer.type_code, 1);
        assert_eq!(ans.answer.rdlen, 4);
        assert_eq!(ans.answer.ttl, 59);
        assert_eq!(ans.answer.rdata, vec![0x8e, 0xfb, 0x24, 0x6e])
    }
}
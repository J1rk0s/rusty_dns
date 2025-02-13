#[derive(Debug)]
pub enum DnsHeaderFlag {
    QR(bool),
    AA(bool),
    TC(bool),
    RD(bool),
    RA(bool),
}

impl DnsHeaderFlag {
    pub fn to_bit(self) -> u16 {
        match self {
            DnsHeaderFlag::QR(val) => (val as u16) << 15,
            DnsHeaderFlag::AA(val) => (val as u16) << 10,
            DnsHeaderFlag::TC(val) => (val as u16) << 9,
            DnsHeaderFlag::RD(val) => (val as u16) << 8,
            DnsHeaderFlag::RA(val) => (val as u16) << 7,
        }
    }
}

#[derive(Debug)]
pub enum DnsQuestionTypes {
    A = 1,
    PTR = 12,
    HINFO = 13, 
    AAAA = 28,
}

#[derive(Debug)]
pub enum DnsQuestionClass {
    Internet = 1,
    Chaos = 3,
    Hesiod = 4
}
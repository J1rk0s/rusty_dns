use crate::{models::dns_flags::{DnsQuestionClass, DnsQuestionTypes}, DnsQuestion};

pub struct DnsQuestionBuilder {
    question: DnsQuestion
}

impl DnsQuestionBuilder {
    pub fn new() -> Self {
        Self {
            question: DnsQuestion::default()
        }
    }

    pub fn set_name(mut self, name: String) -> Self {
        self.question.qname = name;

        self
    }

    pub fn set_qtype(mut self, qtype: DnsQuestionTypes) -> Self {
        self.question.qtype = qtype as u16;

        self
    }

    pub fn seq_qclass(mut self, qclass: DnsQuestionClass) -> Self {
        self.question.qclass = qclass as u16;

        self
    }

    pub fn build(self) -> DnsQuestion {
        self.question
    }
}
use rusty_dns::server::DnsServer;
use std::io::Result;

fn main() -> Result<()> {
    let dns = DnsServer::init("192.168.1.50", 53);
    dns.run()
}

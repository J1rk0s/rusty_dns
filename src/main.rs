use rusty_dns::server::DnsServer;

fn main() {
    let dns = DnsServer::init("192.168.1.50", 53);
    dns.run();
}

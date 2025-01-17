use server::DnsServer;

mod server;
mod models;

fn main() {
    let dns = DnsServer::init("192.168.1.50", 53);
    dns.run();
}

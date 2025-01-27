use std::io::Result;
use clap::Parser;
use rusty_dns_server::DnsServer;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    ip: String,

    #[arg(short, long, default_value_t = 53)]
    port: u16
}

fn main() -> Result<()> {
    let args = Args::parse();

    let dns = DnsServer::new(&args.ip, args.port);
    dns.run()
}
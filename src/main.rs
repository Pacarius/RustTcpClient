mod client;
use client::make_default_client;
use clap::Parser;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    ip: String,
    #[arg(short, long)]
    port: u16,
}
fn main(){
    let args: Args = Args::parse();
    make_default_client(Some(args.ip),Some(args.port));
}
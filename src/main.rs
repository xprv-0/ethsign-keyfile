use ethsign::{Protected, KeyFile};
use hex::ToHex;
use clap::Parser;
use clap::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    key_file: String,

    #[arg(short, long)]
    password: String,
}

fn main() {
    let args = Args::parse();
    let file = std::fs::File::open(&args.key_file).unwrap();
    let key: KeyFile = serde_json::from_reader(file).unwrap();
    let password: Protected = args.password.into();
    let d = key.crypto.decrypt(&password).unwrap();
    println!("{:?}", &d.encode_hex::<String>());
 }
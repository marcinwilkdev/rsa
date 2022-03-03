use std::path::{Path, PathBuf};

use num_bigint::BigInt;
use structopt::StructOpt;

pub mod public_key;
pub mod private_key;

#[derive(StructOpt, Debug)]
pub struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    pub file: PathBuf,
    #[structopt(short, long)]
    pub message: BigInt,
}

fn get_file_content(file: &Path) -> String {
    std::fs::read_to_string(&file).unwrap_or_else(|e| {
        eprintln!("Couldn't read file: {}", e);
        std::process::exit(1);
    })
}

fn check_rsa(kty: &str) {
    if kty != "RSA" {
        eprintln!("This key is not RSA.");
        std::process::exit(1);
    }
}

fn bigint_from_base_64_url(text: &str) -> BigInt {
    let decoded_text = base64_url::decode(text).unwrap_or_else(|e| {
        eprintln!("Couldn't parse from base64: {}", e);
        std::process::exit(1);
    });

    BigInt::from_bytes_be(num_bigint::Sign::Plus, &decoded_text)
}

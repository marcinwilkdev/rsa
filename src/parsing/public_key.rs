use std::path::Path;

use num_bigint::BigInt;
use serde::Deserialize;

#[derive(Deserialize)]
struct Jwk {
    kty: String,
    n: String,
    e: String,
}

pub struct PublicKey {
    pub n: BigInt,
    pub e: BigInt,
}

pub fn parse_public_jwk_from_file(file: &Path) -> PublicKey {
    let file_content = super::get_file_content(file);

    let Jwk { kty, n, e } = deserialize_jwk(&file_content);

    super::check_rsa(&kty);

    let n = super::bigint_from_base_64_url(&n);
    let e = super::bigint_from_base_64_url(&e);

    PublicKey { n, e }
}

fn deserialize_jwk(file_content: &str) -> Jwk {
    serde_json::from_str(&file_content).unwrap_or_else(|e| {
        eprintln!("Wrong file format: {}", e);
        std::process::exit(1);
    })
}

use std::path::Path;

use num_bigint::BigInt;
use serde::Deserialize;

#[derive(Deserialize)]
struct Jwk {
    kty: String,
    p: String,
    q: String,
    dp: String,
    dq: String,
    qi: String,
}

pub struct PrivateKey {
    pub p: BigInt,
    pub q: BigInt,
    pub dp: BigInt,
    pub dq: BigInt,
    pub qi: BigInt,
}

pub fn parse_private_jwk_from_file(file: &Path) -> PrivateKey {
    let file_content = super::get_file_content(file);

    let Jwk {
        kty,
        p,
        q,
        dp,
        dq,
        qi,
    } = deserialize_jwk(&file_content);

    super::check_rsa(&kty);

    let p = super::bigint_from_base_64_url(&p);
    let q = super::bigint_from_base_64_url(&q);
    let dp = super::bigint_from_base_64_url(&dp);
    let dq = super::bigint_from_base_64_url(&dq);
    let qi = super::bigint_from_base_64_url(&qi);

    PrivateKey { p, q, dp, dq, qi }
}

fn deserialize_jwk(file_content: &str) -> Jwk {
    serde_json::from_str(&file_content).unwrap_or_else(|e| {
        eprintln!("Wrong file format: {}", e);
        std::process::exit(1);
    })
}

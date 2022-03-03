use structopt::StructOpt;

use rsa::PublicKey;
use rsa::Opt;

fn main() {
    let Opt { file, message } = Opt::from_args();

    let PublicKey { n, e } = rsa::parse_public_jwk_from_file(&file);

    let rsa = rsa::rsa(&message, &n, &e);

    println!("{}", rsa);
}

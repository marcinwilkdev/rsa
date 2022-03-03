use structopt::StructOpt;

use rsa::PrivateKey;
use rsa::Opt;

fn main() {
    let Opt { file, message } = Opt::from_args();

    let PrivateKey { p, q, dp, dq, qi } = rsa::parse_private_jwk_from_file(&file);

    let message = rsa::crt(&message, &p, &q, &dp, &dq, &qi);

    println!("{}", message);
}

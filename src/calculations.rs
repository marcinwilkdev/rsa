use num_bigint::BigInt;
use num_traits::identities::Zero;

pub fn rsa(message: &BigInt, n: &BigInt, e: &BigInt) -> BigInt {
    message.modpow(e, n)
}

pub fn crt(
    message: &BigInt,
    p: &BigInt,
    q: &BigInt,
    dp: &BigInt,
    dq: &BigInt,
    qi: &BigInt,
) -> BigInt {
    let m1 = message.modpow(&dp, &p);
    let m2 = message.modpow(&dq, &q);

    let mut h = (qi * (&m1 - &m2)) % p;

    if h < BigInt::zero() {
        h += p;
    }

    &m2 + &h * q
}

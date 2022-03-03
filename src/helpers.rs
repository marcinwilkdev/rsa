use num_bigint::BigInt;
use num_traits::identities::{One, Zero};

pub fn invert_mod_big_int(
    a: &BigInt,
    b: &BigInt,
    s: (&BigInt, &BigInt),
    t: (&BigInt, &BigInt),
) -> BigInt {
    if b == &BigInt::zero() {
        return s.0.clone();
    }

    let q = a / b;
    let next_b = a % b;
    let next_s = s.0 - &q * s.1;
    let next_t = t.1 - &q * t.0;

    invert_mod_big_int(b, &next_b, (s.1, &next_s), (t.1, &next_t))
}

pub fn invert_mod(a: &BigInt, b: &BigInt) -> BigInt {
    let zero = BigInt::zero();
    let one = BigInt::one();

    invert_mod_big_int(a, b, (&one, &zero), (&zero, &one))
}

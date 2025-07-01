use num_bigint::BigInt;

pub fn d(s: &str) -> BigInt {
    return BigInt::parse_bytes(s.as_bytes(), 10).unwrap();
}

pub fn fast_pow_mod_impl(base: BigInt, exp: BigInt, modulus: Option<BigInt>) -> BigInt {
    let bits: u64 = exp.bits();

    let mut x = 0;
    let mut answer = BigInt::from(1u32);
    let mut bit = if exp.bit(x) {1} else {0};
    if bit == 1 {
        answer *= &base;
        if let Some(m) = &modulus {
            answer %= m;
        }
    }

    let mut last = base.clone();
    x += 1;
    while x < bits {
        let mut square = &last * &last;
        if let Some(m) = &modulus {
            square %= m;
        }
        bit = if exp.bit(x) {1} else {0};
        if bit == 1 {
            answer *= &square;
            if let Some(m) = &modulus {
                answer %= m;
            }
        }

        x += 1;
        last = square;
    }

    return answer;
}

pub fn fast_pow(base: BigInt, exp: BigInt) -> BigInt {
    return fast_pow_mod_impl(base, exp, None);
}

pub fn fast_pow_mod(base: BigInt, exp: BigInt, modulus: BigInt) -> BigInt {
    return fast_pow_mod_impl(base, exp, Some(modulus));
}

pub fn gcd(mut a: BigInt, mut b: BigInt) -> BigInt {
    while b != BigInt::ZERO {
        let temp = b.clone();
        b = &a % &b;
        a = temp.clone();
    }
    return a;
}

pub fn mod_inverse_extended(a: BigInt, n: BigInt) -> BigInt {
    let mut t = BigInt::ZERO;
    let mut newt = BigInt::from(1u32);
    let mut r = n.clone();
    let mut newr = a.clone();

    while newr != BigInt::ZERO {
        let quotient = &r / &newr;
        let mut temp = newt.clone();
        newt = &t - (&quotient * &newt);
        t = temp.clone();
        temp = newr.clone();
        newr = &r - (&quotient * &newr);
        r = temp.clone();
    }

    if r > BigInt::from(1u32) {
        panic!("a is not invertible");
    }

    if t < BigInt::ZERO {
        t += &n;
    }
    return t;
}

pub fn mod_inv(a: BigInt, m: BigInt) -> BigInt {
    return fast_pow_mod(a, &m - 2, m);
}

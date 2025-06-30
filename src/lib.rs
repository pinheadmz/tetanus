use num_bigint::BigInt;

pub fn fast_pow_mod_impl(base: BigInt, exp: BigInt, modulus: Option<BigInt>) -> BigInt {
    println!("Exponent {} in binary: {}", exp.to_str_radix(10), exp.to_str_radix(2));
    let bits: u64 = exp.bits();

    println!("# of bits in exponent: {}", bits);

    let mut x = 0;
    let mut answer = BigInt::from(1u32);
    let mut bit = if exp.bit(x) {1} else {0};
    if bit == 1 {
        answer *= &base;
        if let Some(m) = &modulus {
            answer %= m;
        }
    }
    println!("{}\n bit: {}\n answer: {}",
        format!("{}^(2^0)=\n {}", base.to_str_radix(10), base.to_str_radix(10)),
        bit,
        answer.to_str_radix(10));

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

        println!("{}\n bit: {}\n answer: {}",
            format!("{}^(2^{})=\n {}", base.to_str_radix(10), x, square.to_str_radix(10)),
            bit,
            answer.to_str_radix(10));
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
        println!("Remainder of {} / {} is {}", temp.to_str_radix(10), a.to_str_radix(10), b.to_str_radix(10));
        a = temp.clone();
    }
    return a;
}

use num_bigint::BigUint;

pub fn fast_pow(base: BigUint, exp: BigUint) -> BigUint {
    println!("Exponent {} in binary: {}", exp.to_str_radix(10), exp.to_str_radix(2));
    let bits: u64 = exp.bits();

    println!("# of bits in exponent: {}", bits);

    let mut x = 0;
    let mut answer = BigUint::from(1u32);
    let mut bit = if exp.bit(x) {1} else {0};
    if bit == 1 {
        answer *= &base;
    }
    println!("{}\n bit: {}\n answer: {}",
        format!("{}^(2^0)=\n {}", base.to_str_radix(10), base.to_str_radix(10)),
        bit,
        answer.to_str_radix(10));

    let mut last = base.clone();
    x += 1;
    while x < bits {
        let square = &last * &last;
        bit = if exp.bit(x) {1} else {0};
        if bit == 1 {
            answer *= &square;
        }

        println!("{}\n bit: {}\n answer: {}",
            format!("{}^(2^{})=\n {}", base.to_str_radix(10), x, square.to_str_radix(10)),
            bit,
            answer.to_str_radix(10));
        x += 1;
        last = square.clone();
    }

    return answer;
}
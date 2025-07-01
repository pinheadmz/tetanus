use num_bigint::{BigInt, RandBigInt, Sign};
use num_primes::Generator;
use rand::thread_rng;

use crate::math::*;

pub struct EgParams {
    pub p: BigInt,
    pub g: BigInt,
}

impl EgParams {
    pub fn generate(bits: usize) -> Self {
        let big_prime = Generator::new_prime(bits);
        let prime_bytes = big_prime.to_bytes_be();
        return Self{
            p: BigInt::from_bytes_be(Sign::Plus, &prime_bytes),
            g: BigInt::from(2u32)};
    }
}

pub struct EgKeyPair {
    pub private: Option<BigInt>,
    pub public: BigInt,
}

impl EgKeyPair {
    pub fn generate(params: &EgParams) -> Self {
        let mut rng = thread_rng();
        let private = rng.gen_bigint_range(&BigInt::ZERO, &params.p);
        let public = fast_pow_mod(&params.g, &private, &params.p);
        return Self{ private: Some(private), public };
    }

    pub fn from_private(params: &EgParams, private: &BigInt) -> Self {
        let public = fast_pow_mod(&params.g, &private, &params.p);
        return Self{ private: Some(private.clone()), public };
    }

    pub fn from_public(public: BigInt) -> Self {
        return Self{ private: None, public };
    }
}

pub fn eg_encrypt(params: &EgParams, key: &EgKeyPair, msg: &BigInt) -> (BigInt, BigInt) {
    let mut rng = thread_rng();
    let k = rng.gen_bigint_range(&BigInt::ZERO, &params.p);
    return eg_encrypt_with_k(params, key, msg, k);
}

pub fn eg_encrypt_with_k(params: &EgParams, key: &EgKeyPair, msg: &BigInt, k: BigInt) -> (BigInt, BigInt) {
    let c1 = fast_pow_mod(&params.g, &k, &params.p);
    let c2 = msg * fast_pow_mod(&key.public, &k, &params.p) % &params.p;
    return (c1, c2);
}

pub fn eg_decrypt(params: &EgParams, key: &EgKeyPair, c1: &BigInt, c2: &BigInt) -> BigInt {
    let x = fast_pow_mod(&c1, key.private.as_ref().unwrap(), &params.p);
    let inv_x = mod_inv(&x, &params.p);
    return inv_x * c2 % &params.p;
}
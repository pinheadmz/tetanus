use num_bigint::{BigInt, Sign};

use tetanus::math::d;
use tetanus::elgamal::*;


fn main() {
    let mut args = std::env::args().skip(1);
    let command = args.next().expect("Expected a command");

    if command == "encrypt" {
        let msg_string = args.next().expect("Expected a message");

        let msg_bytes = msg_string.as_bytes();
        let msg = BigInt::from_bytes_be(Sign::Plus, msg_bytes);

        let params = EgParams::generate(256usize);
        let key = EgKeyPair::generate(&params);

        let (c1, c2) = eg_encrypt(&params, &key, &msg);

        println!(
            " p: {}\n g: {}\n x: {}\n m: {}\n c1: {}\n c2: {}",
            params.p.to_str_radix(10),
            params.g.to_str_radix(10),
            key.private.unwrap().to_str_radix(10),
            msg.to_str_radix(10),
            c1.to_str_radix(10),
            c2.to_str_radix(10));
    } else if command == "decrypt" {
        let p_string = args.next().expect("Expected p");
        let g_string = args.next().expect("Expected g");
        let x_string = args.next().expect("Expected x");
        let c1_string = args.next().expect("Expected c1");
        let c2_string = args.next().expect("Expected c2");

        let params = EgParams {
            p: d(&p_string),
            g: d(&g_string),
        };
        let key = EgKeyPair::from_private(&params, &d(&x_string));
        let msg = eg_decrypt(&params, &key, &d(&c1_string), &d(&c2_string));

        let (_sign, msg_bytes) = msg.to_bytes_be();
        let msg_string = String::from_utf8(msg_bytes.clone()).expect("Invalid UTF-8");
        println!("{}", msg_string);
    } else if command == "generate" {
        let params = EgParams::generate(256usize);
        let key = EgKeyPair::generate(&params);
        println!(
            " p: {}\n g: {}\n x: {}\n A: {}",
            params.p.to_str_radix(10),
            params.g.to_str_radix(10),
            key.private.unwrap().to_str_radix(10),
            key.public.to_str_radix(10));
    } else {
        panic!("Unknown command");
    }
}

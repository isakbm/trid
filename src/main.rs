const ALPHABET: &str = "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789";

use rand::prelude::*;
use std::{
    collections::HashSet,
    time::{self, UNIX_EPOCH},
};

fn bn_encode(i: u128, alphabet: &str) -> String {
    let alpha_array = alphabet.chars().collect::<Vec<_>>();
    let base = alpha_array.len() as u128;

    let mut out = String::from("");

    let mut curr = 0;
    let mut symbol_position = 0;

    while i != curr {
        let alphabet_location = ((i - curr) / base.pow(symbol_position)) % base;
        curr += alphabet_location * base.pow(symbol_position);

        out.push(alpha_array[alphabet_location as usize]);

        symbol_position += 1;
    }

    out.chars().rev().collect()
}

fn bn_decode(i: &String, alphabet: &str) -> u128 {
    let alpha_array = alphabet.chars().collect::<Vec<_>>();
    let base = alpha_array.len() as u128;

    let mut result = 0;
    for (exponent, c) in i.chars().rev().enumerate() {
        let j = alpha_array.iter().position(|&x| x == c).unwrap() as u128;
        result += j * base.pow(exponent as u32);
    }

    result
}

fn uid64() -> u64 {
    let now = time::SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let mut rng = rand::thread_rng();

    let tn = (now.as_millis() % (u32::MAX as u128)) as u32;
    let rn = rng.next_u32();

    let a = tn;
    let b = (tn as u64) << 32;
    let c = (b >> 32) as u32;
    assert!(a == c);

    let out = ((tn as u64) << 32) + (rn as u64);

    out
}

fn uid128() -> u128 {
    let now = time::SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let mut rng = rand::thread_rng();

    let tn = (now.as_millis() % (u64::MAX as u128)) as u64;
    let rn = rng.next_u64();

    let out = ((tn as u128) << 64) + (rn as u128);

    out
}

fn main() {
    println!("now: {:?}", time::Instant::now());
    println!("c.len() = {:?}", ALPHABET.chars().count());

    let i = 512312_u128;

    let encode_i = bn_encode(i, ALPHABET);
    let decode_encode_i = bn_decode(&encode_i, ALPHABET);

    println!("input: {i}");
    println!("encoded: {encode_i}");
    println!("decoded: {decode_encode_i}");

    for _ in 0..10 {
        let id = uid128();
        let enc_id = bn_encode(id, ALPHABET);
        assert_eq!(id, bn_decode(&enc_id, ALPHABET));
        println!("id: {} <= {}", enc_id, id);
    }

    let mut set_of_ids = HashSet::<u64>::default();

    let num_ids_to_generate = 10_000_000;

    let mut collisions = 0;

    let mut rng = rand::thread_rng();

    let start = time::Instant::now();

    for _ in 0..num_ids_to_generate {
        let mut id = uid64();
        while !set_of_ids.insert(id as u64) {
            println!("retrying due to collision");
            collisions += 1;
            id = uid64();
        }
    }

    println!("time elapsed: {:?}", time::Instant::now() - start);

    println!("generated: {num_ids_to_generate} unique ids");

    println!(
        "{} collisions => {:.5}%",
        collisions,
        (collisions as f64 / num_ids_to_generate as f64) * 100.0
    );
}

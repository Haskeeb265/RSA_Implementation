use crate::key_generation;
use std::str;
use num_bigint::BigUint;
use num_traits::FromPrimitive;

pub fn encryption(M: &str, p: &BigUint, q: &BigUint) -> BigUint { //In this function we're taking a message M of type string and 2 prime numbers. The function is returning a number of type BigUint.
    let n = key_generation::calculating_n(p.clone(), q.clone());
    let e = BigUint::from_u64(65537).unwrap(); //Using unwrap to handle options.

    // Convert the message M to a BigUint
    let m = BigUint::from_bytes_be(M.as_bytes()); //Converting the message into a BigUint type. The from_bytes_be method first converts the string into bytes and then into BigUint

    // Encrypt the message using the formula C = M^e mod n
    let c = m.modpow(&e, &n);

    c
}
use crate::key_generation;
use std::str;
use num_bigint::BigUint;
use num_traits::FromPrimitive;

pub fn encryption(m: &str, n: &BigUint) -> BigUint { // Accept n as a parameter
    let e = BigUint::from_u64(65537).unwrap(); // Using unwrap to handle options.

    // The M string is first converted into an array of bytes values which is then converted into a BigUint type.
    let m = BigUint::from_bytes_be(M.as_bytes()); // Converting the message into a BigUint type. The from_bytes_be method first converts the string into bytes and then into BigUint

    // Encrypt the message using the formula C = M^e mod n
    let c = m.modpow(&e, n);

    c
}
use num_bigint::BigUint;
use std::string::FromUtf8Error;

pub fn decryption(c: &BigUint, n: &BigUint, d: &BigUint) -> Result<String, FromUtf8Error> {
    let m = c.modpow(d, n);
    let message = m.to_bytes_be();
    String::from_utf8(message)
}
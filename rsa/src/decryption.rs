use crate::encryption;
use crate::key_generation;

pub fn decryption(c: &BigUint, n: &BigUint , d: &BigUint) -> String {

    let m = c.modpow(d, &n);

    let message = m.to_bytes_be();
    let message = String::from_utf8(message).unwrap();

    message
}

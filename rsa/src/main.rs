mod key_generation;
mod encryption;
mod decryption;
use num_bigint::BigUint;

fn main() {
    let (p, q) = key_generation::two_prime_generator();
    let n = key_generation::calculating_n(p.clone(), q.clone());
    let et = key_generation::euler_totient(p.clone(), q.clone());
    let e = BigUint::from(65537u64);
    let d = key_generation::private_key(e, et.clone());
    let encrpyt = encryption::encryption("My name is Haseeb", &n);

    match decryption::decryption(&encrpyt, &n, &d) {
        Ok(decrypt) => println!("Decrypted Message: {}", decrypt),
        Err(e) => {
            println!("Failed to decrypt message: {}", e);
            // Print the raw bytes for debugging
            let m = encrpyt.modpow(&d, &n);
            let raw_bytes = m.to_bytes_be();
            println!("Raw decrypted bytes: {:?}", raw_bytes);
        },
    }

    println!("Random Primes: {},{}", p, q);
    println!("Euler Totient: {}", et);
    println!("n = : {}", n);
    println!("Private Key: {}", d);
    println!("Encrypted Message: {}", encrpyt);
}
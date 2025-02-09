mod key_generation;
mod encryption;
use num_bigint::BigUint;

fn main() {

    let (p,q) = key_generation::two_prime_generator();
    let n = key_generation::calculating_n(p.clone(),q.clone());
    let et = key_generation::euler_totient(p.clone(),q.clone());
    let e = BigUint::from(65537u64);
    let pk = key_generation::private_key(e,et.clone());
    let encrpyt = encryption::encryption("Hello! My name is Haseeb", &p, &q);

    println!("Random Primes: {},{}", p,q);
    println!("Euler Totient: {}", et);
    println!("n = : {}", n);
    println!("Private Key: {}", pk);
    println!("Encrypted Message: {}", encrpyt);


}
use num_primes::Generator;
use num_bigint::BigUint;

pub fn two_prime_generator() -> (num_primes::BigUint, num_primes::BigUint) {

    let p = Generator::new_prime(64);
    let mut q = Generator::new_prime(64);
    
    while p == q{
        q = Generator::new_prime(64);
    }

    (p,q)
}



pub fn calculating_n (p: BigUint, q: BigUint) -> BigUint{
    
    p*q

}
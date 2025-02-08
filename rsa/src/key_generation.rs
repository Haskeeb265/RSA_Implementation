use num_primes::Generator;

pub fn two_prime_generator() -> (num_primes::BigUint, num_primes::BigUint) {
    let prime1 = Generator::new_prime(64);
    let mut prime2 = Generator::new_prime(64);

    while prime1 == prime2 {
        prime2 = Generator::new_prime(64);
    }

    (prime1, prime2)
}
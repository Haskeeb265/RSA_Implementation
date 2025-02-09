use num_primes::Generator; //This library is used to generate random prime numbers.
use num_bigint::BigUint; //This is to deal with large numbers

pub fn two_prime_generator() -> (num_primes::BigUint, num_primes::BigUint) { //Specifying that the reurn value of this function are 2 values of type BigUint

    let p = Generator::new_prime(64); //p = random 64 bit prime number
    let mut q = Generator::new_prime(64); //p = random 64 bit prime number
    
    while p == q{ //while p and q are same, keep generating new prime number for q
        q = Generator::new_prime(64);
    }

    (p,q) //return p and q
}



pub fn calculating_n (p: BigUint, q: BigUint) -> BigUint{ //Taking p and q of type BigUint as input parameters and returning a value of type BigUint
    
    p*q //Product of p and q

}
use num_primes::Generator; // This library is used to generate random prime numbers.
use num_bigint::{BigUint, BigInt, Sign, ToBigInt}; // This is to deal with large numbers.
use num_integer::Integer; // This is to implement modular arithmetic.
use num_traits::One; // This is to use the 'one' method for BigInt.

pub fn two_prime_generator() -> (num_primes::BigUint, num_primes::BigUint) { //Specifying that the reurn value of this function are 2 values of type BigUint

    let p = Generator::new_prime(64); //p = random 64 bit prime number
    let mut q = Generator::new_prime(64); //p = random 64 bit prime number
    
    while p == q{ //while p and q are same, keep generating new prime number for q
        q = Generator::new_prime(64);
    }

    (p,q) //return p and q
}

pub fn calculating_n(p: BigUint, q: BigUint) -> BigUint{ //Taking p and q of type BigUint as input parameters and returning a value of type BigUint
    
   p*q //Product of p and q
 

}

pub fn euler_totient(p:BigUint, q: BigUint) -> BigUint{
 (p - BigUint::from(1u64)) * (q - BigUint::from(1u64))
}


pub fn private_key(e: BigUint, et: BigUint) -> BigUint {
    let e_bigint = e.to_bigint().unwrap(); //Converting BigUint to BigInt.
    let et_bigint = et.to_bigint().unwrap(); 

    let result = e_bigint.extended_gcd(&et_bigint); 

    // Check if GCD is 1
    if result.gcd != BigInt::one() {
        panic!("Modular inverse does not exist!");
    }

    let mut d = result.x; //.x is the 2nd value we get fomr the extended gcd method.

    // If d is negative, make it positive by adding et
    if d.sign() == Sign::Minus {
        d += &et_bigint;
    }
    
    d.to_biguint().expect("Conversion to BigUint failed")
}
use num_primes::Generator; //This library is used to generate random prime numbers.
use num_bigint::BigUint; //This is to deal with large numbers
use num_integer::Integer; //This is to implement modular arthematic

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
   let et = (p - BigUint::from(1u64)) * (q - BigUint::from(1u64));
   et //Stored euler's totient in variable "et"
}

// Calculate the modular inverse of e modulo φ(n)
pub fn private_key(e: BigUint, et: BigUint) -> BigUint {
    e.modinv(&et).expect("Modular inverse does not exist!") //& sign is used for referencing.
}

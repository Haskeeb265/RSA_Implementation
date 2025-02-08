use rand::Rng; //Generates random numbers. Imported Rng from rand crate
use rand::thread_rng; //Initializes the random number generator for the current thread
use num_prime::RandPrime; //Used to generate prime numbers and apply prime functions

pub fn generating_first_prime() -> u64 { //When we want to return a single value

    let mut rng = thread_rng();
    rng.gen_prime(64) //We don't use semi-colons when we want to return a value.

}


pub fn generating_second_prime() -> (u64,u64) { //when we want to return multiple values

    let p = generating_first_prime();
    let mut q;

    loop{ //Continuously perform the below 2 tasks until the condition is met
        q = generating_first_prime(); //generate a prime value
        if p != q{ //Check if the the prime value generated is not equal to p
            break;
        }
    }

    (p,q) //returning p and q
}

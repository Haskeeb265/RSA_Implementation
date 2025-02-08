mod key_generation;

fn main() {
    // Destructure the returned tuple
    let (p, q) = key_generation::two_prime_generator();
    
    println!("First prime:  {}", p);
    println!("Second prime: {}", q);
}
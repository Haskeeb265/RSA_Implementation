mod key_generation;

fn main() {

    let (p,q) = key_generation::two_prime_generator();
    let n = key_generation::calculating_n(p,q);
    println!("n:{}", n);

}
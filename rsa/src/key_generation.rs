use rand::prelude::*;

let mut rng = rand::rng();

println!("Char: '{}'",rng.random::<char>());
println!("alpha: '{}'", rng.sample(rand::distr::Alphanumeric) as char);let mut nums: Vec<i32> = (1..100).collect();
nums.shuffle(&mut rng);
let _ = nums.choose(&mut rng);
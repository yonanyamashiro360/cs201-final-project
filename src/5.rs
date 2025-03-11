import rand::{Rand, thread_rng};

fn main() {
    let mut rng = thread_rng();
    println!("The random number is: {}", rng.gen::<i32>());
}
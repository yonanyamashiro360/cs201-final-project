fn main() {
    let mut rng = rand::thread_rng();
    let value: u32 = rng.gen_range(1..=5);
    println!("Value generated: {}", value);
}

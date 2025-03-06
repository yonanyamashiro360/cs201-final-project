fn main() {
  let mut rng = rand::thread_rng();
  let mut nums: Vec<i32> = (0..10).map(|_| rng.gen_range(0..10)).collect();
  println!("{:?}", nums);
}

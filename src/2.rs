
  use std::rand;
  fn main() {
    let mut rng = rand::thread_rng();
    println!("{}", rng.gen::<f32>());
  }
fn main() {
  let mut numbers = vec![1, 2, 3];
  let index = rand::thread_rng().gen_range(0..numbers.len());
  println!("{}", numbers[index]);
}

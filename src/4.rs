fn main() {
    let mut rng = thread_rng();
    let number: u32 = rng.gen_range(1..=10);
    println!("The number is {}", number);
}

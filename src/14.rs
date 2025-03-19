fn main() {
    let mut rng = rand::thread_rng();
    let die_roll: u8 = rng.gen_range(1..=6);
    println!("You rolled a {}!", die_roll);
}

use std::{thread, time};

fn main() {
    let mut numbers = (1..=10).collect::<Vec<i32>>();

    thread::spawn(move || {
        for number in &numbers {
            println!("Number: {}", number);
        }
    });

    for number in &mut numbers {
        *number += 1;
    }
}

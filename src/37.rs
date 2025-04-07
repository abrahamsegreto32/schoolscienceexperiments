use std::fs;

fn main() {
    let data = fs::read_to_string("path_to_your_file").expect("Failed to read file");
    println!("{}", data);
}

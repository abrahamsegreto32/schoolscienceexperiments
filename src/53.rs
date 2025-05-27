use std::fs::{self, File};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Read data from file
    let contents = fs::read_to_string("data.txt")?;
    
    // Print the content of the file
    println!("{}", contents);

    Ok(())
}

use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data.txt")?;
    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }

    Ok(())
}

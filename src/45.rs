use std::fs;

fn main() {
    let file_path = "data/schoolscienceexperiments.txt"; // Replace with your actual file path

    if let Ok(file) = fs::read_to_string(file_path) {
        println!("File contents:");
        for line in file.lines() {
            println!("{}", line);
        }
    } else {
        eprintln!("Failed to read the file: {}", file_path);
    }
}

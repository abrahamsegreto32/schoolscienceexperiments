fn main() {
    let mut student_name = "Alice";
    let experiment_name = "Light vs Dark";
    let outcome = if true {
        "Bright light makes us feel better."
    } else {
        "Darkness is more calming for most people."
    };
    println!("Student Name: {}", student_name);
    println!("Experiment Name: {}", experiment_name);
    println!("{}", outcome);
}

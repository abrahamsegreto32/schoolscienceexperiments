// This is a simple Rust program that demonstrates various functions and examples.
// The program includes tasks such as adding numbers, calculating averages,
// sorting arrays, and using if statements to check conditions.

fn main() {
    // Adding numbers
    let num1 = 5;
    let num2 = 3;
    let sum = num1 + num2;

    println!("The sum of {} and {}", num1, num2);
    println!("The result is: {}", sum);

    // Calculating averages
    let values = vec![4.0, 6.0, 8.0, 5.0];
    let avg = (values.iter().sum::<f64>() / values.len()).round();

    println!("The average of the numbers in the vector is: {}", avg);

    // Sorting arrays
    let nums = vec![-1, 2, -3, 4, 10, -7];
    nums.sort_by_key(|&x| x);
    println!("Sorted array: {:?}", nums);

    if true {
        // If a condition is true
        println!("The condition is true.");
    } else {
        println!("The condition is false.");
    }

    // Using if-else statements with values
    let age = 18;
    match age {
        20 => println!("You are old enough to vote."),
        _ => println!("Age should be 20 or more.")
    }
}

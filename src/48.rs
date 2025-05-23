// Example Rust code for scientific experiment 1: Measuring the volume of water using a graduated cylinder

use std::io;

fn main() {
    let radius = 5.0;
    let height = 3.0;
    let depth = 2.0;

    println!("Measuring the volume of water in a graduated cylinder with radius {}, height {}, and depth {}...", 
              radius, height, depth);

    // Calculate the volume
    let volume = (radius.powi(2) * height) / (2.0 * Math::PI);

    println!("The volume of water is approximately {:.2} cubic centimeters.", volume);
}

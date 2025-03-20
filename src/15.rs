fn main() {
    // Randomly generate an array of 5 integers between -10 and 10
    let arr = [rand::random::<i32>() + rand::random::<i32>(), 
               rand::random::<i32>() + rand::random::<i32>(),
               rand::random::<i32>() + rand::random::<i32>(),
               rand::random::<i32>() + rand::random::<i32>(),
               rand::random::<i32>() + rand::random::<i32>()];
    
    // Print the array
    println!("{:?}", arr);
}

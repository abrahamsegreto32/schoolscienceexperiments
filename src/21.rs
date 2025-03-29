use std::collections::HashMap;

fn main() {
    let mut data = HashMap::new();

    // Add data to the map
    data.insert("key1".to_string(), "value1".to_string());
    data.insert("key2", "value2");

    // Print the value of a key
    println!("{}", data.get(&"key1".to_string()).unwrap().clone());

    // Clear the map
    data.clear();
}

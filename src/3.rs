use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    // Insert some values into the map
    map.insert("color", "red");
    map.insert("number", 23);

    // Print out the values in the map
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
}

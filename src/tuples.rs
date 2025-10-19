use std::collections::HashMap;

fn group(vec: Vec<(String, i32)>) -> HashMap<String, Vec<i32>> {
    let mut map = HashMap::new();
    
    for (key, value) in vec {
        // If key exists, push to Vec; otherwise, insert a new Vec
        map.entry(key)
            .or_insert(Vec::new())
            .push(value);
    }

    map
}

fn main() {
    let data = vec![
        (String::from("Ram"), 30),
        (String::from("Hari"), 25),
        (String::from("Neha"), 28),
    ];

    let grouped_data = group(data);

    for (key, values) in &grouped_data {
        println!("{}: {:?}", key, values);
    }
}

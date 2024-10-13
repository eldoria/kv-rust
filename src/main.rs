mod hashmap;
mod linked_list;

use linked_list::Value;


fn main() {
    let mut hashmap = hashmap::HashMap::new(&10);

    let keys: Vec<Value> = vec![
        Value::Int(1),
        Value::Int(2),
        Value::Text("key_1".to_string()),
        Value::Text("key_2".to_string()),
        Value::Text("key_3".to_string()),
    ];

    // Example of creating values as well
    let values: Vec<Value> = vec![
        Value::Int(1),
        Value::Text("2".to_string()),
        Value::Int(3),
        Value::Text("4".to_string()),
        Value::Int(5),
    ];

    for (key, value) in keys.iter().zip(values.iter()) {
        hashmap.push(key.into(), value.into());
    }

    // hashmap.delete(&Value::Text("key_2".to_string())); // to check None case

    for key in keys.iter() {
        match hashmap.get(key) {
            Some(value) => {
                println!("Value of the key {} is {}", key, value);
            },
            None => {
                println!("Key {} not found in the hashmap", key);
            }
        }
    }

    print!("{}", hashmap)
}

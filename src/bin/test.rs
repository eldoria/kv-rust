use lib::storage::Storage;
use lib::hashmap::HashMap;

fn main() {
    let hashmap = HashMap::new(&10);
    print!("return: {:?}", Storage::save(
        hashmap,
        "database/data.json")
    );
    print!("return: {:?}", Storage::load("database/data.json", 5))
}

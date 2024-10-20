
use serde_json;

use crate::hashmap::HashMap;

pub struct Storage {

}

impl Storage {
    pub fn load(file_path: &str, capacity: u32) -> HashMap {
        match std::fs::read_to_string(file_path) {
            Err(_) => HashMap::new(&capacity),
            Ok(hashmap_json) => serde_json::from_str(&hashmap_json)
                .unwrap_or_else(|_| HashMap::new(&capacity))
        }
    }

    pub fn save(hashmap: HashMap, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let hashmap_json = serde_json::to_string(&hashmap)?;
        std::fs::write(file_path, hashmap_json)?;
        Ok(())
    }
}

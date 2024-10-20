
use serde_json;

use crate::hashmap::HashMap;

pub struct Storage {

}

impl Storage {
    pub fn load(file_path: &str) -> Result<HashMap, Box<dyn std::error::Error>> {
        let json_string = std::fs::read_to_string(file_path)?;
        let hashmap: HashMap = serde_json::from_str(&json_string)?;
        Ok(hashmap)
    }

    pub fn save(hashmap: HashMap, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let hashmap_json = serde_json::to_string(&hashmap)?;
        std::fs::write(file_path, hashmap_json)?;
        Ok(())
    }
}

use std::fmt::{self, Display};
use serde::{Serialize, Deserialize};

use crate::linked_list::{LinkedList, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct HashMap {
    capacity: u32,
    size: u32,
    hashmap: Vec<LinkedList>
}


impl HashMap {
    pub fn new(capacity: &u32) -> HashMap {
        let hashmap: Vec<LinkedList> = (0..*capacity)
            .map(|_| LinkedList::new())
            .collect();

        HashMap {
            capacity: *capacity as u32,
            size: 0,
            hashmap,
        }
    }


    fn hash(&self, key: &Value) -> u32 {
        let key_str = match key {
            Value::Int(i) => i.to_string(),
            Value::Text(s) => s.clone()
        };

        let mut hash_value: u32 = 0;

        for c in key_str.chars() {
            hash_value += c as u32;
        }

        hash_value % self.capacity
    }

    fn extend(&mut self) {
        if self.size >= self.capacity / 2 {
            let old_hashmap = self.hashmap.clone();
            let old_capacity = self.capacity;

            self.capacity *= 2;
            let new_hashmap: Vec<LinkedList> = (0..self.capacity)
                .map(|_| LinkedList::new())
                .collect();
            self.hashmap = new_hashmap;
            self.size = 0;

            for i in 0..old_capacity {
                let mut old_linked_list = old_hashmap.get(i as usize).unwrap();
                loop {
                    match old_linked_list.pointer {
                        Some(ref next) => {
                            self.push(old_linked_list.key.as_ref().unwrap(), &old_linked_list.value.as_ref().unwrap());
                            old_linked_list = next;
                        }
                        None => {
                            if old_linked_list.value.is_some() {
                                self.push(&old_linked_list.key.as_ref().unwrap(), &old_linked_list.value.as_ref().unwrap());
                            }
                            break;
                        }
                    }
                }
            }
            println!("Hashmap extended from {} to {}", old_capacity, self.capacity)
        }
    }

    pub fn get(&self, key: &Value) -> Option<&Value> {
        let idx = self.hash(key);
        let mut linked_list = self.hashmap.get(idx as usize)?;

        loop {
            if linked_list.key.as_ref() == Some(key) {
                return linked_list.value.as_ref();
            } else if let Some(ref next) = &linked_list.pointer {
                linked_list = next;
            } else {
                return None;
            }
        }
    }

    pub fn push(&mut self, key: &Value, value: &Value) -> Option<Value> {
        self.extend();
        let idx = self.hash(key);
        let mut linked_list = self.hashmap.get_mut(idx as usize)?;

        loop {
            if linked_list.key.as_ref() == Some(key) { // if we find a key already present, we replace the value
                let old_value = linked_list.value.clone();
                linked_list.value = Some(value.clone());
                self.size += 1;
                return old_value;
            } else if let Some(ref mut next) = linked_list.pointer { // if we are not at the end of the linked list, we continue
                linked_list = next;
            } else { // we are at the end of the list
                match linked_list.value {
                    None => {
                        let old_value = linked_list.value.clone();
                        linked_list.key = Some(key.clone());
                        linked_list.value = Some(value.clone());
                    },
                    Some(_) => {
                        let new_cell: LinkedList = LinkedList {
                            key: Some(key.clone()),
                            value: Some(value.clone()),
                            pointer: None,
                        };
                        linked_list.pointer = Some(Box::new(new_cell));
                    }
                }
                self.size += 1;
                return None;
            }
        }
    }

    pub fn delete(&mut self, key: &Value) -> Option<Value> {
        let idx = self.hash(key);
        let mut linked_list = self.hashmap.get_mut(idx as usize)?;

        loop {
            if linked_list.key.as_ref() == Some(key) {
                let old_value = linked_list.value.clone();
                linked_list.value = None;
                self.size -= 1;
                return old_value;
            } else if (linked_list.pointer.as_ref().unwrap().key.as_ref() == Some(key)) &
                    (linked_list.pointer.as_ref().unwrap().pointer.is_some()) {
                let old_value = linked_list.value.clone();
                linked_list.value = None;
                linked_list.pointer = linked_list.pointer.as_ref().unwrap().pointer.clone();
                self.size -= 1;
                return old_value;
            } else if let Some(ref mut next) = linked_list.pointer {
                linked_list = next;
            }
            else {
                return None
            }
        }
    }
}

impl Display for HashMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.capacity {
            let mut keys: Vec<Value> = Vec::new();
            let mut vals: Vec<Value> = Vec::new();

            let mut current_linked_list = self.hashmap.get(i as usize).unwrap();
            loop {
                // Unwrap and access keys and values
                if let Some(ref node) = current_linked_list.pointer {
                    keys.push(current_linked_list.key.clone().unwrap());
                    vals.push(current_linked_list.value.clone().unwrap());
                    current_linked_list = node;  // Move to the next node
                } else {
                    // Handle last node if it contains data
                    if current_linked_list.value.is_some() {
                        keys.push(current_linked_list.key.clone().unwrap());
                        vals.push(current_linked_list.value.clone().unwrap());
                    }

                    // to align the first key/value between each line
                    let size_id = Value::Int(i as i64).len();
                    let max_size_id = Value::Int((self.capacity - 1) as i64).len();
                    let space_increment_id= " ".repeat(size_id as usize);
                    let max_space_increment_id = " ".repeat((max_size_id - size_id) as usize);

                    // Formatting output
                    let mut line_1 = format!("{}{}   ", i, max_space_increment_id);
                    let mut line_2 = format!("{}{}   ", space_increment_id, max_space_increment_id);
                    for j in 0..keys.len() {
                        let key = keys.get(j).unwrap();
                        let val = vals.get(j).unwrap();

                        // to align keys/values line by line
                        let size_key = key.len();
                        let size_val = val.len();
                        let space_increment_key =
                            " ".repeat(size_val.saturating_sub(size_key) as usize);
                        let space_increment_val =
                            " ".repeat(size_key.saturating_sub(size_val) as usize);

                        if j == keys.len() - 1 {
                            line_1.push_str(&format!("key: {}{}", key, space_increment_key));
                            line_2.push_str(&format!("val: {}{}", val, space_increment_val));
                        } else {
                            line_1.push_str(&format!("key: {}{} -> ", key, space_increment_key));
                            line_2.push_str(&format!("val: {}{} -> ", val, space_increment_val));
                        }
                    }

                    write!(f, "{}\n{}\n", line_1, line_2)?;
                    break;
                }
            }
        }
        Ok(())
    }
}

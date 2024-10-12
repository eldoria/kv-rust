use crate::linked_list::LinkedList;
use crate::linked_list::Value;

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

            self.capacity *= 2;
            let new_hashmap: Vec<LinkedList> = (0..self.capacity)
                .map(|_| LinkedList::new())
                .collect();
            self.hashmap = new_hashmap;

            for i in 0..self.capacity {
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
            if linked_list.key.as_ref() == Some(key) {
                let old_value = linked_list.value.clone();
                linked_list.value = Some(value.clone());
                self.size += 1;
                return old_value;
            } else if let Some(ref mut next) = linked_list.pointer {
                linked_list = next;
            } else {
                let new_cell: LinkedList = LinkedList {
                    key: Some(key.clone()),
                    value: Some(value.clone()),
                    pointer: None,
                };
                linked_list.pointer = Some(Box::new(new_cell));
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

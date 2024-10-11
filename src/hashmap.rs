use crate::linked_list::LinkedList;
use crate::linked_list::Value;
use array_init::array_init;


const HASHMAP_INITIAL_CAPACITY: usize = 10;

pub struct HashMap {
    capacity: u32,
    size: u32,
    map: [LinkedList; HASHMAP_INITIAL_CAPACITY]
}


impl HashMap {
    pub fn new() -> HashMap {
        let linked_list_array : [LinkedList; HASHMAP_INITIAL_CAPACITY] = array_init(|_| LinkedList::new());

        HashMap{
            capacity: HASHMAP_INITIAL_CAPACITY as u32,
            size: 0,
            map: linked_list_array
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

    fn get(&self, key: &Value) -> Option<&Value> {
        let idx = self.hash(key);
        let mut linked_list = self.map.get(idx as usize)?;

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

    fn push(mut self, key: &Value, value: &Value) -> Option<Value> {
        let idx = self.hash(key);
        let mut linked_list = self.map.get_mut(idx as usize)?;

        loop {
            if linked_list.key.as_ref() == Some(key) {
                linked_list.value = Some(value.clone());
                return linked_list.value.clone();  // Optionally return the new value
            } else if let Some(ref mut next) = linked_list.pointer {
                linked_list = next;
            } else {
                let new_cell: LinkedList = LinkedList {
                    key: Some(key.clone()),
                    value: Some(value.clone()),
                    pointer: None,
                };
                linked_list.pointer = Some(Box::new(new_cell));
                return None;  // Indicate that the value was newly inserted
            }
        }
    }
}

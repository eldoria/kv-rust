use crate::linked_list::LinkedList;

const HASHMAP_INITIAL_CAPACITY: u8 = 10;

pub struct HashMap {
    capacity: u32,
    size: u32,
    map: [LinkedList]
}


impl HashMap {
    fn new() -> HashMap {
        HashMap(10, 0, [LinkedList; 10])
    }
}

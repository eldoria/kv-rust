#[derive(Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Text(String)
}

pub struct LinkedList {
    pub key: Option<Value>,
    pub value: Option<Value>,
    pub pointer: Option<Box<LinkedList>>,
}

impl LinkedList {
    pub fn new() -> LinkedList {
        LinkedList{
            key: None,
            value: None,
            pointer: None
        }
    }
}

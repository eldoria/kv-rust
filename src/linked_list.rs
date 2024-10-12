use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Int(i64),
    Text(String)
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Int(i) => write!(f, "{}", i),
            Self::Text(s) => write!(f, "{}", s)
        }
    }
}

#[derive(Clone)]
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

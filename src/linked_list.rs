use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Value {
    Int(i64),
    Text(String)
}

pub struct DisplayableOption<'a>(pub Option<&'a Value>);

impl<'a> fmt::Display for DisplayableOption<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(value) => write!(f, "{}", value),
            None => write!(f, "None"),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Int(i) => write!(f, "{}", i),
            Self::Text(s) => write!(f, "{}", s)
        }
    }
}

impl Value {
    pub fn len(&self) -> u32 {
        match self {
            Self::Int(i) => i.to_string().chars().count().try_into().unwrap(),
            Self::Text(t) => t.len().try_into().unwrap(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
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

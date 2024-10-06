enum Value {
    Int(i64),
    Text(String)
}

pub struct LinkedList {
    key: String,
    value: Option<Value>,
    pointer: Option<Box<LinkedList>>,
}

impl LinkedList {
    fn new(key: String, value: Option<Value>) -> LinkedList {
        LinkedList(key, value)
    }
}


fn main() {
    let node1 = LinkedList {
        key: String::from("node1"),
        value: Some(Value::Int(64)),
        pointer: None,
    };

    let node2 = LinkedList {
        key: String::from("node2"),
        value: Some(Value::Text(String::from("Hello"))),
        pointer: Some(Box::new(node1)),
    };

}

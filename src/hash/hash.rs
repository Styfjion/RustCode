use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Hash, Eq, PartialEq)]
struct Student<'a> {
    name: &'a str,
    age: u8
}

impl<'a> Student<'a> {
    pub fn new(name: &'a str, age: u8) -> Self {
        Self {name, age}
    }
}

fn main() {
    let mut hasher = DefaultHasher::new();
    let student = Student::new("Tyr", 18);
    student.hash(&mut hasher);
    let map = HashMap::from([(student, vec!["Math", "Writing"])]);
    println!("hash 0x{:x}, map :{:?}", hasher.finish(), map);
}
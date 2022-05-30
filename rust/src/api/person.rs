use crate::models::{Person};

pub fn get_person() -> Person {
    Person {
        first_name: String::from("Hello"),
        last_name: String::from("World")
    }
}
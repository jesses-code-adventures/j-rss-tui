use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub name: String,
}

#[allow(unused)]
impl User {
    pub fn new(name: &str) -> User {
        User {
            name: String::from(name),
        }
    }
}

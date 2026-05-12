use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub text: String,
    pub completed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct List {
    pub name: String,
    pub items: Vec<Item>,
}

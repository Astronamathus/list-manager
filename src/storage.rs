use std::fs;
use crate::models::List;

const DATA_FILE: &str = "lists.json";

pub fn load_lists() -> Vec<List> {
    let data = fs::read_to_string(DATA_FILE);

    match data {
        Ok(content) => {
            serde_json::from_str(&content).unwrap_or_else(|_| vec![])
        }
        Err(_) => vec![],
    }
}

pub fn save_lists(lists: &Vec<List>) {
    let json = serde_json::to_string_pretty(lists)
        .expect("Failed to serialize lists");

    fs::write(DATA_FILE, json)
        .expect("Failed to write file");
}

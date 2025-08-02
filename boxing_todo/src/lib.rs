use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;


mod err;

use std::error::Error;

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let file = File::open(path);
        let reader = BufReader::new(file);
        let json_data: Value = serde_json::from_reader(reader)?; 
        println!("Parsed JSON: {:?}", json_data);

    // Example: Accessing a specific field if you know the structure
    if let Some(name) = json_data["name"].as_str() {
        println!("Name: {}", name);
    }

    Ok(())
    }
}
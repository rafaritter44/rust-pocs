mod table;

use crate::table::Table;
use std::fs;
use std::path::Path;

fn main() {
    let input_path = Path::new("input.json");

    let input_string = match fs::read_to_string(input_path) {
        Ok(input_string) => input_string,
        Err(error) => {
            eprintln!("Error reading file '{}': {}.", input_path.display(), error);
            return;
        }
    };

    let table: Table = match serde_json::from_str(&input_string) {
        Ok(table) => table,
        Err(error) => {
            eprintln!("Error parsing JSON: {}.", error);
            return;
        }
    };

    println!("Successfully parsed input JSON:\n{}", serde_json::to_string_pretty(&table).unwrap());
}
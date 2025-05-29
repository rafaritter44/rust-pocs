mod table;
mod renderers;

use crate::table::Table;
use std::fs;
use std::path::Path;
use renderers::{Renderer, HtmlRenderer, CsvRenderer, RenderedContent};

fn main() {
    let input_path = Path::new("input.json");

    let input_json = match fs::read_to_string(input_path) {
        Ok(input_json) => input_json,
        Err(error) => {
            eprintln!("Error reading file '{}': {}.", input_path.display(), error);
            return;
        }
    };

    let table: Table = match serde_json::from_str(&input_json) {
        Ok(table) => table,
        Err(error) => {
            eprintln!("Error parsing JSON: {}.", error);
            return;
        }
    };

    let renderers: Vec<Box<dyn Renderer>> = vec![
        Box::new(HtmlRenderer),
        Box::new(CsvRenderer),
    ];

    for renderer in renderers.iter() {
        let rendered = renderer.render(&table);
        let output_file = format!("output.{}", renderer.file_extension());
        let output_path = Path::new(&output_file);

        match rendered {
            RenderedContent::Text(content) => {
                if let Err(e) = fs::write(output_path, content) {
                    eprintln!("Error writing to '{}': {}.", output_path.display(), e);
                }
            }
            RenderedContent::Binary(_) => {
                eprintln!("PDF rendering not supported yet.");
            }
        }
    }
}
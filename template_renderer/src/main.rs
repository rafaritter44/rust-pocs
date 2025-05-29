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
        Err(e) => {
            eprintln!("Error reading file '{}': {}.", input_path.display(), e);
            return;
        }
    };

    let table: Table = match serde_json::from_str(&input_json) {
        Ok(table) => table,
        Err(e) => {
            eprintln!("Error parsing JSON: {}.", e);
            return;
        }
    };

    let renderers: Vec<Box<dyn Renderer>> = vec![
        Box::new(HtmlRenderer),
        Box::new(CsvRenderer),
    ];

    for renderer in renderers.iter() {
        let rendered_content = renderer.render(&table);
        let output_filename = format!("output.{}", renderer.file_extension());
        let output_path = Path::new(&output_filename);

        match rendered_content {
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
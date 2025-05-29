use crate::table::Table;
use super::renderer::Renderer;
use super::rendered_content::RenderedContent;

pub struct CsvRenderer;

impl Renderer for CsvRenderer {
    fn render(&self, table: &Table) -> RenderedContent {
        let mut csv = String::new();

        csv.push_str(&table.headers.join(","));
        csv.push('\n');

        for row in &table.rows {
            csv.push_str(&row.join(","));
            csv.push('\n');
        }

        RenderedContent::Text(csv)
    }
}
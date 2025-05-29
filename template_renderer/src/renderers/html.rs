use crate::table::Table;
use super::renderer::Renderer;
use super::rendered_content::RenderedContent;

pub struct HtmlRenderer;

impl Renderer for HtmlRenderer {
    fn render(&self, table: &Table) -> RenderedContent {
        let mut html = String::new();
        html.push_str("<div>\n<div>\n");

        for header in &table.headers {
            html.push_str(&format!("<h1>{}</h1>\n", header));
        }
        html.push_str("</div>\n");

        for row in &table.rows {
            html.push_str("<div>\n");
            for cell in row {
                html.push_str(&format!("<p>{}</p>\n", cell));
            }
            html.push_str("</div>\n");
        }

        html.push_str("</div>");
        RenderedContent::Text(html)
    }
    
    fn file_extension(&self) -> &str {
        "html"
    }
}
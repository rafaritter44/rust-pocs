use crate::table::Table;
use super::renderer::Renderer;
use super::rendered_content::RenderedContent;

pub struct HtmlRenderer;

impl Renderer for HtmlRenderer {
    fn render(&self, table: &Table) -> RenderedContent {
        let mut html = String::new();
        html.push_str("<table>\n<thead>\n<tr>");

        for header in &table.headers {
            html.push_str(&format!("<th>{}</th>", header));
        }
        html.push_str("</tr>\n</thead>\n<tbody>\n");

        for row in &table.rows {
            html.push_str("<tr>");
            for cell in row {
                html.push_str(&format!("<td>{}</td>", cell));
            }
            html.push_str("</tr>\n");
        }

        html.push_str("</tbody>\n</table>");
        RenderedContent::Text(html)
    }
    
    fn file_extension(&self) -> &str {
        "html"
    }
}
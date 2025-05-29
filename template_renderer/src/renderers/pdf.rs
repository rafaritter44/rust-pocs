use crate::table::Table;
use super::renderer::Renderer;
use super::rendered_content::RenderedContent;

pub struct PdfRenderer;

impl Renderer for PdfRenderer {
    fn render(&self, table: &Table) -> RenderedContent {
        let pdf_bytes = vec![/* PDF content as bytes */];
        RenderedContent::Binary(pdf_bytes)
    }
}
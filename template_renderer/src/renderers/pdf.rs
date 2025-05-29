use std::collections::BTreeMap;

use crate::table::Table;
use super::renderer::Renderer;
use super::rendered_content::RenderedContent;
use printpdf::*;

pub struct PdfRenderer;

impl Renderer for PdfRenderer {
    fn render(&self, table: &Table) -> RenderedContent {
        let html = "";
        let images = BTreeMap::new();
        let fonts = BTreeMap::new();
        let options = GeneratePdfOptions {
            ..Default::default()
        };
        let mut warnings = Vec::new();

        let pdf_bytes = PdfDocument::from_html(html, &images, &fonts, &options, &mut warnings)
            .unwrap()
            .save(&PdfSaveOptions::default(), &mut warnings);

        RenderedContent::Binary(pdf_bytes)
    }

    fn file_extension(&self) -> &str {
        "pdf"
    }
}
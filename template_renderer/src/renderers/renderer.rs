use crate::table::Table;
use super::rendered_content::RenderedContent;

pub trait Renderer {
    fn render(&self, table: &Table) -> RenderedContent;
    fn file_extension(&self) -> &str;
}
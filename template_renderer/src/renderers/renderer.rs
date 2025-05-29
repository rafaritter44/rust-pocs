use crate::table::Table;
use crate::rendered_content::RenderedContent;

pub trait Renderer {
    fn render(&self, table: &Table) -> RenderedContent;
}
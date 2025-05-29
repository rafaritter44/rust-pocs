pub enum RenderedContent {
    Text(String),
    Binary(Vec<u8>),
}

impl RenderedContent {
    pub fn into_bytes(self) -> Vec<u8> {
        match self {
            RenderedContent::Text(string) => string.into_bytes(),
            RenderedContent::Binary(bytes) => bytes,
        }
    }
}
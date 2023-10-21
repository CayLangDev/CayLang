#[derive(Debug)]
pub struct TemplateLiteral {
    pub relative: bool,
    pub parts: Vec<Vec<TemplatePart>>
}

#[derive(Debug)]
pub enum TemplatePart {
    LayerPart(String),
    Text(String)
}

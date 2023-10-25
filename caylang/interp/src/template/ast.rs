#[derive(Debug)]
pub struct TemplateLiteral {
    pub relative: bool,
    /// The actual outuput consists of concatenating each element list of parts, evaluating LayerPart values
    /// and then joining each of those strings between path component seperators ("/").
    pub parts: Vec<Vec<TemplatePart>>
}

#[derive(Debug)]
pub enum TemplatePart {
    /// Layer part i.e. "{a}" where a is an identifier from a type destructure
    /// the string stored is the contents between the brackets
    LayerPart(String),
    /// Text part i.e. "_", plain text to be inserted as is
    Text(String)
}

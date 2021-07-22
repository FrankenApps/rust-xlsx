use super::document_properties::DocumentProperties;

pub struct Document {
    general_properties: DocumentProperties,
    rows: Vec<String>,
    shared_strings: Vec<String>
}

impl Document {
    pub fn new(general_properties: DocumentProperties) -> Self {
        Self {
            general_properties,
            rows: Vec::new(),
            shared_strings: Vec::new()
        }
    }
}
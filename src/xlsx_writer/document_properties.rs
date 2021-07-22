pub struct DocumentProperties {
    author: String,
    company: String,
    created: String,
}

impl DocumentProperties {
    pub fn new(author: String, company: String, created: String) -> Self {
        Self {
            author,
            company,
            created,
        }
    }
}

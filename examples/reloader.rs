pub struct FileReloader {
    path: String,
    content: String,
}

impl FileReloader {
    pub fn new(path: &str) -> FileReloader {
        let path = path.to_string();
        Self {
            path,
            content: "".to_string(),
        }
    }

    /// Returns Some(new_text) if file changed since last call, otherwise None.
    pub fn update(&mut self) -> Option<String> {
        let content_new = std::fs::read_to_string(&self.path).unwrap();
        if self.content != content_new {
            self.content = content_new;
            Some(self.content.clone())
        } else {
            None
        }
    }
}

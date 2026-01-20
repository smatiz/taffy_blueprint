pub struct BasicFileHotReloader {
    path: String,
    content: String,
}

impl BasicFileHotReloader {
    pub fn new(path: &str) -> BasicFileHotReloader {
        let path = path.to_string();
        Self {
            path,
            content: "".to_string(),
        }
    }

    pub fn update(&mut self) -> Option<String> {
        match std::fs::read_to_string(&self.path) {
            Ok(content_new) => {
                if self.content != content_new {
                    self.content = content_new;
                    Some(self.content.clone())
                } else {
                    None
                }
            }
            Err(e) => {
                println!("Error FileReloader: {}", e);
                None
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Alias(String);

impl From<&str> for Alias {
    fn from(value: &str) -> Self {
        Self::new(value.to_string())
    }
}

impl Alias {
    pub fn new(name: String) -> Self {
        Alias(name)
    }
    pub fn name(&self) -> &str {
        &self.0
    }
}

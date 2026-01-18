#[derive(Debug)]
pub struct Column {
    pub name: &'static str,
}

impl Column {
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }
}

#[cfg(test)]
#[path = "schema_test.rs"]
mod tests; // 告诉编译器，这个模块的内容在 a_test.rs 里

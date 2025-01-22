pub struct VM {}

impl VM {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&self, source: &str) -> Result<(), String> {
        println!("Interpreting some code:\n {}", source);
        Ok(())
    }
}

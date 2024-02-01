pub struct Block {
    pub name: String,
    pub instructions: Vec<String>,
}

impl Block {
    pub fn to_string(&self) -> String {
        let mut result = String::new();

        result += &format!(":{}\n", self.name);
        result += &self.instructions.join("\n");

        result
    }
}
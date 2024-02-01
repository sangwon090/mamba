use crate::{irgen::block::Block, lexer::Identifier, types::DataType};

pub struct Function {
    name: String,
    parameters: Vec<(Identifier, DataType)>,
    blocks: Vec<Block>,
    return_type: DataType,
}

impl Function {
    pub fn new(name: String, parameters: Vec<(Identifier, DataType)>, return_type: DataType) -> Function {
        let blocks: Vec<Block> = Vec::new();

        Function {
            name,
            parameters,
            blocks,
            return_type,
        }
    }

    pub fn generate_code(&self) -> String {
        let mut result = String::new();

        // generate header
        result += &format!("define {} @{}(i32 noundef %0) #0 {{",
            self.return_type.to_llvm_type(),
            self.name,
        );

        // generate body
        

        // generate footer
        result += &format!("}}");

        result
    }
}
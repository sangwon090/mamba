use std::collections::HashMap;

use crate::{types::MambaValue, error::CompileError};

pub struct AsmGenerator {
    variables: HashMap<String, MambaValue>,
}

impl AsmGenerator {
    pub fn new() -> AsmGenerator {
        let variables: HashMap<String, MambaValue> = HashMap::new();

        AsmGenerator {
            variables,
        }
    }

    pub fn add_function(&mut self, name: String) -> Result<(), CompileError> {
        use std::fmt::Write;

        let mut result = String::new();
        
        writeln!(result, "{}:", name).unwrap();
        writeln!(result, "push ebp").unwrap();
        writeln!(result, "mov ebp, esp").unwrap();
        // do something
        writeln!(result, "mov esp, ebp").unwrap();
        writeln!(result, "pop ebp").unwrap();
        writeln!(result, "ret").unwrap();
        Ok(())
    }

    pub fn add_variable(&mut self, ident: String, value: MambaValue) -> Result<(), CompileError> {
        if self.variables.contains_key(&ident) {
            Err(CompileError(format!("variable with identifier `{}` already exists", &ident)))
        } else {
            self.variables.insert(ident, value);
            Ok(())
        }
    }

    pub fn generate_asm() -> String {
        "".into()
    }
}
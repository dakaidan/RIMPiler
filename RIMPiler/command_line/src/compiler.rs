use crate::ast::create_ast;

pub struct Compiler {
    input_file: String,
    output_file: String,
}

impl Compiler {
    pub(crate) fn new(input_file: String, output_file: String) -> Compiler {
        Compiler {
            input_file,
            output_file,
        }
    }

    pub(crate) fn compile(&self) -> Result<(), String> {
        let ast = create_ast(&self.input_file);

        if ast.is_err() {
            return Err(ast.unwrap_err().to_string());
        }

        println!("ast: {}", ast.clone().unwrap());

        Ok(())
    }
}
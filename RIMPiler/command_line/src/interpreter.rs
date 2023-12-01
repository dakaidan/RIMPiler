use RIMPiler_frontend::{post_parse::inverter::invert_and_combine, interpreter::InterpreterEngine};
use crate::ast::create_ast;

pub struct Interpreter {
    input_file: String,
}

impl Interpreter {
    pub(crate) fn new(input_file: String) -> Interpreter {
        Interpreter {
            input_file,
        }
    }

    pub(crate) fn interpret(&self) -> Result<(), String> {
        let ast = create_ast(&self.input_file);

        if ast.is_err() {
            return Err(ast.unwrap_err().to_string());
        }

        let inverted = invert_and_combine(&ast.clone().unwrap());

        println!("statements: \n{}", inverted);

        let mut interpreter = InterpreterEngine::new();

        let result = interpreter.interpret(&inverted);

        if result.is_err() {
            return Err(result.unwrap_err().to_string());
        }

        Ok(())
    }
}
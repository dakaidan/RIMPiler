use RIMPiler_backend::java::assemble_byte_code;
use RIMPiler_frontend::Backend;
use RIMPiler_frontend::JVM::code_gen::JVMCompiler;
use RIMPiler_frontend::post_parse::inverter::invert_and_combine;
use super::ast::create_ast;

pub struct Compiler {
    input_file: String,
    output: String,
}

impl Compiler {
    pub(crate) fn new(input_file: String, output: String) -> Compiler {
        Compiler {
            input_file,
            output,
        }
    }

    pub(crate) fn compile(&self) -> Result<(), String> {
        let ast = create_ast(&self.input_file);

        if ast.is_err() {
            return Err(ast.unwrap_err().to_string());
        }

        let ast = ast.unwrap();

        let inverted = invert_and_combine(&ast);

        let byte_code = JVMCompiler::compile(&inverted);

        assemble_byte_code(byte_code, self.output.clone());

        Ok(())
    }
}

use crate::AST::Program;

// compile to jvm from an AST
struct JVMCompiler;

impl JVMCompiler {
    fn compile(&self, program: Program) -> Result<String, String> {
        Ok("".to_string())
    }
}
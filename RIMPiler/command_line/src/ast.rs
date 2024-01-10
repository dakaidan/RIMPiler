use RIMPiler_frontend::lexer::Tokeniser;
use RIMPiler_frontend::parser::parse;
use RIMPiler_frontend::AST::Program;
use utilities::debug::Result;

pub fn create_ast(input_file: &str) -> Result<Program> {
    let input_file_contents = utilities::files::load_file(input_file).unwrap();

    let tokens = Tokeniser::new()
        .initialise()
        .tokenise(input_file_contents)
        .unwrap();

    parse(&mut tokens.into())
}

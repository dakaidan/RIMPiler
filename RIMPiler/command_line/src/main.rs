mod ast;
mod compiler;
mod interpreter;

use utilities::args_parser::*;

fn get_args() -> CommandLineResult {
    let parser = CommandLineArgumentsBuilder::new()
        .add_argument(
            ArgumentBuilder::new("input")
                .short_name("i")
                .long_name("input")
                .string()
                .description("The RIMP file to compile")
                .required(),
        )
        .add_argument(
            ArgumentBuilder::new("output")
                .short_name("o")
                .long_name("output")
                .string()
                .description("The output file")
                .optional(),
        )
        .add_flag(
            FlagBuilder::new("compile")
                .short_name("c")
                .long_name("compile")
                .description("Compile the input file"),
        )
        .add_flag(
            FlagBuilder::new("interpret")
                .short_name("r")
                .long_name("interpret")
                .description("Interpret the input file"),
        )
        .add_flag(
            FlagBuilder::new("llvm")
                .short_name("l")
                .long_name("llvm")
                .description("Compile to LLVM IR"),
        )
        .add_flag(
            FlagBuilder::new("pisa")
                .short_name("p")
                .long_name("pisa")
                .description("Compile to PISA"),
        )
        .build();

    let args = parser.parse();

    match args {
        Ok(args) if args.flags.contains("help") => {
            println!("{}", args);
            std::process::exit(0);
        }
        Ok(args) => {
            if (args.flags.contains("compile")
                || args.flags.contains("llvm")
                || args.flags.contains("pisa"))
                && args.flags.contains("interpret")
            {
                println!("Error: Cannot compile and interpret at the same time");
                println!("{}", parser);
                std::process::exit(1);
            }

            if args.flags.contains("llvm") && args.flags.contains("pisa") {
                println!("Error: Cannot compile to both LLVM IR and PISA");
                println!("{}", parser);
                std::process::exit(1);
            }

            args
        }
        Err(error) => {
            println!("Error: {}", error);
            println!("{}", parser);
            std::process::exit(1);
        }
    }
}

static DEFAULT_OUTPUT_FILE: &str = "a.out";

fn main() {
    let args = get_args();

    let input_file = args.arguments.get("input").unwrap();
    let mut output_file = DEFAULT_OUTPUT_FILE.to_string();

    if args.flags.contains("interpret") {
        let interpreter = interpreter::Interpreter::new(input_file.to_string());
        interpreter.interpret().unwrap();
    } else {
        let output_file_opt = args.arguments.get("output");
        if output_file_opt.is_some() {
            output_file = output_file_opt.unwrap().to_string();
        }
        let compiler = compiler::Compiler::new(input_file.to_string(), output_file.to_string());
        compiler.compile().unwrap();
    }
}

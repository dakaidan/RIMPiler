mod ast;
mod compiler;
mod interpreter;
mod types;
mod abstract_machine;

use utilities::args_parser::*;
use types::Target;

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
                .description("The output file/folder")
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
            FlagBuilder::new("jvm")
                .short_name("j")
                .long_name("jvm")
                .description("Compile to JVM"),
        )
        .add_flag(
            FlagBuilder::new("pisa")
                .short_name("p")
                .long_name("pisa")
                .description("Compile to PISA"),
        )
        .add_flag(
            FlagBuilder::new("abstract")
                .short_name("a")
                .long_name("abstract")
                .description("Compile to abstract machine"),
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
                || args.flags.contains("pisa")
                || args.flags.contains("jvm"))
                && (args.flags.contains("interpret")
                || args.flags.contains("abstract"))
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

            if args.flags.contains("llvm") && args.flags.contains("jvm") {
                println!("Error: Cannot compile to both LLVM IR and JVM");
                println!("{}", parser);
                std::process::exit(1);
            }

            if args.flags.contains("jvm") && args.flags.contains("pisa") {
                println!("Error: Cannot compile to both JVM and PISA");
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

static DEFAULT_OUTPUT_FILE: &str = "Main";

fn main() {
    let args = get_args();

    let input_file = args.arguments.get("input").unwrap();
    let mut output_file = DEFAULT_OUTPUT_FILE.to_string();

    if args.flags.contains("interpret") || args.flags.contains("abstract") {
        if args.flags.contains("abstract") {
            let abstract_machine = abstract_machine::AbstractMachine::new(input_file.to_string());
            abstract_machine.run().unwrap();
        } else {
            let interpreter = interpreter::Interpreter::new(input_file.to_string());
            interpreter.interpret().unwrap();
        }
    } else {
        let output_file_opt = args.arguments.get("output");
        if output_file_opt.is_some() {
            output_file = output_file_opt.unwrap().to_string();
        }
        let target = if args.flags.contains("llvm") {
            Target::LLVM
        } else if args.flags.contains("jvm") {
            Target::JVM
        } else {
            Target::JVM
        };
        let compiler = compiler::Compiler::new(input_file.to_string(), output_file.to_string(), target);
        compiler.compile().unwrap();
    }
}

mod ast;
mod compiler;
mod interpreter;
mod abstract_machine;

use utilities::args_parser::*;

const LOGO: &str = r#"
██████╗ ██╗███╗   ███╗██████╗ ██╗██╗     ███████╗██████╗
██╔══██╗██║████╗ ████║██╔══██╗██║██║     ██╔════╝██╔══██╗
██████╔╝██║██╔████╔██║██████╔╝██║██║     █████╗  ██████╔╝
██╔══██╗██║██║╚██╔╝██║██╔═══╝ ██║██║     ██╔══╝  ██╔══██╗
██║  ██║██║██║ ╚═╝ ██║██║     ██║███████╗███████╗██║  ██║
╚═╝  ╚═╝╚═╝╚═╝     ╚═╝╚═╝     ╚═╝╚══════╝╚══════╝╚═╝  ╚═╝
"#;

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
            FlagBuilder::new("abstract machine")
                .short_name("m")
                .long_name("abstract")
                .description("Run the abstract machine"),
        )
        .add_flag(
            FlagBuilder::new("help")
                .short_name("h")
                .long_name("help")
                .description("Prints the help message"),
        )
        .build();

    let args = parser.parse();

    match args {
        Ok(args) if args.flags.contains("help") => {
            println!("{}", LOGO);
            println!("{}", parser);
            std::process::exit(0);
        }
        Ok(args) => {
            if (args.flags.contains("compile") && args.flags.contains("interpret"))
                || (args.flags.contains("compile") && args.flags.contains("abstract machine"))
                || (args.flags.contains("interpret") && args.flags.contains("abstract machine"))
            {
                println!("{}", LOGO);
                println!("Error: Only one of the flags compile, interpret, or abstract machine can be used at a time");
                println!();
                println!("{}", parser);
                std::process::exit(1);
            } else if args.arguments.get("output").is_none()
                && !args.flags.contains("abstract machine")
                && !args.flags.contains("interpret") {
                println!("{}", LOGO);
                println!("Error: The output flag is required when compiling a file");
                println!();
                println!("{}", parser);
                std::process::exit(1);
            } else if args.arguments.get("output").is_some()
                && (args.flags.contains("abstract machine") || args.flags.contains("interpret")) {
                println!("{}", LOGO);
                println!("Error: The output flag is only required when running the compiler");
                println!();
                println!("{}", parser);
                std::process::exit(1);
            } else {
                args
            }
        }
        Err(error) => {
            println!("{}", LOGO);
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

    if args.flags.is_empty() || args.flags.contains("compile") {
        let output_file_opt = args.arguments.get("output");
        if output_file_opt.is_some() {
            output_file = output_file_opt.unwrap().to_string();
        }

        let compiler = compiler::Compiler::new(input_file.to_string(), output_file.to_string());
        compiler.compile().unwrap();
    } else if args.flags.contains("abstract machine") {
        let abstract_machine = abstract_machine::AbstractMachine::new(input_file.to_string());
        abstract_machine.run().unwrap();
    } else if args.flags.contains("interpret") {
        let interpreter = interpreter::Interpreter::new(input_file.to_string());
        interpreter.interpret().unwrap();
    } else {
        unreachable!("Invalid flag combination")
    }
}

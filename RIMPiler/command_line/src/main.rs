use utilities::args_parser::{*};
fn get_args() -> CommandLineResult {
    let parser = CommandLineArgumentsBuilder::new()
        .add_argument(
            ArgumentBuilder::new("input")
                .short_name("i")
                .long_name("input")
                .string()
                .description("The RIMP file to compile")
                .required()
        )
        .build();

    let args = parser.parse();

    match args {
        Ok(args) if args.flags.contains("help") => {
            println!("{}", args);
            std::process::exit(0);
        }
        Ok(args) => {
            args
        }
        Err(error) => {
            println!("Error: {}", error);
            println!("{}", parser);
            std::process::exit(1);
        }
    }
}

fn main() {
    let args = get_args();

    println!("Args: {}", args);
}
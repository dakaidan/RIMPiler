use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CommandLineArgumentType {
    String,
    Integer,
}

impl Display for CommandLineArgumentType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            CommandLineArgumentType::String => write!(f, "String"),
            CommandLineArgumentType::Integer => write!(f, "Integer"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Identifier {
    name: String,
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Names {
    short: String,
    long: String,
}

impl Display for Names {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "-{} or --{}", self.short, self.long)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Required {
    Required,
    Optional,
}

impl Display for Required {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Required::Required => write!(f, "required"),
            Required::Optional => write!(f, "optional"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Description {
    description: String,
}

impl Display for Description {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.description)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CommandLineSpecification {
    Flag(Identifier, Names, Description),
    Argument(Identifier, Names, CommandLineArgumentType, Required, Description),
}

impl Display for CommandLineSpecification {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            CommandLineSpecification::Flag(identifier, names, description) => {
                write!(f, "{}: {} ({})", identifier, names, description)
            }
            CommandLineSpecification::Argument(identifier, names, argument_type, required, description) => {
                write!(f, "{}: {} ({}, {})\t\t{}", identifier, names, argument_type, required, description)
            }
        }
    }
}

pub struct ArgumentBuilder {
    identifier: Identifier,
    short_name: String,
    long_name: String,
    argument_type: Option<CommandLineArgumentType>,
    required: bool,
    description: String
}

impl ArgumentBuilder {
    pub fn new(identifier: &str) -> Self {
        ArgumentBuilder {
            identifier: Identifier { name: identifier.to_string() },
            short_name: String::new(),
            long_name: String::new(),
            argument_type: None,
            required: false,
            description: String::new(),
        }
    }

    pub fn short_name(mut self, short_name: &str) -> Self {
        self.short_name = short_name.to_string();
        self
    }

    pub fn long_name(mut self, long_name: &str) -> Self {
        self.long_name = long_name.to_string();
        self
    }

    pub fn string(mut self) -> Self {
        self.argument_type = Some(CommandLineArgumentType::String);
        self
    }

    pub fn integer(mut self) -> Self {
        self.argument_type = Some(CommandLineArgumentType::Integer);
        self
    }

    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    pub fn optional(mut self) -> Self {
        self.required = false;
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    pub fn build(self) -> CommandLineSpecification {
        if self.short_name.is_empty() || self.long_name.is_empty() {
            panic!("Cannot build command line specification: short name and long name must be specified.");
        }
        if self.argument_type.is_none() {
            panic!("Cannot build command line specification: argument type must be specified.");
        }
        if self.description.is_empty() {
            panic!("Cannot build command line specification: description must be specified.");
        }

        CommandLineSpecification::Argument(
            self.identifier,
            Names { short: self.short_name, long: self.long_name },
            self.argument_type.unwrap(),
            if self.required { Required::Required } else { Required::Optional },
            Description { description: self.description }
        )
    }
}

pub struct FlagBuilder {
    identifier: Identifier,
    short_name: String,
    long_name: String,
    description: String
}

impl FlagBuilder {
    pub fn new(identifier: &str) -> Self {
        FlagBuilder {
            identifier: Identifier { name: identifier.to_string() },
            short_name: String::new(),
            long_name: String::new(),
            description: String::new(),
        }
    }

    pub fn short_name(mut self, short_name: &str) -> Self {
        self.short_name = short_name.to_string();
        self
    }

    pub fn long_name(mut self, long_name: &str) -> Self {
        self.long_name = long_name.to_string();
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    pub fn build(self) -> CommandLineSpecification {
        if self.short_name.is_empty() || self.long_name.is_empty() {
            panic!("Cannot build command line specification: short name and long name must be specified.");
        }
        if self.description.is_empty() {
            panic!("Cannot build command line specification: description must be specified.");
        }

        CommandLineSpecification::Flag(
            self.identifier,
            Names { short: self.short_name, long: self.long_name },
            Description { description: self.description }
        )
    }
}

pub struct CommandLineArguments {
    specifications: HashMap<Identifier, CommandLineSpecification>
}

impl Display for CommandLineArguments {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut output = String::new();
        for specification in self.specifications.values() {
            match specification {
                CommandLineSpecification::Argument(_, _, _, _, _) => {
                    output.push_str(&format!("{}", specification));
                }
                CommandLineSpecification::Flag(_, _, _) => {
                    output.push_str(&format!("{}", specification));
                }
            }
        }
        write!(f, "{}", output)
    }
}

pub struct CommandLineArgumentsBuilder {
    specifications: HashMap<Identifier, CommandLineSpecification>
}

impl CommandLineArgumentsBuilder {
    pub fn new() -> Self {
        CommandLineArgumentsBuilder {
            specifications: HashMap::new()
        }
    }

    pub fn add_argument(mut self, argument: ArgumentBuilder) -> Self {
        let specification = argument.build();
        if let CommandLineSpecification::Argument(identifier, _, _, _, _) = &specification {
            self.specifications.insert(identifier.clone(), specification);
        } else {
            panic!("Cannot add argument: specification is not an argument.");
        }
        self
    }

    pub fn add_flag(mut self, flag: FlagBuilder) -> Self {
        let specification = flag.build();
        if let CommandLineSpecification::Flag(identifier, _, _) = &specification {
            self.specifications.insert(identifier.clone(), specification);
        } else {
            panic!("Cannot add flag: specification is not a flag.");
        }
        self
    }

    pub fn add_help(mut self) -> Self {
        self = self.add_flag(
            FlagBuilder::new("help")
                .short_name("h")
                .long_name("help")
                .description("Prints this help message.")
        );
        self
    }

    pub fn build(self) -> CommandLineArguments {
        CommandLineArguments {
            specifications: self.specifications
        }
    }
}

pub struct CommandLineResult {
    pub arguments: HashMap<String, String>,
    pub flags: HashSet<String>
}

impl Display for CommandLineResult {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut output = String::new();
        for (identifier, value) in self.arguments.iter() {
            output.push_str(&format!("{}: {}\n", identifier, value));
        }
        for identifier in self.flags.iter() {
            output.push_str(&format!("{}\n", identifier));
        }
        write!(f, "{}", output)
    }
}

impl CommandLineArguments {
    fn get_specification_by_name(&self, name: &str) -> Option<&CommandLineSpecification> {
        for specification in self.specifications.values() {
            match specification {
                CommandLineSpecification::Argument(_, names, _, _, _) => {
                    if names.long == name || names.short == name {
                        return Some(specification);
                    }
                }
                CommandLineSpecification::Flag(_, names, _) => {
                    if names.long == name || names.short == name {
                        return Some(specification);
                    }
                }
            }
        }
        None
    }

    pub fn parse(&self) -> Result<CommandLineResult, String> {
        let mut result = CommandLineResult {
            arguments: HashMap::new(),
            flags: HashSet::new()
        };

        let mut args = std::env::args();
        args.next(); // Skip the first argument, which is the name of the program.

        while let Some(arg) = args.next() {
            if arg.starts_with("--") {
                let identifier = arg[2..].to_string();
                if let Some(specification) = self.get_specification_by_name(&identifier) {
                    match specification {
                        CommandLineSpecification::Argument(id, _, argument_type, _, _) => {
                            if let Some(value) = args.next() {
                                match argument_type {
                                    CommandLineArgumentType::String => {
                                        result.arguments.insert(id.to_string(), value);
                                    }
                                    CommandLineArgumentType::Integer => {
                                        match value.parse::<i32>() {
                                            Ok(value) => {
                                                result.arguments.insert(identifier, value.to_string());
                                            }
                                            Err(_) => {
                                                return Err(format!("Error: argument {} must be an integer.", identifier));
                                            }
                                        }
                                    }
                                }
                            } else {
                                return Err(format!("Error: argument {} must be followed by a value.", identifier));
                            }
                        }
                        CommandLineSpecification::Flag(id, _, _) => {
                            result.flags.insert(id.to_string());
                        }
                    }
                } else {
                    return Err(format!("Error: unknown argument {}.", identifier));
                }
            } else if arg.starts_with("-") {
                let identifier = arg[1..].to_string();
                if let Some(specification) = self.get_specification_by_name(&identifier) {
                    match specification {
                        CommandLineSpecification::Argument(id, _, argument_type, _, _) => {
                            if let Some(value) = args.next() {
                                match argument_type {
                                    CommandLineArgumentType::String => {
                                        result.arguments.insert(id.to_string(), value);
                                    }
                                    CommandLineArgumentType::Integer => {
                                        match value.parse::<i32>() {
                                            Ok(value) => {
                                                result.arguments.insert(identifier, value.to_string());
                                            }
                                            Err(_) => {
                                                return Err(format!("Error: argument {} must be an integer.", identifier));
                                            }
                                        }
                                    }
                                }
                            } else {
                                return Err(format!("Error: argument {} must be followed by a value.", identifier));
                            }
                        }
                        CommandLineSpecification::Flag(id, _, _) => {
                            result.flags.insert(id.to_string());
                        }
                    }
                } else {
                    return Err(format!("Error: unknown argument {}.", identifier));
                }
            } else {
                return Err(format!("Error: unknown argument {}.", arg));
            }
        }

        for (identifier, specification) in self.specifications.iter() {
            match specification {
                CommandLineSpecification::Argument(_, _, _, required, _) => {
                    if *required == Required::Required {
                        if !result.arguments.contains_key(&identifier.name) {
                            return Err(format!("Error: argument {} is required.", identifier));
                        }
                    }
                }
                CommandLineSpecification::Flag(_, _, _) => {}
            }
        }

        Ok(result)
    }
}
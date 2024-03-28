use std::io::Write;
use RIMPiler_frontend::abstract_machine::engine::Engine;
use super::ast::create_ast_without_transform;

pub struct AbstractMachine {
    input_file: String,
}

impl AbstractMachine {
    pub(crate) fn new(input_file: String) -> AbstractMachine {
        AbstractMachine { input_file }
    }

    pub(crate) fn run(&self) -> Result<(), String> {
        let ast = create_ast_without_transform(&self.input_file);

        if ast.is_err() {
            return Err(ast.unwrap_err().to_string());
        }

        let mut engine = Engine::new(ast.unwrap());

        println!("control stack: \n{}", engine.get_control_stack());

        let mut is_forward = true;

        loop {
            if engine.is_done() {
                if is_forward {
                    println!("Program has finished executing");
                } else {
                    println!("Program has been rewound to the beginning");
                }
            }

            print!("> ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            match input {
                "step" | "s" => {
                    if engine.is_done() {
                        println!("Program has finished executing, reverse to continue");
                        continue;
                    }
                    engine.step();
                }
                "back" | "b" => {
                    engine.reverse();
                    if engine.is_done() {
                        println!("Program has been rewound to the beginning");
                        engine.reverse();
                        continue;
                    }
                    engine.step();
                    engine.reverse();
                }
                "print control stack" | "pcs" => {
                    println!("control stack: \n{}", engine.get_control_stack());
                }
                "print back stack" | "pbs" => {
                    println!("back stack: \n{}", engine.get_back_stack());
                }
                "print result stack" | "prs" => {
                    println!("result stack: \n{}", engine.get_result_stack());
                }
                "print store" | "ps" => {
                    println!("store: \n{}", engine.get_store());
                }
                "print all" | "pa" => {
                    println!("control stack: \n{}", engine.get_control_stack());
                    println!("result stack: \n{}", engine.get_result_stack());
                    println!("store: \n{}", engine.get_store());
                    println!("back stack: \n{}", engine.get_back_stack());
                }
                "step and print" | "sp" => {
                    if engine.is_done() {
                        println!("Program has finished executing, reverse to continue");
                        continue;
                    }
                    engine.step();
                    println!("control stack: \n{}", engine.get_control_stack());
                    println!("result stack: \n{}", engine.get_result_stack());
                    println!("store: \n{}", engine.get_store());
                    println!("back stack: \n{}", engine.get_back_stack());
                }
                "run" | "r" => {
                    while !engine.is_done() {
                        engine.step();
                    }
                }
                "run and print" | "rp" => {
                    while !engine.is_done() {
                        engine.step();
                        println!("control stack: \n{}", engine.get_control_stack());
                        println!("result stack: \n{}", engine.get_result_stack());
                        println!("store: \n{}", engine.get_store());
                        println!("back stack: \n{}", engine.get_back_stack());
                        println!("");
                    }
                }
                "run print reverse" | "rpr" => {
                    while !engine.is_done() {
                        engine.step();
                        println!("control stack: \n{}", engine.get_control_stack());
                        println!("result stack: \n{}", engine.get_result_stack());
                        println!("store: \n{}", engine.get_store());
                        println!("back stack: \n{}", engine.get_back_stack());
                    }

                    engine.reverse();
                    is_forward = !is_forward;

                    while !engine.is_done() {
                        engine.step();
                        println!("control stack: \n{}", engine.get_control_stack());
                        println!("result stack: \n{}", engine.get_result_stack());
                        println!("store: \n{}", engine.get_store());
                        println!("back stack: \n{}", engine.get_back_stack());
                    }
                }
                "direction" | "d" => {
                    if is_forward {
                        println!("Direction: forward");
                    } else {
                        println!("Direction: backward");
                    }
                }
                "reverse" | "rv" => {
                    engine.reverse();
                    is_forward = !is_forward;
                }
                "quit" | "q" => {
                    break;
                }
                c => {
                    if c.is_empty() {
                        continue;
                    }

                    if c != "help" && c != "h" {
                        println!("Invalid command: {}", c);
                    }

                    println!("Commands:");
                    println!("step (s) - step through the program");
                    println!("back (b) - step backwards through the program once");
                    println!("print control stack (pcs) - print the control stack");
                    println!("print back stack (pbs) - print the back stack");
                    println!("print result stack (prs) - print the result stack");
                    println!("print store (ps) - print the store");
                    println!("print all (pa) - print all stacks and the store");
                    println!("step and print (sp) - step through the program and print all stacks and the store");
                    println!("run (r) - run the program to completion");
                    println!("run and print (rp) - run the program to completion and print all stacks and the store");
                    println!("run print reverse (rpr) - run the program to completion, reverse, then repeat");
                    println!("direction (d) - check the direction of execution");
                    println!("reverse (rv) - reverse the direction of execution");
                    println!("quit (q) - quit the program");
                    println!("help (h) - print this help message");
                }
            }
        }

        Ok(())
    }
}
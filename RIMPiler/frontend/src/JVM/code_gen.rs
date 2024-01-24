use std::collections::HashMap;
use std::io::Read;
use crate::AST::{ArithmeticExpression, ArithmeticOperator, Assignment, Block, BooleanExpression, BooleanOperator, Program, RelationOperator, Statement, UnaryArithmeticOperator, UnaryBooleanOperator};
use crate::Backend;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum Type {
    Integer,
}

fn compile_relational_operator(operator: &RelationOperator) -> String {
    match operator {
        RelationOperator::Equal => "if_icmpne".to_string(),
        RelationOperator::NotEqual => "if_icmpeq".to_string(),
        RelationOperator::LessThan => "if_icmpge".to_string(),
        RelationOperator::GreaterThan => "if_icmple".to_string(),
    }
}

fn compile_arithmetic_operation(operator: &ArithmeticOperator) -> String {
    match operator {
        ArithmeticOperator::Addition => String::from("iadd"),
        ArithmeticOperator::Subtraction => String::from("isub"),
        ArithmeticOperator::Multiplication => String::from("imul"),
        ArithmeticOperator::Division => String::from("idiv"),
        ArithmeticOperator::Exponentiation => unreachable!("Exponentiation is not supported by jvm"),
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct JVMCompiler {
    // variable name -> (index, type)
    variables: HashMap<String, (usize, Type)>,
    // index used by the last variable
    last_variable_index: usize,

    // index for fresh labels
    index: usize,

    // stack size
    max_stack: usize,
    current_stack: usize,
}

impl JVMCompiler {
    // Stack is less optimal to ensure it is more than enough, could reorder increments/decrements to be more optimal
    pub fn new() -> Self {
        JVMCompiler {
            variables: HashMap::new(),
            last_variable_index: 0,
            index: 0,
            max_stack: 0,
            current_stack: 0,
        }
    }

    fn increment_stack(&mut self) {
        self.current_stack += 1;
        if self.current_stack > self.max_stack {
            self.max_stack = self.current_stack;
        }
    }

    fn decrement_stack(&mut self) {
        self.current_stack -= 1;
    }

    fn new_label(&mut self, prefix: &str) -> String {
        let label = format!("L{}{}", prefix, self.index);
        self.index += 1;
        label
    }

    fn load_variable(&mut self, variable: &String) -> String {
        self.increment_stack();
        self.increment_stack();
        self.decrement_stack();

        let index = self.variables.get(variable);

        if index.is_none() {
            panic!("Variable {} used before assignment", variable);
        }

        let (index, _) = index.unwrap();

        format!("aload {}\ninvokevirtual Method RIMPInt get ()I\n", index)
    }

    pub fn _compile(&mut self, program: &Program) -> String {
        let content = self.compile_program(program);
        let mut file = std::fs::File::open("./data/Main.j").unwrap();

        let mut file_content = String::new();
        file.read_to_string(&mut file_content).unwrap();

        let content = self.compile_assignments() + &content;

        let file_content = file_content.replace("<code>", &content);

        let max_stack = self.max_stack + 1;
        let max_locals = self.last_variable_index + 1;

        let file_content = file_content.replace("<stack>", &max_stack.to_string()).replace("<locals>", &max_locals.to_string());

        file_content
    }

    fn compile_assignments(&self) -> String {
        let mut variables: Vec<(&String, &(usize, Type))> = self.variables.iter().collect();
        variables.sort_by(|(_, (index1, _)), (_, (index2, _))| index1.cmp(index2));

        let mut code = String::new();
        for (variable, (index, _)) in variables {
            code.push_str(&format!("new RIMPInt\ndup\nldc \"{}\"\ninvokespecial Method RIMPInt <init> (Ljava/lang/String;)V\nastore {}\n", variable, index));
        }
        code
    }

    fn compile_program(&mut self, program: &Program) -> String {
        match program {
            Program::Statements(block) => self.compile_block(block),
        }
    }

    fn compile_block(&mut self, block: &Block) -> String {
        block.iter().map(|statement| self.compile_statement(statement)).collect()
    }

    fn compile_statement(&mut self, statement: &Statement) -> String {
        match statement {
            Statement::Skip => String::from(""),
            Statement::If(condition, then_block, else_block) => {
                self.compile_if(condition, then_block, else_block)
            }
            Statement::While(condition, block) => {
                self.compile_while(condition, block)
            }
            Statement::Assignment(assignment) => {
                self.compile_assignment(assignment)
            }
            Statement::ReverseAssignment(assignment) => {
                self.compile_reverse_assignment(assignment)
            }
            Statement::ReversePoint => {
                self.insert_reverse_point()
            }
        }
    }

    fn insert_reverse_point(&mut self) -> String {
        let mut code = String::new();
        self.increment_stack();
        self.increment_stack();
        self.decrement_stack();
        self.decrement_stack();
        for (_, (index, _)) in self.variables.iter() {
            code.push_str(&format!("aload {}\ninvokevirtual Method RIMPInt print ()V\n", index));
        }
        code
    }

    fn compile_if(&mut self, condition: &BooleanExpression, then_block: &Block, else_block: &Block) -> String {
        let else_label = self.new_label("ELSE");
        let end_label = self.new_label("ENDELSE");

        let condition_code = self.compile_boolean_expression(condition, &else_label);
        let then_code = self.compile_block(then_block);
        let else_code = self.compile_block(else_block);

        format!("{}{}goto {}\n{}:\n{}\n{}:\n", condition_code, then_code, end_label, else_label, else_code, end_label)
    }

    fn compile_while(&mut self, condition: &BooleanExpression, block: &Block) -> String {
        let start_label = self.new_label("START");
        let end_label = self.new_label("ENDLOOP");

        let condition_code = self.compile_boolean_expression(condition, &end_label);
        let block_code = self.compile_block(block);

        format!("{}:\n{}{}goto {}\n{}:\n", start_label, condition_code, block_code, start_label, end_label)
    }

    fn compile_assignment(&mut self, assignment: &Assignment) -> String {
        // CURRENTLY ASSUMING ONLY TYPE IS INTEGERS
        let Assignment::Integer(variable, expression) = assignment;
        self.increment_stack();
        self.decrement_stack();

        let expr_code = self.compile_arithmetic_expression(expression);

        let var = self.variables.get(variable);

        if var.is_none() {
            self.last_variable_index += 1;
            self.variables.insert(variable.clone(), (self.last_variable_index, Type::Integer));
            format!("aload {}\n{}invokevirtual Method RIMPInt assign (I)V\n", self.last_variable_index, expr_code)
        } else {
            let (index, _) = var.unwrap();
            format!("aload {}\n{}invokevirtual Method RIMPInt assign (I)V\n", index, expr_code)
        }
    }

    fn compile_reverse_assignment(&self, assignment: &Assignment) -> String {
        let Assignment::Integer(variable, _) = assignment;

        let var = self.variables.get(variable);

        if var.is_none() {
            panic!("Variable {} being unassigned before assignment", variable);
        }

        let (index, _) = var.unwrap();

        format!("aload {}\ninvokevirtual Method RIMPInt unAssign ()V\n", index)
    }

    fn compile_arithmetic_expression(&mut self, arithmetic_expression: &ArithmeticExpression) -> String {
        // CURRENTLY ASSUMING ONLY TYPE IS INTEGERS
        match arithmetic_expression {
            ArithmeticExpression::Variable(variable) => {
                self.load_variable(variable)
            }
            ArithmeticExpression::Integer(value) => {
                self.increment_stack();
                format!("ldc {}\n", value)
            }
            ArithmeticExpression::Unary(operator, expression) => {
                match operator {
                    UnaryArithmeticOperator::Negation => {
                        let expr_code = self.compile_arithmetic_expression(expression);
                        self.increment_stack();
                        self.decrement_stack();
                        format!("{}ineg\n", expr_code)
                    }
                }
            }
            ArithmeticExpression::Operation(operator, left, right) => {
                let lhs = self.compile_arithmetic_expression(left);
                let rhs = self.compile_arithmetic_expression(right);
                match operator {
                    ArithmeticOperator::Exponentiation => {
                        self.decrement_stack();
                        format!("{}{}invokestatic java/lang/Math/pow(DD)D\n", lhs, rhs)
                    }
                    _ => {
                        let operator = compile_arithmetic_operation(operator);
                        format!("{}{}{}\n", lhs, rhs, operator)
                    }
                }
            }
        }
    }

    fn compile_boolean_expression(&mut self, boolean_expression: &BooleanExpression, jump_if_false: &String) -> String {
        match boolean_expression {
            BooleanExpression::Unary(operator, expression) => {
                self.compile_boolean_unary(operator, expression, jump_if_false)
            }
            BooleanExpression::Logical(operator, left, right) => {
                match operator {
                    BooleanOperator::And => {
                        let lhs = self.compile_boolean_expression(left, jump_if_false);
                        let rhs = self.compile_boolean_expression(right, jump_if_false);
                        format!("{}{}\n", lhs, rhs)
                    }
                    BooleanOperator::Or => {
                        let next_or_label = self.new_label("OR");
                        let lhs = self.compile_boolean_expression(left, &next_or_label);
                        let rhs = self.compile_boolean_expression(right, jump_if_false);

                        format!("{}{}:\n{}\n", lhs, next_or_label, rhs)
                    }
                }
            }
            BooleanExpression::Relational(operator, left, right) => {
                let lhs = self.compile_arithmetic_expression(left);
                let rhs = self.compile_arithmetic_expression(right);

                let operator = compile_relational_operator(operator);
                self.decrement_stack();
                self.decrement_stack();

                format!("{}\n{}\n{} {}\n", lhs, rhs, operator, jump_if_false)
            }
        }
    }

    fn compile_boolean_unary(&mut self, operator: &UnaryBooleanOperator, expression: &BooleanExpression, jump_if_false: &String) -> String {
        match operator {
            UnaryBooleanOperator::Negation => {
                let expr_code = self.compile_boolean_expression(expression, jump_if_false);
                self.decrement_stack();

                format!("{}ifeq {}\n", expr_code, jump_if_false)
            }
        }
    }
}

impl Backend for JVMCompiler {
    fn compile(program: &Program) -> String {
        let mut compiler = JVMCompiler::new();
        compiler._compile(program)
    }
}

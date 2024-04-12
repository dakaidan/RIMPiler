use super::super::AST::{
    ArithmeticExpression, Block, BooleanExpression, Program, RelationOperator,
    Statement, Variable
};

pub fn invert_and_combine(program: &Program) -> Program {
    let inverted = invert(program);
    combine(program, &inverted)
}

pub fn combine(forward: &Program, backward: &Program) -> Program {
    let Program::Statements(forward_statements) = forward;
    let Program::Statements(backward_statements) = backward;

    let mut statements = Vec::new();
    statements.extend(forward_statements.clone());
    statements.push(Statement::ReversePoint);
    statements.extend(backward_statements.clone());

    Program::Statements(statements)
}

pub fn invert(program: &Program) -> Program {
    let mut inverter = Inverter::new();
    inverter.invert_program(program)
}

struct Inverter {
    last_variable_name: Option<String>,
    last_variable_type: Option<String>,
}

impl Inverter {
    pub fn new() -> Inverter {
        Inverter {
            last_variable_name: None,
            last_variable_type: None,
        }
    }

    fn invert_program(&mut self, program: &Program) -> Program {
        match program {
            Program::Statements(statements) => {
                let mut new_statements = Vec::with_capacity(statements.len());

                for _ in 0..statements.len() {
                    new_statements.push(Statement::Skip);
                }

                for (index, statement) in statements.iter().enumerate() {
                    if let Statement::Assignment(variable, _) = statement {
                        match variable {
                            Variable::Integer(variable) => {
                                self.last_variable_name = Some(variable.clone());
                                self.last_variable_type = Some("int".to_string());
                            }
                            Variable::Float(variable) => {
                                self.last_variable_name = Some(variable.clone());
                                self.last_variable_type = Some("float".to_string());
                            }
                        }
                    }
                    new_statements[statements.len() - index - 1] = self.invert_statement(statement);
                }

                Program::Statements(new_statements)
            }
        }
    }

    fn invert_statement(&mut self, statement: &Statement) -> Statement {
        match statement {
            Statement::If(boolean_expression, if_block, else_block) => {
                self.invert_if_statement(boolean_expression, if_block, else_block)
            }
            Statement::While(boolean_expression, block) => {
                self.invert_while_statement(boolean_expression, block)
            }
            Statement::Assignment(variable, expression) => self.invert_assignment(variable, expression),
            Statement::Skip => Statement::Skip,
            Statement::ReverseAssignment(variable, expression) => Statement::Assignment(variable.clone(), expression.clone()),
            Statement::ReversePoint => Statement::ReversePoint,
        }
    }

    fn invert_if_statement(
        &mut self,
        boolean_expression: &BooleanExpression,
        if_block: &Block,
        else_block: &Block,
    ) -> Statement {
        Statement::If(
            Box::new(boolean_expression.clone()),
            Box::new(self.invert_block(if_block)),
            Box::new(self.invert_block(else_block)),
        )
    }

    fn invert_while_statement(
        &mut self,
        _: &BooleanExpression,
        block: &Block,
    ) -> Statement {
        if self.last_variable_name.is_none() {
            unreachable!(
                "This should only be called on an AST that has gone through semantic transformations"
            )
        }

        match self.last_variable_type.clone().unwrap().as_str() {
            "int" => Statement::While(
                Box::new(BooleanExpression::Relational(
                    RelationOperator::GreaterThan,
                    Box::new(ArithmeticExpression::Variable(
                        Variable::Integer(self.last_variable_name.clone().unwrap())
                    )),
                    Box::new(ArithmeticExpression::Integer(0)),
                )),
                Box::new(self.invert_block(block)),
            ),
            _ => unreachable!("This should only be called on an AST that has gone through semantic transformations")
        }

    }

    fn invert_assignment(&mut self, variable: &Variable, expression: &ArithmeticExpression) -> Statement {
        Statement::ReverseAssignment(variable.clone(), expression.clone())
    }

    fn invert_block(&mut self, block: &Block) -> Block {
        let mut new_block = Vec::with_capacity(block.len());
        for _ in 0..block.len() {
            new_block.push(Statement::Skip);
        }

        for (index, statement) in block.iter().enumerate() {
            if let Statement::Assignment(variable, _) = statement {
                match variable {
                    Variable::Integer(variable) => {
                        self.last_variable_name = Some(variable.clone());
                        self.last_variable_type = Some("int".to_string());
                    }
                    Variable::Float(variable) => {
                        self.last_variable_name = Some(variable.clone());
                        self.last_variable_type = Some("float".to_string());
                    }
                }
            }
            new_block[block.len() - index - 1] = self.invert_statement(statement);
        }

        new_block
    }
}

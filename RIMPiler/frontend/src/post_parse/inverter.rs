use ordered_float::NotNan;
use crate::AST::Variable;
use super::super::AST::{
    ArithmeticExpression, Block, BooleanExpression, Program, RelationOperator,
    Statement,
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
    invert_program(program)
}

fn invert_program(program: &Program) -> Program {
    match program {
        Program::Statements(statements) => {
            let mut new_statements = Vec::with_capacity(statements.len());

            // TODO: Fix this complete mess
            for _ in 0..statements.len() {
                new_statements.push(Statement::Skip);
            }

            let mut last_variable_name = None;
            let mut last_variable_type = None;

            for (index, statement) in statements.iter().enumerate() {
                if let Statement::Assignment(variable, _) = statement {
                    match variable {
                        Variable::Integer(variable) => {
                            last_variable_name = Some(variable.clone());
                            last_variable_type = Some("int".to_string());
                        }
                        Variable::Float(variable) => {
                            last_variable_name = Some(variable.clone());
                            last_variable_type = Some("float".to_string());
                        }
                    }
                }
                new_statements[statements.len() - index - 1] = invert_statement(statement, &last_variable_name, &last_variable_type);
            }

            Program::Statements(new_statements)
        }
    }
}

fn invert_statement(statement: &Statement, last_variable_name: &Option<String>, last_variable_type: &Option<String>) -> Statement {
    match statement {
        Statement::If(boolean_expression, if_block, else_block) => {
            invert_if_statement(boolean_expression, if_block, else_block)
        }
        Statement::While(boolean_expression, block) => {
            invert_while_statement(boolean_expression, block, last_variable_name, last_variable_type)
        }
        Statement::Assignment(variable, expression) => invert_assignment(variable, expression),
        Statement::Skip => Statement::Skip,
        Statement::ReverseAssignment(variable, expression) => Statement::Assignment(variable.clone(), expression.clone()),
        Statement::ReversePoint => Statement::ReversePoint,
    }
}

fn invert_if_statement(
    boolean_expression: &BooleanExpression,
    if_block: &Block,
    else_block: &Block,
) -> Statement {
    Statement::If(
        Box::new(boolean_expression.clone()),
        Box::new(invert_block(if_block)),
        Box::new(invert_block(else_block)),
    )
}

fn invert_while_statement(
    _: &BooleanExpression,
    block: &Block,
    last_variable_name: &Option<String>,
    last_variable_type: &Option<String>
) -> Statement {
    if last_variable_name.is_none() {
        unreachable!(
            "This should only be called on an AST that has gone through semantic transformations"
        )
    }

    match last_variable_type.clone().unwrap().as_str() {
        "int" => Statement::While(
                    Box::new(BooleanExpression::Relational(
                        RelationOperator::GreaterThan,
                        Box::new(ArithmeticExpression::Variable(
                            Variable::Integer(last_variable_name.clone().unwrap())
                        )),
                        Box::new(ArithmeticExpression::Integer(0)),
                    )),
                    Box::new(invert_block(block)),
                ),
        _ => unreachable!("This should only be called on an AST that has gone through semantic transformations")
    }

}

fn invert_assignment(variable: &Variable, expression: &ArithmeticExpression) -> Statement {
    Statement::ReverseAssignment(variable.clone(), expression.clone())
}

fn invert_block(block: &Block) -> Block {
    let mut new_block = Vec::with_capacity(block.len());
    // TODO: And this one
    for _ in 0..block.len() {
        new_block.push(Statement::Skip);
    }
    let mut last_variable_name = None;
    let mut last_variable_type = None;

    for (index, statement) in block.iter().enumerate() {
        if let Statement::Assignment(variable, _) = statement {
            match variable {
                Variable::Integer(variable) => {
                    last_variable_name = Some(variable.clone());
                    last_variable_type = Some("int".to_string());
                }
                Variable::Float(variable) => {
                    last_variable_name = Some(variable.clone());
                    last_variable_type = Some("float".to_string());
                }
            }
        }
        new_block[block.len() - index - 1] = invert_statement(statement, &last_variable_name, &last_variable_type);
    }

    new_block
}

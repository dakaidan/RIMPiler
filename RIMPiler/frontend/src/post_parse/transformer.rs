use super::super::post_parse::name_generator::NameGenerator;
use super::super::AST::{
    ArithmeticExpression, ArithmeticOperator, Block, BooleanExpression, Program,
    Statement, Variable
};
use std::collections::{HashMap, HashSet};

pub fn transform(program: &Program) -> Program {
    let mut name_generator = NameGenerator::new(String::from("semantic_transformer"));
    transform_program(program, &mut name_generator)
}

fn transform_program(program: &Program, name_generator: &mut NameGenerator) -> Program {
    match program {
        Program::Statements(statements) => {
            Program::Statements(transform_block(statements, name_generator))
        }
    }
}

fn transform_block(block: &Block, name_generator: &mut NameGenerator) -> Block {
    let mut new_block = Vec::new();
    for statement in block {
        if let Some(new_statement) = transform_statement(statement, name_generator) {
            new_block.extend(new_statement);
        } else {
            new_block.push(statement.clone());

        }
    }
    new_block
}

fn transform_statement(statement: &Statement, name_generator: &mut NameGenerator) -> Option<Block> {
    match statement {
        Statement::If(boolean_expression, if_block, else_block) => {
            transform_if_statement(boolean_expression,
                                   &transform_block(if_block, name_generator),
                                   &transform_block(else_block, name_generator),
                                   name_generator)
        }
        Statement::While(boolean_expression, block) => {
            transform_while_statement(boolean_expression,
                                      &transform_block(block, name_generator),
                                      name_generator)
        }
        _ => None,
    }
}

pub fn transform_if_only(program: &Program) -> Program {
    let mut name_generator = NameGenerator::new(String::from("semantic_transformer"));
    transform_program_if_only(program, &mut name_generator)
}

fn transform_program_if_only(program: &Program, name_generator: &mut NameGenerator) -> Program {
    match program {
        Program::Statements(statements) => {
            let mut new_statements = Vec::new();
            for statement in statements {
                if let Some(statements) = transform_statement_if_only(statement, name_generator) {
                    new_statements.extend(statements);
                } else {
                    new_statements.push(statement.clone());
                }
            }
            Program::Statements(new_statements)
        }
    }
}

fn transform_statement_if_only(statement: &Statement, name_generator: &mut NameGenerator) -> Option<Block> {
    match statement {
        Statement::If(boolean_expression, if_block, else_block) => {
            transform_if_statement(boolean_expression, if_block, else_block, name_generator)
        }
        _ => None,
    }
}

fn transform_if_statement(
    boolean_expression: &BooleanExpression,
    if_block: &Block,
    else_block: &Block,
    name_generator: &mut NameGenerator,
) -> Option<Block> {
    // get all variables assigned to in the if an else block
    // check all variables in the boolean expression
    // for each of these that overlap, create a hashmap of name -> new_name
    // then transform the boolean expression

    let variables_in_if_block = get_variables_in_block(if_block);
    let variables_in_else_block = get_variables_in_block(else_block);

    let variables_in_boolean_expression = get_variables_in_boolean_expression(boolean_expression);

    let mut variables_in_both = HashSet::new();
    variables_in_both.extend(variables_in_if_block.intersection(&variables_in_boolean_expression));
    variables_in_both
        .extend(variables_in_else_block.intersection(&variables_in_boolean_expression));

    if variables_in_both.is_empty() {
        return None;
    }

    let mut variables = HashMap::new();

    for variable in variables_in_both {
        let new_variable_name = name_generator.generate();
        variables.insert(variable.clone(), new_variable_name);
    }

    let new_boolean_expression =
        remap_variables_in_boolean_expression(boolean_expression, &variables);

    let mut new_block = Vec::new();
    for (variable, new_variable) in &variables {
        new_block.push(create_assignment_statement(
            new_variable.clone(),
            variable.clone(),
        ));
    }

    new_block.push(Statement::If(
        Box::new(new_boolean_expression),
        Box::new(if_block.clone()),
        Box::new(else_block.clone()),
    ));

    Some(new_block)
}

fn transform_while_statement(
    boolean_expression: &BooleanExpression,
    block: &Block,
    name_generator: &mut NameGenerator,
) -> Option<Block> {
    // here we need to add a counter assignment at the start of the block
    // then we need to add an increment at the end of the whiles block

    let counter_variable_name = name_generator.generate();
    let counter = Statement::Assignment(Variable::Integer(counter_variable_name.clone()),
        ArithmeticExpression::Integer(0),
    );

    let increment = Statement::Assignment(Variable::Integer(counter_variable_name.clone()),
        ArithmeticExpression::Operation(
            ArithmeticOperator::Addition,
            Box::new(ArithmeticExpression::Variable(
                Variable::Integer(counter_variable_name.clone())
            )),
            Box::new(ArithmeticExpression::Integer(1)),
        ),
    );

    let mut new_block = Vec::new();
    new_block.push(counter);

    let mut while_block = block.clone();
    while_block.push(increment);

    new_block.push(Statement::While(
        Box::new(boolean_expression.clone()),
        Box::new(while_block),
    ));

    Some(new_block)
}

// helper functions

fn get_variables_in_block(block: &Block) -> HashSet<(String, String)> {
    let mut variables = HashSet::new();
    for statement in block {
        variables.extend(get_variables_in_statement(statement));
    }
    variables
}

fn get_variables_in_statement(statement: &Statement) -> HashSet<(String, String)> {
    match statement {
        Statement::Skip => HashSet::new(),
        Statement::If(_, if_block, else_block) => {
            let mut variables = HashSet::new();
            variables.extend(get_variables_in_block(if_block));
            variables.extend(get_variables_in_block(else_block));
            variables
        }
        Statement::While(_, block) => {
            let mut variables = HashSet::new();
            variables.extend(get_variables_in_block(block));
            variables
        }
        Statement::Assignment(variable, _) => match variable {
            Variable::Integer(varriable) => {
                let mut variables = HashSet::new();
                variables.insert((varriable.clone(), "int".to_string()));
                variables
            }
            Variable::Float(varriable) => {
                let mut variables = HashSet::new();
                variables.insert((varriable.clone(), "float".to_string()));
                variables
            }
        },
        Statement::ReverseAssignment(_, _) | Statement::ReversePoint => {
            unreachable!(
                "You should never call this function after the reversal function has been called!"
            )
        }
    }
}

fn get_variables_in_boolean_expression(boolean_expression: &BooleanExpression) -> HashSet<(String, String)> {
    let mut variables = HashSet::new();
    match boolean_expression {
        BooleanExpression::Relational(_, arithmetic_expression1, arithmetic_expression2) => {
            variables.extend(get_variables_in_arithmetic_expression(
                arithmetic_expression1,
            ));
            variables.extend(get_variables_in_arithmetic_expression(
                arithmetic_expression2,
            ));
        }
        BooleanExpression::Unary(_, boolean_expression) => {
            variables.extend(get_variables_in_boolean_expression(boolean_expression));
        }
        BooleanExpression::Logical(_, boolean_expression1, boolean_expression2) => {
            variables.extend(get_variables_in_boolean_expression(boolean_expression1));
            variables.extend(get_variables_in_boolean_expression(boolean_expression2));
        }
    }
    variables
}

fn get_variables_in_arithmetic_expression(
    arithmetic_expression: &ArithmeticExpression,
) -> HashSet<(String, String)> {
    let mut variables = HashSet::new();
    match arithmetic_expression {
        ArithmeticExpression::Variable(variable) => {
            match variable {
                Variable::Integer(varriable) => {
                    variables.insert((varriable.clone(), "int".to_string()));
                }
                Variable::Float(varriable) => {
                    variables.insert((varriable.clone(), "float".to_string()));
                }
            }
        }
        ArithmeticExpression::Integer(_) => {}
        ArithmeticExpression::Float(_) => {}
        ArithmeticExpression::Unary(_, arithmetic_expression) => {
            variables.extend(get_variables_in_arithmetic_expression(
                arithmetic_expression,
            ));
        }
        ArithmeticExpression::Operation(_, arithmetic_expression1, arithmetic_expression2) => {
            variables.extend(get_variables_in_arithmetic_expression(
                arithmetic_expression1,
            ));
            variables.extend(get_variables_in_arithmetic_expression(
                arithmetic_expression2,
            ));
        }
    }
    variables
}

fn remap_variables_in_boolean_expression(
    boolean_expression: &BooleanExpression,
    variables: &HashMap<(String, String), String>,
) -> BooleanExpression {
    match boolean_expression {
        BooleanExpression::Relational(op, arithmetic_expression1, arithmetic_expression2) => {
            BooleanExpression::Relational(
                op.clone(),
                Box::new(remap_variables_in_arithmetic_expression(
                    arithmetic_expression1,
                    variables,
                )),
                Box::new(remap_variables_in_arithmetic_expression(
                    arithmetic_expression2,
                    variables,
                )),
            )
        }
        BooleanExpression::Unary(op, boolean_expression) => BooleanExpression::Unary(
            op.clone(),
            Box::new(remap_variables_in_boolean_expression(
                boolean_expression,
                variables,
            )),
        ),
        BooleanExpression::Logical(op, boolean_expression1, boolean_expression2) => {
            BooleanExpression::Logical(
                op.clone(),
                Box::new(remap_variables_in_boolean_expression(
                    boolean_expression1,
                    variables,
                )),
                Box::new(remap_variables_in_boolean_expression(
                    boolean_expression2,
                    variables,
                )),
            )
        }
    }
}

fn remap_variables_in_arithmetic_expression(
    arithmetic_expression: &ArithmeticExpression,
    variables: &HashMap<(String, String), String>,
) -> ArithmeticExpression {
    match arithmetic_expression {
        ArithmeticExpression::Variable(variable) => {
            let (type_of_variable, name) = match variable {
                Variable::Integer(name) => ("int".to_string(), name.clone()),
                Variable::Float(name) => ("float".to_string(), name.clone()),
            };

            if let Some(new_variable) = variables.get(&(name, type_of_variable.clone())) {
                match type_of_variable.as_str() {
                    "int" => ArithmeticExpression::Variable(Variable::Integer(new_variable.clone())),
                    "float" => ArithmeticExpression::Variable(Variable::Float(new_variable.clone())),
                    _ => unreachable!(),
                }
            } else {
                ArithmeticExpression::Variable(variable.clone())
            }
        }
        ArithmeticExpression::Integer(i) => ArithmeticExpression::Integer(i.clone()),
        ArithmeticExpression::Float(f) => ArithmeticExpression::Float(f.clone()),
        ArithmeticExpression::Unary(op, arithmetic_expression) => ArithmeticExpression::Unary(
            op.clone(),
            Box::new(remap_variables_in_arithmetic_expression(
                arithmetic_expression,
                variables,
            )),
        ),
        ArithmeticExpression::Operation(op, arithmetic_expression1, arithmetic_expression2) => {
            ArithmeticExpression::Operation(
                op.clone(),
                Box::new(remap_variables_in_arithmetic_expression(
                    arithmetic_expression1,
                    variables,
                )),
                Box::new(remap_variables_in_arithmetic_expression(
                    arithmetic_expression2,
                    variables,
                )),
            )
        }
    }
}

fn create_assignment_statement(
    new_variable_name: String,
    right_hand_variable_name: (String, String),
) -> Statement {
    match right_hand_variable_name.1.as_str() {
        "int" => Statement::Assignment(Variable::Integer(new_variable_name),
                ArithmeticExpression::Variable(Variable::Integer(right_hand_variable_name.0)),
        ),
        "float" => Statement::Assignment(
            Variable::Float(new_variable_name),
            ArithmeticExpression::Variable(Variable::Float(right_hand_variable_name.0)),
        ),
        _ => unreachable!(),
    }
}

use std::collections::HashMap;
use ordered_float::NotNan;
use utilities::debug::{Error, Location};
use crate::AST::{ArithmeticExpression, ArithmeticOperator, Block, BooleanExpression, BooleanOperator, Program, RelationOperator, Statement, UnaryArithmeticOperator, UnaryBooleanOperator, Variable};
use crate::lexer::tokens::{Bracket, Keyword, Operator, RIMPToken, Tokens};
use crate::parser::precedence;
use crate::post_parse::transformer::transform;

fn expect_operator(operator: Operator, tokens: &mut Tokens) -> std::result::Result<(), Location> {
    let next_token = tokens.next();
    match next_token {
        Some(token) => {
            if token.value != RIMPToken::Operator(operator) {
                Err(token.location)
            } else {
                Ok(())
            }
        }
        None => Err(Location::default()),
    }
}

fn expect_identifier(tokens: &mut Tokens) -> std::result::Result<String, Location> {
    let next_token = tokens.next();
    match next_token {
        Some(token) => match token.value {
            RIMPToken::Identifier(identifier) => Ok(identifier),
            _ => Err(token.location),
        },
        None => Err(Location::default()),
    }
}

fn expect_keyword(keyword: Keyword, tokens: &mut Tokens) -> std::result::Result<(), Location> {
    let next_token = tokens.next();
    match next_token {
        Some(token) => {
            if token.value != RIMPToken::Keyword(keyword) {
                Err(token.location)
            } else {
                Ok(())
            }
        }
        None => {
            Err(Location::default())
        }
    }
}

fn expect_bracket(bracket: Bracket, tokens: &mut Tokens) -> std::result::Result<(), Location> {
    let next_token = tokens.next();
    match next_token {
        Some(token) => {
            if token.value != RIMPToken::Bracket(bracket) {
                Err(token.location)
            } else {
                Ok(())
            }
        }
        None => {
            Err(Location::default())
        }
    }
}

fn expect_semicolon(tokens: &mut Tokens) -> std::result::Result<(), Location> {
    let next_token = tokens.next();
    match next_token {
        Some(token) => {
            if token.value != RIMPToken::Semicolon {
                Err(token.location)
            } else {
                Ok(())
            }
        }
        None => Err(Location::default()),
    }
}

pub fn parse(tokens: &mut Tokens) -> utilities::debug::Result<Program> {
    let mut parser = Parser::new();
    let result = parser.parse_program(tokens);
    if result.is_err() {
        return Err(result.unwrap_err());
    }

    Ok(transform(&result.unwrap()))
}

pub fn parse_without_transform(tokens: &mut Tokens) -> utilities::debug::Result<Program> {
    let mut parser = Parser::new();
    let result = parser.parse_program(tokens);
    if result.is_err() {
        return Err(result.unwrap_err());
    }

    Ok(result.unwrap())
}


pub struct Parser {
    variable_type: HashMap<String, String>
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            variable_type: HashMap::new()
        }
    }

    fn map_type(&mut self, identifier: String, type_: String) -> utilities::debug::Result<()>
    {
        if self.variable_type.contains_key(&identifier) {
            return Err(Error::new(
                Location::default(),
                format!("Variable {} already declared", identifier),
                "Parser".to_string(),
            ));
        }
        self.variable_type.insert(identifier, type_);
        Ok(())
    }

    fn get_type(&self, identifier: &String) -> utilities::debug::Result<String>
    {
        match self.variable_type.get(identifier) {
            Some(type_) => Ok(type_.to_string()),
            None => Err(Error::new(
                Location::default(),
                format!("Variable {} not declared", identifier),
                "Parser".to_string(),
            ))
        }
    }


    pub fn parse_program(&mut self, tokens: &mut Tokens) -> utilities::debug::Result<Program> {
        let mut statements = Vec::new();

        loop {
            let next_token = tokens.peek();
            match next_token {
                Some(_) => {}
                None => return Ok(Program::Statements(statements)),
            }

            let statement = self.parse_statement(tokens);

            if statement.is_err() {
                return Err(statement.unwrap_err());
            }

            let result = expect_semicolon(tokens);

            if result.is_err() {
                return Err(Error::new(
                    result.unwrap_err(),
                    "Expected semicolon".to_string(),
                    "Parser".to_string(),
                ));
            }

            statements.push(statement.unwrap());
        }
    }

    fn parse_statement(&mut self, tokens: &mut Tokens) -> utilities::debug::Result<Statement> {
        /*
           A statement is either:
               - an assignment : If the next token is an identifier, then we have an assignment
               - a while loop : If the next token is a while keyword, then we have a while loop
               - an if statement : If the next token is an if keyword, then we have an if statement
               - a skip statement : If the next token is a skip, then we have a skip statement
        */
        match tokens.next() {
            Some(token) => match token.value {
                RIMPToken::Identifier(identifier) => {
                    let result = expect_operator(Operator::Assign, tokens);

                    if result.is_err() {
                        return Err(Error::new(
                            result.unwrap_err(),
                            "Expected assignment operator".to_string(),
                            "Parser".to_string(),
                        ));
                    }

                    let expression = self.parse_arithmetic_expression(tokens, 0);

                    if expression.is_err() {
                        return Err(expression.unwrap_err());
                    }

                    let type_ = self.get_type(&identifier);
                    if type_.is_err() {
                        let location = token.location;
                        return Err(
                            Error::new(
                                location,
                                "Variable not declared".to_string(),
                                "Parser".to_string(),
                            )
                        )
                    }

                    match type_.unwrap().as_str() {
                        "int" => Ok(Statement::Assignment(Variable::Integer(identifier), expression.unwrap())),
                        "float" => Ok(Statement::Assignment(Variable::Float(identifier), expression.unwrap())),
                        type_ => {
                            return Err(Error::new(
                                token.location,
                                format!("Unknown type {}", type_),
                                "Parser".to_string(),
                            ))
                        }
                    }
                }
                RIMPToken::Keyword(keyword) => match keyword {
                    Keyword::While => {
                        let condition = self.parse_boolean_expression(tokens, 0);

                        if condition.is_err() {
                            return Err(condition.unwrap_err());
                        }

                        let result = expect_keyword(Keyword::Do, tokens);

                        if result.is_err() {
                            return Err(Error::new(
                                result.unwrap_err(),
                                "Expected keyword do".to_string(),
                                "Parser".to_string(),
                            ));
                        }

                        let block = self.parse_block(tokens);

                        if block.is_err() {
                            return Err(block.unwrap_err());
                        }

                        Ok(Statement::While(
                            Box::new(condition.unwrap()),
                            Box::new(block.unwrap()),
                        ))
                    }
                    Keyword::If => {
                        let condition = self.parse_boolean_expression(tokens, 0);

                        if condition.is_err() {
                            return Err(condition.unwrap_err());
                        }

                        let result = expect_keyword(Keyword::Then, tokens);

                        if result.is_err() {
                            return Err(Error::new(
                                result.unwrap_err(),
                                "Expected keyword then".to_string(),
                                "Parser".to_string(),
                            ));
                        }

                        let if_block = self.parse_block(tokens);

                        if if_block.is_err() {
                            return Err(if_block.unwrap_err());
                        }

                        let result = expect_keyword(Keyword::Else, tokens);

                        if result.is_err() {
                            return Err(Error::new(
                                result.unwrap_err(),
                                "Expected keyword else".to_string(),
                                "Parser".to_string(),
                            ));
                        }

                        let else_block = self.parse_block(tokens);

                        if else_block.is_err() {
                            return Err(else_block.unwrap_err());
                        }

                        Ok(Statement::If(
                            Box::new(condition.unwrap()),
                            Box::new(if_block.unwrap()),
                            Box::new(else_block.unwrap()),
                        ))
                    }
                    Keyword::Skip => Ok(Statement::Skip),
                    Keyword::Int => {
                        let identifier = expect_identifier(tokens);

                        if identifier.is_err() {
                            return Err(Error::new(
                                identifier.unwrap_err(),
                                "Expected identifier".to_string(),
                                "Parser".to_string(),
                            ));
                        }

                        let result = expect_operator(Operator::Assign, tokens);

                        if result.is_err() {
                            return Err(Error::new(
                                result.unwrap_err(),
                                "Expected assignment operator".to_string(),
                                "Parser".to_string(),
                            ));
                        }

                        let expression = self.parse_arithmetic_expression(tokens, 0);

                        if expression.is_err() {
                            return Err(expression.unwrap_err());
                        }

                        let map_result = self.map_type(identifier.clone().unwrap(), "int".to_string());
                        if map_result.is_err() {
                            let location = token.location;
                            return Err(
                                Error::new(
                                    location,
                                    "Variable already declared".to_string(),
                                    "Parser".to_string(),
                                )
                            )
                        }

                        Ok(Statement::Assignment(Variable::Integer(identifier.unwrap()), expression.unwrap()))
                    }
                    Keyword::Float => {
                        let identifier = expect_identifier(tokens);

                        if identifier.is_err() {
                            return Err(Error::new(
                                identifier.unwrap_err(),
                                "Expected identifier".to_string(),
                                "Parser".to_string(),
                            ));
                        }

                        let result = expect_operator(Operator::Assign, tokens);

                        if result.is_err() {
                            return Err(Error::new(
                                result.unwrap_err(),
                                "Expected assignment operator".to_string(),
                                "Parser".to_string(),
                            ));
                        }

                        let expression = self.parse_arithmetic_expression(tokens, 0);

                        if expression.is_err() {
                            return Err(expression.unwrap_err());
                        }

                        let map_result = self.map_type(identifier.clone().unwrap(), "float".to_string());
                        if map_result.is_err() {
                            let location = token.location;
                            return Err(
                                Error::new(
                                    location,
                                    "Variable already declared".to_string(),
                                    "Parser".to_string(),
                                )
                            )
                        }

                        // if expression is just a single integer, we can convert it to a float now
                        match expression.unwrap() {
                            ArithmeticExpression::Integer(integer) => Ok(Statement::Assignment(
                                Variable::Float(identifier.unwrap()),
                                ArithmeticExpression::Float(NotNan::new(integer as f32).unwrap()))
                            ),
                            exp => Ok(Statement::Assignment(Variable::Float(identifier.unwrap()), exp))
                        }
                    }
                    _ => {
                        return Err(Error::new(
                            token.location,
                            "Expected statement".to_string(),
                            "Parser".to_string(),
                        ))
                    }
                },
                _ => {
                    return Err(Error::new(
                        token.location,
                        "Expected statement".to_string(),
                        "Parser".to_string(),
                    ))
                }
            },
            None => {
                return Err(Error::new(
                    Location::default(),
                    "Expected statement found EOF".to_string(),
                    "Parser".to_string(),
                ))
            }
        }
    }

    fn parse_block(&mut self, tokens: &mut Tokens) -> utilities::debug::Result<Block> {
        let mut statements = Vec::new();

        let result = expect_bracket(Bracket::LeftBrace, tokens);

        if result.is_err() {
            return Err(Error::new(
                result.unwrap_err(),
                "Expected opening brace".to_string(),
                "Parser".to_string(),
            ));
        }

        loop {
            let next_token = tokens.peek();
            match next_token {
                Some(token) => {
                    if token.value == RIMPToken::Bracket(Bracket::RightBrace) {
                        tokens.next();
                        return Ok(statements);
                    }
                }
                None => {
                    return Err(Error::new(
                        result.unwrap_err(),
                        "Expected closing brace".to_string(),
                        "Parser".to_string(),
                    ));
                }
            }

            let statement = self.parse_statement(tokens);

            if statement.is_err() {
                return Err(statement.unwrap_err());
            }

            let result = expect_semicolon(tokens);

            if result.is_err() {
                return Err(Error::new(
                    result.unwrap_err(),
                    "Expected semicolon".to_string(),
                    "Parser".to_string(),
                ));
            }

            statements.push(statement.unwrap());
        }
    }

    pub(crate) fn parse_arithmetic_expression(
        &mut self,
        tokens: &mut Tokens,
        min_binding_power: u8,
    ) -> utilities::debug::Result<ArithmeticExpression> {
        let mut left_hand_side = match tokens.next() {
            Some(token) => match token.value {
                RIMPToken::Integer(number) => ArithmeticExpression::Integer(number),
                RIMPToken::Float(number) => ArithmeticExpression::Float(number),
                RIMPToken::Identifier(identifier) => {
                    let type_ = self.get_type(&identifier);
                    if type_.is_err() {
                        let location = token.location;
                        return Err(
                            Error::new(
                                location,
                                "Variable not declared".to_string(),
                                "Parser".to_string(),
                            )
                        )
                    }
                    let type_ = type_.unwrap();
                    match type_.as_str() {
                        "int" => ArithmeticExpression::Variable(Variable::Integer(identifier)),
                        "float" => ArithmeticExpression::Variable(Variable::Float(identifier)),
                        _ => {
                            return Err(Error::new(
                                token.location,
                                format!("Unknown type {}", type_),
                                "Parser".to_string(),
                            ))
                        }
                    }
                },
                RIMPToken::Bracket(b) if b == Bracket::LeftParenthesis => {
                    let expression = self.parse_arithmetic_expression(tokens, 0);
                    if expression.is_err() {
                        return expression;
                    }
                    let next_token = tokens.next();
                    match next_token {
                        Some(token) => {
                            if token.value != RIMPToken::Bracket(Bracket::RightParenthesis) {
                                return Err(Error::new(
                                    token.location,
                                    "Expected closing parenthesis".to_string(),
                                    "Parser".to_string(),
                                ));
                            }
                        }
                        None => {
                            return Err(Error::new(
                                token.location,
                                "Expected closing parenthesis found EOF".to_string(),
                                "Parser".to_string(),
                            ));
                        }
                    }
                    expression.unwrap()
                }
                RIMPToken::Operator(op) => match op {
                    Operator::Minus => {
                        let operator = UnaryArithmeticOperator::Negation;
                        let ((), right_binding_power) =
                            precedence::arithmetic_unary_binding_power(&operator);
                        let right_hand_side = self.parse_arithmetic_expression(tokens, right_binding_power);
                        if right_hand_side.is_err() {
                            return right_hand_side;
                        }

                        ArithmeticExpression::Unary(operator, Box::new(right_hand_side.unwrap()))
                    }
                    _ => {
                        return Err(Error::new(
                            token.location,
                            "Expected number or identifier".to_string(),
                            "Parser".to_string(),
                        ))
                    }
                },
                _ => {
                    return Err(Error::new(
                        token.location,
                        "Expected number or identifier found EOF".to_string(),
                        "Parser".to_string(),
                    ))
                }
            },
            None => {
                return Err(Error::new(
                    Location::default(),
                    "Expected number or identifier found EOF".to_string(),
                    "Parser".to_string(),
                ))
            }
        };

        loop {
            let operator = match tokens.peek() {
                Some(token) => match token.value {
                    RIMPToken::Operator(operator) => match operator {
                        Operator::Add => ArithmeticOperator::Addition,
                        Operator::Minus => ArithmeticOperator::Subtraction,
                        Operator::Multiply => ArithmeticOperator::Multiplication,
                        Operator::Divide => ArithmeticOperator::Division,
                        Operator::Exponent => ArithmeticOperator::Exponentiation,
                        _ => break,
                    },
                    _ => break,
                },
                None => break,
            };

            /*
            Here we can handle postfix unary operators something like this:
                if is unary
                get binding power
                check if left is less than min, if so, break
                consume next token
                left_hand_side = Operation (op left)
                continue
             */

            let (left_binding_power, right_binding_power) =
                precedence::arithmetic_binding_power(&operator);

            if left_binding_power < min_binding_power {
                break;
            }

            tokens.next();
            let right_hand_side = self.parse_arithmetic_expression(tokens, right_binding_power);

            if right_hand_side.is_err() {
                return right_hand_side;
            } else {
                left_hand_side = ArithmeticExpression::Operation(
                    operator,
                    Box::new(left_hand_side),
                    Box::new(right_hand_side.unwrap()),
                );
            }
            continue;
        }

        Ok(left_hand_side)
    }

    pub(crate) fn parse_boolean_expression(
        &mut self,
        tokens: &mut Tokens,
        min_binding_power: u8,
    ) -> utilities::debug::Result<BooleanExpression> {
        /*
            NOTE:
            A boolean expression is either:
                - an arithmetic expression with a relation operator and another arithmetic expression
                - a boolean expression with a boolean operator and another boolean expression

            A boolean expression will always start with an arithmetic expression, so we can parse that first
            the next token will be a relation operator.
            the next token will be another arithmetic expression
            we will do these in another function.

            In this function then, we will use parse_relations to get the first boolean expression
            then we will check if the next token is a boolean operator
            if it is, we will parse the next boolean expression

            like this we build in type checking for boolean expressions in the parser.

            This does need to be checked a bit more formally later, for now, it appears correct
            and we can always quickly fall back to parser combinators later if we need to.
        */

        let left_hand_side = match tokens.peek() {
            Some(token) => match token.value {
                RIMPToken::Bracket(b) if b == Bracket::LeftParenthesis => {
                    tokens.next();
                    let expression = self.parse_boolean_expression(tokens, 0);
                    if expression.is_err() {
                        return expression;
                    }
                    let next_token = tokens.next();
                    match next_token {
                        Some(token) => {
                            if token.value != RIMPToken::Bracket(Bracket::RightParenthesis) {
                                return Err(Error::new(
                                    token.location,
                                    "Expected closing parenthesis".to_string(),
                                    "Parser".to_string(),
                                ));
                            }
                        }
                        None => {
                            return Err(Error::new(
                                token.location,
                                "Expected closing parenthesis found EOF".to_string(),
                                "Parser".to_string(),
                            ))
                        }
                    }
                    expression
                }
                RIMPToken::Operator(op) => match op {
                    Operator::Not => {
                        tokens.next();
                        let operator = UnaryBooleanOperator::Negation;
                        let ((), right_binding_power) =
                            precedence::boolean_unary_binding_power(&operator);
                        let right_hand_side = self.parse_boolean_expression(tokens, right_binding_power);
                        if right_hand_side.is_err() {
                            return right_hand_side;
                        }

                        Ok(BooleanExpression::Unary(
                            operator,
                            Box::new(right_hand_side.unwrap()),
                        ))
                    }
                    _ => self.parse_relations(tokens),
                },
                _ => self.parse_relations(tokens),
            },
            None => {
                return Err(Error::new(
                    Location::default(),
                    "Expected boolean expression found EOF".to_string(),
                    "Parser".to_string(),
                ))
            }
        };

        if left_hand_side.is_err() {
            return left_hand_side;
        }

        let mut left_hand_side = left_hand_side.unwrap();

        loop {
            let operator = match tokens.peek() {
                Some(token) => match token.value {
                    RIMPToken::Operator(operator) => match operator {
                        Operator::And => BooleanOperator::And,
                        Operator::Or => BooleanOperator::Or,
                        _ => break,
                    },
                    _ => break,
                },
                None => break,
            };

            let (left_binding_power, right_binding_power) =
                precedence::boolean_operator_binding_power(&operator);

            if left_binding_power < min_binding_power {
                break;
            }

            tokens.next();
            let right_hand_side = self.parse_boolean_expression(tokens, right_binding_power);

            if right_hand_side.is_err() {
                return right_hand_side;
            } else {
                left_hand_side = BooleanExpression::Logical(
                    operator,
                    Box::new(left_hand_side),
                    Box::new(right_hand_side.unwrap()),
                );
            }
        }

        Ok(left_hand_side)
    }

    pub(crate) fn parse_relations(&mut self, tokens: &mut Tokens) -> utilities::debug::Result<BooleanExpression> {
        let left_hand_side = self.parse_arithmetic_expression(tokens, 0);

        if left_hand_side.is_err() {
            return Err(left_hand_side.unwrap_err());
        }

        let left_hand_side = left_hand_side.unwrap();

        let operator = match tokens.next() {
            Some(token) => match token.value {
                RIMPToken::Operator(operator) => match operator {
                    Operator::Equal => RelationOperator::Equal,
                    Operator::NotEqual => RelationOperator::NotEqual,
                    Operator::GreaterThan => RelationOperator::GreaterThan,
                    Operator::LessThan => RelationOperator::LessThan,
                    _ => {
                        return Err(Error::new(
                            token.location,
                            "Expected relation operator".to_string(),
                            "Parser".to_string(),
                        ))
                    }
                },
                _ => {
                    return Err(Error::new(
                        token.location,
                        "Expected relation operator".to_string(),
                        "Parser".to_string(),
                    ))
                }
            },
            None => {
                return Err(Error::new(
                    Location::default(),
                    "Expected relation operator found EOF".to_string(),
                    "Parser".to_string(),
                ))
            }
        };

        let right_hand_side = self.parse_arithmetic_expression(tokens, 0);

        if right_hand_side.is_err() {
            return Err(right_hand_side.unwrap_err());
        }

        let right_hand_side = right_hand_side.unwrap();

        Ok(BooleanExpression::Relational(
            operator,
            Box::new(left_hand_side),
            Box::new(right_hand_side),
        ))
    }
}

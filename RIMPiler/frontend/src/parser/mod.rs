use regex::lexer::{Location, TokenMeta};
use crate::AST::{ArithmeticExpression, ArithmeticOperator, Assignment, Block, BooleanExpression, BooleanOperator, Program, RelationOperator, Statement, UnaryArithmeticOperator, UnaryBooleanOperator};
use crate::lexer::tokens::{Bracket, Keyword, Operator, RIMPToken, Tokens};
use crate::parser::ErrorType::{UnexpectedEndOfFile, UnexpectedToken};

mod precedence;
mod tests;

#[derive(Debug, Eq, PartialEq, Clone)]
enum ErrorType {
    UnexpectedToken,
    UnexpectedEndOfFile,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Error {
    message: String,
    location: Option<Location>,
    error_type: ErrorType,
}

impl Error {
    fn new(message: String, error_type: ErrorType, location: Option<Location>) -> Self {
        Error { message, location, error_type }
    }
}

pub type ParseResult<T> = Result<T, Error>;

fn expect_operator(operator: Operator, tokens: &mut Tokens) -> Result<(), TokenMeta<RIMPToken>> {
    let next_token = tokens.next();
    match next_token {
        Some(token) => {
            if token.token != RIMPToken::Operator(operator) {
                Err(token)
            } else {
                Ok(())
            }
        }
        None => Err(TokenMeta::new("".to_string(), Location::default(), "operator".to_string()).unwrap()),
    }
}

fn expect_identifier(tokens: &mut Tokens) -> Result<String, TokenMeta<RIMPToken>> {
    let next_token = tokens.next();
    match next_token {
        Some(token) => {
            match token.token {
                RIMPToken::Identifier(identifier) => Ok(identifier),
                _ => Err(token),
            }
        }
        None => Err(TokenMeta::new("".to_string(), Location::default(), "identifier".to_string()).unwrap()),
    }
}

fn expect_keyword(keyword: Keyword, tokens: &mut Tokens) -> Result<(), TokenMeta<RIMPToken>> {
    let next_token = tokens.next();
    match next_token {
        Some(token) => {
            if token.token != RIMPToken::Keyword(keyword) {
                Err(token)
            } else {
                Ok(())
            }
        }
        None => Err(TokenMeta::new("".to_string(), Location::default(), "keyword".to_string()).unwrap()),
    }
}

fn expect_bracket(bracket: Bracket, tokens: &mut Tokens) -> Result<(), TokenMeta<RIMPToken>> {
    let next_token = tokens.next();
    match next_token {
        Some(token) => {
            if token.token != RIMPToken::Bracket(bracket) {
                Err(token)
            } else {
                Ok(())
            }
        }
        None => Err(TokenMeta::new("".to_string(), Location::default(), "bracket".to_string()).unwrap()),
    }
}

fn expect_semicolon(tokens: &mut Tokens) -> Result<(), TokenMeta<RIMPToken>> {
    let next_token = tokens.next();
    match next_token {
        Some(token) => {
            if token.token != RIMPToken::Semicolon {
                Err(token)
            } else {
                Ok(())
            }
        }
        None => Err(TokenMeta::new("".to_string(), Location::default(), "semicolon".to_string()).unwrap()),
    }
}

pub fn parse_program(tokens: &mut Tokens) -> ParseResult<Program> {
    let mut statements = Vec::new();

    loop {
        let next_token = tokens.peek();
        match next_token {
            Some(_) => {}
            None => return Ok(Program::Statements(statements)),
        }

        let statement = parse_statement(tokens);

        if statement.is_err() {
            return Err(statement.unwrap_err());
        }

        let result = expect_semicolon(tokens);

        if result.is_err() {
            return Err(Error::new("Expected semicolon".to_string(), UnexpectedToken, Some(result.unwrap_err().location)));
        }

        statements.push(statement.unwrap());
    }
}

fn parse_statement(tokens: &mut Tokens) -> ParseResult<Statement> {
    /*
        A statement is either:
            - an assignment : If the next token is an identifier, then we have an assignment
            - a while loop : If the next token is a while keyword, then we have a while loop
            - an if statement : If the next token is an if keyword, then we have an if statement
            - a skip statement : If the next token is a skip, then we have a skip statement
     */
    match tokens.next() {
        Some(token) => {
            match token.token {
                RIMPToken::Identifier(identifier) => {
                    let result = expect_operator(Operator::Assign, tokens);

                    if result.is_err() {
                        return Err(Error::new("Expected assignment operator".to_string(), UnexpectedToken, Some(result.unwrap_err().location)));
                    }

                    let expression = parse_arithmetic_expression(tokens, 0);

                    if expression.is_err() {
                        return Err(expression.unwrap_err());
                    }

                    Ok(Statement::Assignment(Assignment::Integer(identifier, Box::new(expression.unwrap()))))
                }
                RIMPToken::Keyword(keyword) => {
                    match keyword {
                        Keyword::While => {
                            let condition = parse_boolean_expression(tokens, 0);

                            if condition.is_err() {
                                return Err(condition.unwrap_err());
                            }

                            let result = expect_keyword(Keyword::Do, tokens);

                            if result.is_err() {
                                return Err(Error::new("Expected do keyword".to_string(), UnexpectedToken, Some(result.unwrap_err().location)));
                            }

                            let block = parse_block(tokens);

                            if block.is_err() {
                                return Err(block.unwrap_err());
                            }

                            Ok(Statement::While(Box::new(condition.unwrap()), Box::new(block.unwrap())))
                        }
                        Keyword::If => {
                            let condition = parse_boolean_expression(tokens, 0);

                            if condition.is_err() {
                                return Err(condition.unwrap_err());
                            }

                            let result = expect_keyword(Keyword::Then, tokens);

                            if result.is_err() {
                                return Err(Error::new("Expected then keyword".to_string(), UnexpectedToken, Some(result.unwrap_err().location)));
                            }

                            let if_block = parse_block(tokens);

                            if if_block.is_err() {
                                return Err(if_block.unwrap_err());
                            }

                            let result = expect_keyword(Keyword::Else, tokens);

                            if result.is_err() {
                                return Err(Error::new("Expected else keyword".to_string(), UnexpectedToken, Some(result.unwrap_err().location)));
                            }

                            let else_block = parse_block(tokens);

                            if else_block.is_err() {
                                return Err(else_block.unwrap_err());
                            }

                            Ok(Statement::If(Box::new(condition.unwrap()), Box::new(if_block.unwrap()), Box::new(else_block.unwrap())))
                        }
                        Keyword::Skip => {
                            Ok(Statement::Skip)
                        }
                        Keyword::Int => {
                            let identifier = expect_identifier(tokens);

                            if identifier.is_err() {
                                return Err(Error::new("Expected identifier".to_string(), UnexpectedToken, Some(identifier.unwrap_err().location)));
                            }

                            let result = expect_operator(Operator::Assign, tokens);

                            if result.is_err() {
                                return Err(Error::new("Expected assignment operator".to_string(), UnexpectedToken, Some(result.unwrap_err().location)));
                            }

                            let expression = parse_arithmetic_expression(tokens, 0);

                            if expression.is_err() {
                                return Err(expression.unwrap_err());
                            }

                            Ok(Statement::Assignment(Assignment::Integer(identifier.unwrap(), Box::new(expression.unwrap()))))
                        }
                        _ => return Err(Error::new("Expected statement".to_string(), UnexpectedToken, Some(token.location))),
                    }
                }
                _ => return Err(Error::new("Expected statement".to_string(), UnexpectedToken, Some(token.location))),
            }
        }
        None => return Err(Error::new("Expected statement".to_string(), UnexpectedEndOfFile, None)),
    }
}

fn parse_block(tokens: &mut Tokens) -> ParseResult<Block> {
    let mut statements = Vec::new();

    let result = expect_bracket(Bracket::LeftBrace, tokens);

    if result.is_err() {
        return Err(Error::new("Expected opening brace".to_string(), UnexpectedToken, Some(result.unwrap_err().location)));
    }

    loop {
        let next_token = tokens.peek();
        match next_token {
            Some(token) => {
                if token.token == RIMPToken::Bracket(Bracket::RightBrace) {
                    tokens.next();
                    return Ok(statements);
                }
            }
            None => return Err(Error::new("Expected closing brace".to_string(), UnexpectedEndOfFile, None)),
        }

        let statement = parse_statement(tokens);

        if statement.is_err() {
            return Err(statement.unwrap_err());
        }

        let result = expect_semicolon(tokens);

        if result.is_err() {
            return Err(Error::new("Expected semicolon".to_string(), UnexpectedToken, Some(result.unwrap_err().location)));
        }

        statements.push(statement.unwrap());
    }
}

fn parse_arithmetic_expression(tokens: &mut Tokens, min_binding_power: u8) -> ParseResult<ArithmeticExpression> {
    let mut left_hand_side = match tokens.next() {
        Some(token) => {
            match token.token {
                RIMPToken::Number(number) => ArithmeticExpression::Integer(number),
                RIMPToken::Identifier(identifier) => ArithmeticExpression::Variable(identifier),
                RIMPToken::Bracket(b) if b == Bracket::LeftParenthesis => {
                    let expression = parse_arithmetic_expression(tokens, 0);
                    if expression.is_err() {
                        return expression;
                    }
                    let next_token = tokens.next();
                    match next_token {
                        Some(token) => {
                            if token.token != RIMPToken::Bracket(Bracket::RightParenthesis) {
                                return Err(Error::new("Expected closing parenthesis".to_string(), UnexpectedToken, Some(token.location)));
                            }
                        }
                        None => return Err(Error::new("Expected closing parenthesis".to_string(), UnexpectedEndOfFile, None)),
                    }
                    expression.unwrap()
                }
                RIMPToken::Operator(op) => {
                    match op {
                        Operator::Minus => {
                            let operator = UnaryArithmeticOperator::Negation;
                            let ((), right_binding_power) = precedence::arithmetic_unary_binding_power(&operator);
                            let right_hand_side = parse_arithmetic_expression(tokens, right_binding_power);
                            if right_hand_side.is_err() {
                                return right_hand_side;
                            }

                            ArithmeticExpression::Unary(operator, Box::new(right_hand_side.unwrap()))
                        }
                        _ => return Err(Error::new("Expected number or identifier".to_string(), UnexpectedToken, Some(token.location))),
                    }
                },
                _ => return Err(Error::new("Expected number or identifier".to_string(), UnexpectedToken, Some(token.location))),
            }
        }
        None => return Err(Error::new("Expected number or identifier".to_string(), UnexpectedEndOfFile, None)),
    };

    loop {
        let operator = match tokens.peek() {
            Some(token) => {
                match token.token {
                    RIMPToken::Operator(operator) => match operator {
                        Operator::Add => ArithmeticOperator::Addition,
                        Operator::Minus => ArithmeticOperator::Subtraction,
                        Operator::Multiply => ArithmeticOperator::Multiplication,
                        Operator::Divide => ArithmeticOperator::Division,
                        Operator::Exponent => ArithmeticOperator::Exponentiation,
                        _ => break,
                    }
                    _ => break,
                }
            }
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

        let (left_binding_power, right_binding_power) = precedence::arithmetic_binding_power(&operator);

        if left_binding_power < min_binding_power {
            break;
        }

        tokens.next();
        let right_hand_side = parse_arithmetic_expression(tokens, right_binding_power);

        if right_hand_side.is_err() {
            return right_hand_side;
        } else {
            left_hand_side = ArithmeticExpression::Operation(operator, Box::new(left_hand_side), Box::new(right_hand_side.unwrap()));
        }
        continue;
    }

    Ok(left_hand_side)
}

fn parse_boolean_expression(tokens: &mut Tokens, min_binding_power: u8) -> ParseResult<BooleanExpression> {
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
        Some(token) => {
            match token.token {
                RIMPToken::Bracket(b) if b == Bracket::LeftParenthesis => {
                    tokens.next();
                    let expression = parse_boolean_expression(tokens, 0);
                    if expression.is_err() {
                        return expression;
                    }
                    let next_token = tokens.next();
                    match next_token {
                        Some(token) => {
                            if token.token != RIMPToken::Bracket(Bracket::RightParenthesis) {
                                return Err(Error::new("Expected closing parenthesis".to_string(), UnexpectedToken, Some(token.location)));
                            }
                        }
                        None => return Err(Error::new("Expected closing parenthesis".to_string(), UnexpectedEndOfFile, None)),
                    }
                    expression
                },
                RIMPToken::Operator(op) => {
                    match op {
                        Operator::Not => {
                            tokens.next();
                            let operator = UnaryBooleanOperator::Negation;
                            let ((), right_binding_power) = precedence::boolean_unary_binding_power(&operator);
                            let right_hand_side = parse_boolean_expression(tokens, right_binding_power);
                            if right_hand_side.is_err() {
                                return right_hand_side;
                            }

                            Ok(BooleanExpression::Unary(operator, Box::new(right_hand_side.unwrap())))
                        }
                        _ => parse_relations(tokens),
                    }
                },
                _ => parse_relations(tokens),
            }
        }
        None => return Err(Error::new("Expected boolean expression".to_string(), UnexpectedEndOfFile, None)),
    };

    if left_hand_side.is_err() {
        return left_hand_side;
    }

    let mut left_hand_side = left_hand_side.unwrap();

    loop {
        let operator = match tokens.peek() {
            Some(token) => {
                match token.token {
                    RIMPToken::Operator(operator) => match operator {
                        Operator::And => BooleanOperator::And,
                        Operator::Or => BooleanOperator::Or,
                        _ => break,
                    }
                    _ => break,
                }
            }
            None => break,
        };

        let (left_binding_power, right_binding_power) = precedence::boolean_operator_binding_power(&operator);

        if left_binding_power < min_binding_power {
            break;
        }

        tokens.next();
        let right_hand_side = parse_boolean_expression(tokens, right_binding_power);

        if right_hand_side.is_err() {
            return right_hand_side;
        } else {
            left_hand_side = BooleanExpression::Logical(operator, Box::new(left_hand_side), Box::new(right_hand_side.unwrap()));
        }
    }

    Ok(left_hand_side)
}

fn parse_relations(tokens: &mut Tokens) -> ParseResult<BooleanExpression> {
    let left_hand_side = parse_arithmetic_expression(tokens, 0);

    if left_hand_side.is_err() {
        return Err(left_hand_side.unwrap_err());
    }

    let left_hand_side = left_hand_side.unwrap();

    let operator = match tokens.next() {
        Some(token) => {
            match token.token {
                RIMPToken::Operator(operator) => match operator {
                    Operator::Equal => RelationOperator::Equal,
                    Operator::NotEqual => RelationOperator::NotEqual,
                    Operator::GreaterThan => RelationOperator::GreaterThan,
                    Operator::LessThan => RelationOperator::LessThan,
                    _ => return Err(Error::new("Expected relation operator".to_string(), UnexpectedToken, Some(token.location))),
                }
                _ => return Err(Error::new("Expected relation operator".to_string(), UnexpectedToken, Some(token.location))),
            }
        }
        None => return Err(Error::new("Expected relation operator".to_string(), UnexpectedEndOfFile, None)),
    };

    let right_hand_side = parse_arithmetic_expression(tokens, 0);

    if right_hand_side.is_err() {
        return Err(right_hand_side.unwrap_err());
    }

    let right_hand_side = right_hand_side.unwrap();

    Ok(BooleanExpression::Relational(operator, Box::new(left_hand_side), Box::new(right_hand_side)))
}
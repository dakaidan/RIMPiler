// display functions for AST

use std::fmt::{Display, Formatter};
use crate::AST::{ArithmeticExpression, ArithmeticOperator, Assignment, BooleanExpression, BooleanOperator, Program, RelationOperator, Statement, UnaryArithmeticOperator, UnaryBooleanOperator};

impl Display for ArithmeticOperator {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            ArithmeticOperator::Addition => write!(f, "+"),
            ArithmeticOperator::Subtraction => write!(f, "-"),
            ArithmeticOperator::Multiplication => write!(f, "*"),
            ArithmeticOperator::Division => write!(f, "/"),
            ArithmeticOperator::Exponentiation => write!(f, "^"),
        }
    }
}

impl Display for UnaryArithmeticOperator {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            UnaryArithmeticOperator::Negation => write!(f, "-"),
        }
    }
}

impl Display for ArithmeticExpression {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            ArithmeticExpression::Integer(integer) => write!(f, "{}", integer),
            ArithmeticExpression::Variable(variable) => write!(f, "{}", variable),
            ArithmeticExpression::Operation(operation, left_hand_side, right_hand_side) => {
                write!(f, "({} {} {})", operation, left_hand_side, right_hand_side)
            }
            ArithmeticExpression::Unary(operation, operand) => {
                write!(f, "({} {})", operation, operand)
            }
        }
    }
}

impl Display for BooleanOperator {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            BooleanOperator::And => write!(f, "∧"),
            BooleanOperator::Or => write!(f, "∨"),
        }
    }
}

impl Display for RelationOperator {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            RelationOperator::Equal => write!(f, "=="),
            RelationOperator::NotEqual => write!(f, "!="),
            RelationOperator::LessThan => write!(f, "<"),
            RelationOperator::GreaterThan => write!(f, ">"),
        }
    }
}

impl Display for UnaryBooleanOperator {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            UnaryBooleanOperator::Negation => write!(f, "¬"),
        }
    }
}

impl Display for BooleanExpression {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            BooleanExpression::Logical(operator, left_hand_side, right_hand_side) => {
                write!(f, "({} {} {})", operator, left_hand_side, right_hand_side)
            }
            BooleanExpression::Relational(operator, left_hand_side, right_hand_side) => {
                write!(f, "({} {} {})", operator, left_hand_side, right_hand_side)
            }
            BooleanExpression::Unary(operator, operand) => {
                write!(f, "({} {})", operator, operand)
            }
        }
    }
}

impl Display for Assignment {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Assignment::Integer(variable, expression) => {
                write!(f, "{} := {}", variable, expression)
            }
        }
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Statement::Skip => write!(f, "skip\n"),
            Statement::If(condition, then_block, else_block) => {
                write!(
                    f,
                    "if {}\n{}\nelse\n{}end\n",
                    condition,
                    then_block.iter().map(|s| format!("\t{}", s)).collect::<String>(),
                    else_block.iter().map(|s| format!("\t{}", s)).collect::<String>(),
                )
            }
            Statement::While(condition, block) => {
                write!(
                    f,
                    "while {}\n{} end\n",
                    condition,
                    block.iter().map(|s| format!("\t{}", s)).collect::<String>(),
                )
            }
            Statement::Assignment(assignment) => write!(f, "{}\n", assignment),
            Statement::ReverseAssignment(assignment) => write!(f, "({})'\n", assignment),
            Statement::ReversePoint => write!(f, "---------------rp---------------'\n"),
        }
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Program::Statements(block) => {
                write!(
                    f,
                    "{}",
                    block.iter().map(|s| format!("{}\n", s)).collect::<String>()
                )
            }
        }
    }
}
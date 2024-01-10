use super::super::AST::{
    ArithmeticOperator, BooleanOperator, UnaryArithmeticOperator, UnaryBooleanOperator,
};

pub fn arithmetic_binding_power(op: &ArithmeticOperator) -> (u8, u8) {
    match op {
        ArithmeticOperator::Addition => (1, 2),
        ArithmeticOperator::Subtraction => (1, 2),
        ArithmeticOperator::Multiplication => (3, 4),
        ArithmeticOperator::Division => (3, 4),
        ArithmeticOperator::Exponentiation => (5, 6),
    }
}

pub fn arithmetic_unary_binding_power(op: &UnaryArithmeticOperator) -> ((), u8) {
    match op {
        UnaryArithmeticOperator::Negation => ((), 5),
    }
}
pub fn boolean_operator_binding_power(op: &BooleanOperator) -> (u8, u8) {
    match op {
        BooleanOperator::Or => (1, 2),
        BooleanOperator::And => (3, 4),
    }
}

pub fn boolean_unary_binding_power(op: &UnaryBooleanOperator) -> ((), u8) {
    match op {
        UnaryBooleanOperator::Negation => ((), 5),
    }
}

mod display;

pub type Block = Vec<Statement>;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Program {
    Statements(Block),
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Assignment {
    Integer(String, Box<ArithmeticExpression>),
}
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Statement {
    Skip,
    If(Box<BooleanExpression>, Box<Block>, Box<Block>),
    While(Box<BooleanExpression>, Box<Block>),
    Assignment(Assignment),
    ReverseAssignment(Assignment),
    ReversePoint,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum ArithmeticOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponentiation,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum UnaryArithmeticOperator {
    Negation,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum ArithmeticExpression {
    Variable(String),
    Integer(i32),
    Unary(UnaryArithmeticOperator, Box<ArithmeticExpression>),
    Operation(
        ArithmeticOperator,
        Box<ArithmeticExpression>,
        Box<ArithmeticExpression>,
    ),
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum BooleanOperator {
    And,
    Or,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum RelationOperator {
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum UnaryBooleanOperator {
    Negation,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum BooleanExpression {
    Logical(
        BooleanOperator,
        Box<BooleanExpression>,
        Box<BooleanExpression>,
    ),
    Relational(
        RelationOperator,
        Box<ArithmeticExpression>,
        Box<ArithmeticExpression>,
    ),
    Unary(UnaryBooleanOperator, Box<BooleanExpression>),
}

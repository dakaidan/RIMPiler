/*

We have the following stacks

c, b ::= nil | i · c
i ::= P | l | lab
r ::= nil | P · r | l · r

lab is a label, this is going to be used to indicate what is next
For example, the label exp means the next two elements on the stack
are expresions.

P can be an expression:
 - n                    // number
 - m                    // inverted number
 - !l                   // variable
 - ¬!l                  // inverted variable
 - E1 op E2             // binary operation where E1 and E2 are expressions
 - Uop E                // unary operation where E is an expression

P can be a command:
 - skip                 // skip
 - l := E               // assignment where l is a variable and E is an expression
 - l =: E               // inverted assignment where l is a variable and E is an expression
 - C1; C2               // sequence of commands where C1 and C2 are commands

P can be a conditional:
 - if E then C1 else C2 // conditional where E is an expression and C1 and C2 are commands

P can be a loop:
 - while_i E do C       // loop where E is an expression and C is a command, and i indicates
                           which loop instance this is.

We then have the labels for expressions:
 - oper                 // where oper is an operation, or the inverse. this will indicate that
                           the previous two expressions are to be operated on by this oper.
 - exp                  // this is used to indicate the inverse of evaluating expressions
                           before their operation
 - op                   // we have one for each boolean op, this indicates we are applying an op
 - inverse_op           // this is the inverse of the op label
 - unary_op             // this is the label for unary operations
 - inverse_unary_op     // this is the inverse of the unary_op label

labels for commands:
 - asgn                 // this is the label for assignment
 - :=                   // this is the label for assignment application
 - asgnr                // this is the label for inverted assignment
 - =:                   // this is the label for inverted assignment application
 - seq                  // this is the label for sequence
 - ;                    // this is the label for sequence application

labels for conditionals:
 - cond                 // this is the label for conditionals
 - if                   // this is the label for conditionals application
 - cond_inv             // this is the inverse of the cond label
 - if_inv               // this is the inverse of the if label

labels for loops:
 - loop_i               // this is the label for loops where i is the loop instance
 - while_i              // this is the label for loops application where i is the loop instance
 - end_w_i              // this is the label for the end of a loop where i is the loop instance

l is a variable.
*/

use std::fmt::Display;
use ordered_float::NotNan;
use crate::AST::{ArithmeticExpression, ArithmeticOperator, BooleanExpression, BooleanOperator, Program, RelationOperator, Statement, UnaryArithmeticOperator, UnaryBooleanOperator, Variable};
use super::super::post_parse::transformer::transform_if_only;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Int,
    Float
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Num {
    Int(i32),
    Float(NotNan<f32>)
}

impl Num {
    pub fn into_inner(self) -> NotNan<f32> {
        match self {
            Num::Int(n) => NotNan::new(n as f32).unwrap(),
            Num::Float(n) => n
        }
    }

    pub fn into_float(self) -> f32 {
        match self {
            Num::Int(n) => n as f32,
            Num::Float(n) => n.into_inner()
        }
    }

    pub fn into_int(self) -> i32 {
        match self {
            Num::Int(n) => n,
            Num::Float(n) => n.into_inner() as i32
        }
    }
}

impl Display for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Num::Int(n) => write!(f, "{}", n),
            Num::Float(n) => write!(f, "{}", n)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Var {
    Int(String),
    Float(String)
}

impl Var {
    pub fn from_variable(variable: Variable) -> Var {
        match variable {
            Variable::Integer(name) => Var::Int(name),
            Variable::Float(name) => Var::Float(name)
        }
    }

    pub fn unwrap(&self) -> (Type, String) {
        match self {
            Var::Int(name) => (Type::Int, name.clone()),
            Var::Float(name) => (Type::Float, name.clone()),
        }
    }
}

impl Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Var::Int(v) => write!(f, "{}", v),
            Var::Float(v) => write!(f, "{}", v)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Dda,
    Sub,
    Bus,
    Mul,
    Lum,
    Div,
    Vid,
    Exp,
    Pxe,
    And,
    Dna,
    Or,
    Ro,
    Eq,
    Qe,
    Neq,
    Qen,
    Lt,
    Tl,
    Gt,
    Tg
}

impl Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinOp::Add => write!(f, "+"),
            BinOp::Dda => write!(f, "+'"),
            BinOp::Sub => write!(f, "-"),
            BinOp::Bus => write!(f, "-'"),
            BinOp::Mul => write!(f, "*"),
            BinOp::Lum => write!(f, "*'"),
            BinOp::Div => write!(f, "/"),
            BinOp::Vid => write!(f, "/'"),
            BinOp::Exp => write!(f, "^"),
            BinOp::Pxe => write!(f, "^'"),
            BinOp::And => write!(f, "&"),
            BinOp::Dna => write!(f, "&'"),
            BinOp::Or => write!(f, "∨"),
            BinOp::Ro => write!(f, "∨'"),
            BinOp::Eq => write!(f, "="),
            BinOp::Qe => write!(f, "="),
            BinOp::Neq => write!(f, "≠"),
            BinOp::Qen => write!(f, "≠"),
            BinOp::Lt => write!(f, "<"),
            BinOp::Tl => write!(f, "<"),
            BinOp::Gt => write!(f, ">"),
            BinOp::Tg => write!(f, ">")
        }
    }
}

impl BinOp {
    pub fn apply(&self, n1: Num, n2: Num) -> Num {
        match self {
            BinOp::Add => match (n1, n2) {
                (Num::Int(n1), Num::Int(n2)) => Num::Int(n1 + n2),
                (Num::Float(n1), Num::Float(n2)) => Num::Float(n1 + n2),
                (Num::Int(n1), Num::Float(n2)) => Num::Int(n1 + n2.into_inner() as i32),
                (Num::Float(n1), Num::Int(n2)) => Num::Float(n1 + n2 as f32),
            },
            BinOp::Sub => match (n1, n2) {
                (Num::Int(n1), Num::Int(n2)) => Num::Int(n1 - n2),
                (Num::Float(n1), Num::Float(n2)) => Num::Float(n1 - n2),
                (Num::Int(n1), Num::Float(n2)) => Num::Int(n1 - n2.into_inner() as i32),
                (Num::Float(n1), Num::Int(n2)) => Num::Float(n1 - n2 as f32),
            },
            BinOp::Mul => match (n1, n2) {
                (Num::Int(n1), Num::Int(n2)) => Num::Int(n1 * n2),
                (Num::Float(n1), Num::Float(n2)) => Num::Float(n1 * n2),
                (Num::Int(n1), Num::Float(n2)) => Num::Int(n1 * n2.into_inner() as i32),
                (Num::Float(n1), Num::Int(n2)) => Num::Float(n1 * n2 as f32),
            },
            BinOp::Div => match (n1, n2) {
                (Num::Int(n1), Num::Int(n2)) => Num::Int(n1 / n2),
                (Num::Float(n1), Num::Float(n2)) => Num::Float(n1 / n2),
                (Num::Int(n1), Num::Float(n2)) => Num::Int(n1 / n2.into_inner() as i32),
                (Num::Float(n1), Num::Int(n2)) => Num::Float(n1 / n2 as f32),
            },
            BinOp::Exp => match (n1, n2) {
                (Num::Int(n1), Num::Int(n2)) => Num::Int(n1.pow(n2 as u32)),
                (Num::Float(n1), Num::Float(n2)) => Num::Float(NotNan::new(n1.into_inner().powf(n2.into_inner())).unwrap()),
                (Num::Int(n1), Num::Float(n2)) => Num::Int(n1.pow(n2.into_inner() as u32)),
                (Num::Float(n1), Num::Int(n2)) => Num::Float(NotNan::new(n1.into_inner().powf(n2 as f32)).unwrap()),
            },
            BinOp::And => match (n1, n2) {
                (Num::Int(n1), Num::Int(n2)) => Num::Int(n1 & n2),
                _ => panic!("Cannot apply AND to non-integer values")
            },
            BinOp::Or => match (n1, n2) {
                (Num::Int(n1), Num::Int(n2)) => Num::Int(n1 | n2),
                _ => panic!("Cannot apply OR to non-integer values")
            },
            BinOp::Eq => match (n1, n2) {
                (Num::Int(n1), Num::Int(n2)) => Num::Int(if n1 == n2 { 1 } else { 0 }),
                (Num::Float(n1), Num::Float(n2)) => Num::Int(if n1 == n2 { 1 } else { 0 }),
                (Num::Int(n1), Num::Float(n2)) => Num::Int(if n1 == n2.into_inner() as i32 { 1 } else { 0 }),
                (Num::Float(n1), Num::Int(n2)) => Num::Int(if n1.into_inner() as i32 == n2 { 1 } else { 0 }),
            },
            BinOp::Neq => match (n1, n2) {
                (Num::Int(n1), Num::Int(n2)) => Num::Int(if n1 != n2 { 1 } else { 0 }),
                (Num::Float(n1), Num::Float(n2)) => Num::Int(if n1 != n2 { 1 } else { 0 }),
                (Num::Int(n1), Num::Float(n2)) => Num::Int(if n1 != n2.into_inner() as i32 { 1 } else { 0 }),
                (Num::Float(n1), Num::Int(n2)) => Num::Int(if n1.into_inner() as i32 != n2 { 1 } else { 0 }),
            },
            BinOp::Lt => match (n1, n2) {
                (Num::Int(n1), Num::Int(n2)) => Num::Int(if n1 < n2 { 1 } else { 0 }),
                (Num::Float(n1), Num::Float(n2)) => Num::Int(if n1 < n2 { 1 } else { 0 }),
                (Num::Int(n1), Num::Float(n2)) => Num::Int(if n1 < n2.into_inner() as i32 { 1 } else { 0 }),
                (Num::Float(n1), Num::Int(n2)) => Num::Int(if n1.into_inner() < n2 as f32 { 1 } else { 0 }),
            },
            BinOp::Gt => match (n1, n2) {
                (Num::Int(n1), Num::Int(n2)) => Num::Int(if n1 > n2 { 1 } else { 0 }),
                (Num::Float(n1), Num::Float(n2)) => Num::Int(if n1 > n2 { 1 } else { 0 }),
                (Num::Int(n1), Num::Float(n2)) => Num::Int(if n1 > n2.into_inner() as i32 { 1 } else { 0 }),
                (Num::Float(n1), Num::Int(n2)) => Num::Int(if n1.into_inner() > n2 as f32 { 1 } else { 0 }),
            },
            _ => panic!("Cannot apply operation to non-number values")
        }
    }

    pub fn is_reverse(&self) -> bool {
        match self {
            BinOp::Dda | BinOp::Bus | BinOp::Lum | BinOp::Vid | BinOp::Pxe | BinOp::Dna | BinOp::Ro | BinOp::Qe | BinOp::Qen | BinOp::Tl | BinOp::Tg => true,
            _ => false
        }
    }

    pub fn inverse(&self) -> BinOp {
        match self {
            BinOp::Add => BinOp::Dda,
            BinOp::Dda => BinOp::Add,
            BinOp::Sub => BinOp::Bus,
            BinOp::Bus => BinOp::Sub,
            BinOp::Mul => BinOp::Lum,
            BinOp::Lum => BinOp::Mul,
            BinOp::Div => BinOp::Vid,
            BinOp::Vid => BinOp::Div,
            BinOp::Exp => BinOp::Pxe,
            BinOp::Pxe => BinOp::Exp,
            BinOp::And => BinOp::Dna,
            BinOp::Dna => BinOp::And,
            BinOp::Or => BinOp::Ro,
            BinOp::Ro => BinOp::Or,
            BinOp::Eq => BinOp::Qe,
            BinOp::Qe => BinOp::Eq,
            BinOp::Neq => BinOp::Qen,
            BinOp::Qen => BinOp::Neq,
            BinOp::Lt => BinOp::Tl,
            BinOp::Tl => BinOp::Lt,
            BinOp::Gt => BinOp::Tg,
            BinOp::Tg => BinOp::Gt
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UnOp {
    Neg,
    Gen,
    Not,
    Ton,
}

impl Display for UnOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnOp::Neg => write!(f, "-"),
            UnOp::Gen => write!(f, "-'"),
            UnOp::Not => write!(f, "¬"),
            UnOp::Ton => write!(f, "¬'")
        }
    }
}

impl UnOp {
    pub fn apply(&self, n: Num) -> Num {
        match self {
            UnOp::Neg => match n {
                Num::Int(n) => Num::Int(-n),
                Num::Float(n) => Num::Float(NotNan::new(-n.into_inner()).unwrap())
            },
            UnOp::Not => match n {
                Num::Int(n) => Num::Int(!n),
                _ => panic!("Cannot apply NOT to non-integer values")
            }
            _ => panic!("Cannot apply operation to non-number values")
        }
    }

    pub fn is_reverse(&self) -> bool {
        match self {
            UnOp::Neg => true,
            _ => false
        }
    }

    pub fn inverse(&self) -> UnOp {
        match self {
            UnOp::Neg => UnOp::Gen,
            UnOp::Gen => UnOp::Neg,
            UnOp::Not => UnOp::Ton,
            UnOp::Ton => UnOp::Not
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum P {
    // expressions
    Num(Num),
    Mun(Num),
    Var(Var),
    Rav(Var),
    BinOp(Box<P>, Box<P>, BinOp),
    UnOp(Box<P>, UnOp),
    Rexp(Box<P>),                  // this is to represent the reverse of an expression
    // commands
    Skip,
    Asgn(Var, Box<P>),
    Ngsa(Var, Box<P>),
    Seq(Box<P>, Box<P>),
    // conditionals
    If(Box<P>, Box<P>, Box<P>),
    // loops
    While(Box<P>, Box<P>, usize),
}

impl P {
    pub fn is_expression(&self) -> bool {
        match self {
            P::Num(_) | P::Mun(_) | P::Var(_) | P::Rav(_) | P::BinOp(_, _, _) | P::UnOp(_, _) | P::Rexp(_) => true,
            _ => false
        }
    }

    pub fn unwrap_num(&self) -> &Num {
        match self {
            P::Num(n) | P::Mun(n) => n,
            _ => panic!("Expected Number, got something else")
        }
    }

    pub fn unwrap_mun(&self) -> &Num {
        match self {
            P::Mun(n) => n,
            _ => panic!("Expected Inverted Number, got something else")
        }
    }

    pub fn unwrap_var(&self) -> &Var {
        match self {
            P::Var(v) | P::Rav(v) => v,
            _ => panic!("Expected Variable, got something else")
        }
    }

    pub fn unwrap_rav(&self) -> &Var {
        match self {
            P::Rav(v) => v,
            _ => panic!("Expected Inverted Variable, got something else")
        }
    }

    pub fn unwrap_binop(&self) -> (&P, &P, &BinOp) {
        match self {
            P::BinOp(e1, e2, op) => (e1, e2, op),
            _ => panic!("Expected Binary Operation, got something else")
        }
    }

    pub fn unwrap_unop(&self) -> (&P, &UnOp) {
        match self {
            P::UnOp(e, op) => (e, op),
            _ => panic!("Expected Unary Operation, got something else")
        }
    }

    pub fn unwrap_rexp(&self) -> &P {
        match self {
            P::Rexp(e) => {
                match e.as_ref() {
                    P::Rexp(_) | P::Mun(_) | P::Rav(_) => e.unwrap_rexp(),
                    _ => e
                }
            },
            P::Mun(_) => self,
            P::Rav(_) => self,
            _ => panic!("Expected Reverse Expression, got something else")
        }
    }

    pub fn unwrap_asgn(&self) -> (&Var, &P) {
        match self {
            P::Asgn(v, e) => (v, e),
            _ => panic!("Expected Assignment, got something else")
        }
    }

    pub fn unwrap_ngsa(&self) -> (&Var, &P) {
        match self {
            P::Ngsa(v, e) => (v, e),
            _ => panic!("Expected Inverted Assignment, got something else")
        }
    }

    pub fn unwrap_seq(&self) -> (&P, &P) {
        match self {
            P::Seq(c1, c2) => (c1, c2),
            _ => panic!("Expected Sequence, got something else")
        }
    }

    pub fn unwrap_if(&self) -> (&P, &P, &P) {
        match self {
            P::If(e, c1, c2) => (e, c1, c2),
            _ => panic!("Expected Conditional, got something else")
        }
    }

    pub fn unwrap_while(&self) -> (&P, &P, usize) {
        match self {
            P::While(e, c, i) => (e, c, *i),
            _ => panic!("Expected Loop, got something else")
        }
    }
}

impl Display for P {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            P::Num(n) => write!(f, "{}", n),
            P::Mun(n) => write!(f, "({})'", n),
            P::Var(v) => write!(f, "!{}", v),
            P::Rav(v) => write!(f, "(!{})'", v),
            P::BinOp(e1, e2, op) => write!(f, "{} {} {}", e1, e2, op),
            P::UnOp(e, op) => write!(f, "{}{}", op, e),
            P::Rexp(e) => write!(f, "({})'", e),
            P::Skip => write!(f, "skip"),
            P::Asgn(v, e) => write!(f, "{} := {}", v, e),
            P::Ngsa(v, e) => write!(f, "{} =: {}", v, e),
            P::Seq(c1, c2) => write!(f, "{}; {}", c1, c2),
            P::If(e, c1, c2) => write!(f, "if {} then {} else {}", e, c1, c2),
            P::While(e, c, i) => write!(f, "while_{} {} do {}", i, e, c),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Lab {
    // expressions
    Oper,
    Exp,
    UnExp,
    BinOp(BinOp),
    BinPo(BinOp),
    UnOp(UnOp),
    UnPo(UnOp),
    // commands
    Asgn,
    Assign,
    Ngsa,
    Ngissa,
    Seq,
    Sequence,
    // conditionals
    Cond,
    If,
    Dnoc,
    Fi,
    // loops
    Loop(usize),
    Pool(usize),
    While(usize),
    Elihw(usize),
    EndW(usize),
}

impl Lab {
    pub fn unwrap_binop(&self) -> &BinOp {
        match self {
            Lab::BinOp(op) | Lab::BinPo(op) => op,
            _ => panic!("Expected Binary Operation Label, got something else")
        }
    }

    pub fn unwrap_binpo(&self) -> &BinOp {
        match self {
            Lab::BinPo(op) => op,
            _ => panic!("Expected Inverted Binary Operation Label, got something else")
        }
    }

    pub fn unwrap_unop(&self) -> &UnOp {
        match self {
            Lab::UnOp(op) | Lab::UnPo(op) => op,
            _ => panic!("Expected Unary Operation Label, got something else")
        }
    }

    pub fn unwrap_unpo(&self) -> &UnOp {
        match self {
            Lab::UnPo(op) => op,
            _ => panic!("Expected Inverted Unary Operation Label, got something else")
        }
    }

    pub fn unwrap_loop(&self) -> usize {
        match self {
            Lab::Loop(i) | Lab::Pool(i) | Lab::While(i) | Lab::Elihw(i) | Lab::EndW(i) => *i,
            _ => panic!("Expected Loop Label, got something else")
        }
    }

    pub fn unwrap_pool(&self) -> usize {
        match self {
            Lab::Pool(i) => *i,
            _ => panic!("Expected Pool Label, got something else")
        }
    }

    pub fn unwrap_while(&self) -> usize {
        match self {
            Lab::While(i) => *i,
            _ => panic!("Expected While Label, got something else")
        }
    }

    pub fn unwrap_elihw(&self) -> usize {
        match self {
            Lab::Elihw(i) => *i,
            _ => panic!("Expected Elihw Label, got something else")
        }
    }

    pub fn unwrap_endw(&self) -> usize {
        match self {
            Lab::EndW(i) => *i,
            _ => panic!("Expected EndW Label, got something else")
        }
    }
}

impl Display for Lab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Lab::Oper => write!(f, "oper"),
            Lab::Exp => write!(f, "exp"),
            Lab::BinOp(op) => write!(f, "{}", op),
            Lab::BinPo(op) => write!(f, "inverse_{}", op),
            Lab::UnExp => write!(f, "unexp"),
            Lab::UnOp(op) => write!(f, "{}", op),
            Lab::UnPo(op) => write!(f, "inverse_{}", op),
            Lab::Asgn => write!(f, "asgn"),
            Lab::Assign => write!(f, ":="),
            Lab::Ngsa => write!(f, "asgnr"),
            Lab::Ngissa => write!(f, "=:"),
            Lab::Seq => write!(f, "seq"),
            Lab::Sequence => write!(f, ";"),
            Lab::Cond => write!(f, "cond"),
            Lab::If => write!(f, "if"),
            Lab::Dnoc => write!(f, "cond_inv"),
            Lab::Fi => write!(f, "if_inv"),
            Lab::Loop(i) => write!(f, "loop_{}", i),
            Lab::Pool(i) => write!(f, "pool_{}", i),
            Lab::While(i) => write!(f, "while_{}", i),
            Lab::Elihw(i) => write!(f, "elihw_{}", i),
            Lab::EndW(i) => write!(f, "end_w_{}", i),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum C {
    P(P),
    Lab(Lab),
}

impl Display for C {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            C::P(p) => write!(f, "{}", p),
            C::Lab(l) => write!(f, "{}", l)
        }
    }
}

impl C {
    pub fn new(p: P) -> C {
        C::P(p)
    }

    pub fn new_lab(l: Lab) -> C {
        C::Lab(l)
    }

    pub fn is_expression(&self) -> bool {
        match self {
            C::P(p) => p.is_expression(),
            _ => false
        }
    }

    pub fn is_reverse_expression(&self) -> bool {
        match self {
            C::P(P::Rexp(_)) => true,
            _ => false
        }
    }

    pub fn unwrap_p(&self) -> &P {
        match self {
            C::P(p) => p,
            _ => panic!("Expected Program, got Lab")
        }
    }

    pub fn unwrap_lab(&self) -> &Lab {
        match self {
            C::Lab(l) => l,
            _ => panic!("Expected Label, got P")
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Stack<T> {
    Nil,
    Cons(T, Box<Stack<T>>),
}

impl<T> Stack<T> where T: Clone {
    pub fn new() -> Stack<T> {
        Stack::Nil
    }

    pub fn push(&mut self, value: T) {
        *self = Stack::Cons(value, Box::new(self.clone()));
    }

    pub fn pop(&mut self) -> Option<T> {
        match self {
            Stack::Nil => None,
            Stack::Cons(value, rest) => {
                let value = value.clone();
                *self = *rest.clone();
                Some(value)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match self {
            Stack::Nil => None,
            Stack::Cons(value, _) => Some(value)
        }
    }

    pub fn peek_n(&self, n: usize) -> Vec<T> {
        let mut result = Vec::new();
        let mut current = self.clone();
        for _ in 0..n {
            match current {
                Stack::Nil => break,
                Stack::Cons(value, rest) => {
                    result.push(value.clone());
                    current = *rest.clone();
                }
            }
        }
        result
    }


    pub fn is_empty(&self) -> bool {
        match self {
            Stack::Nil => true,
            Stack::Cons(_, _) => false
        }
    }
}

impl<T> Display for Stack<T> where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stack::Nil => write!(f, "Nil"),
            Stack::Cons(value, rest) => write!(f, "{} · {}", value, rest)
        }
    }
}

pub type ControlStack = Stack<C>;
pub type BackStack = Stack<C>;

pub struct Builder {
    loop_count: usize,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            loop_count: 0,
        }
    }

    pub fn from_ast(&mut self, ast: Program) -> Stack<C> {
        let ast = transform_if_only(&ast);
        let mut stack = Stack::new();
        match ast {
            Program::Statements(block) => {
                for statement in block.iter().rev() {
                    stack.push(self.c_from_statement(statement.clone()));
                }
            }
        }
        stack
    }

    pub fn c_from_statement(&mut self, statement: Statement) -> C {
        C::P(self.from_statement(statement))
    }

    pub fn from_statement(&mut self, statement: Statement) -> P {
        match statement {
            Statement::Skip => P::Skip,
            Statement::If(condition, then_block, else_block) => {
                P::If(Box::new(self.from_expression(*condition)),
                      Box::new(self.from_block(*then_block)),
                      Box::new(self.from_block(*else_block)))
            },
            Statement::While(condition, block) => {
                self.loop_count += 1;
                P::While(Box::new(self.from_expression(*condition)),
                         Box::new(self.from_block(*block)),
                            self.loop_count - 1)
            },
            Statement::Assignment(variable, expression) => {
                P::Asgn(Var::from_variable(variable), Box::new(self.from_arithmetic_expression(expression)))
            },
            _ => panic!("Unsupported statement")
        }
    }

    pub fn from_expression(&mut self, expression: BooleanExpression) -> P {
        match expression {
            BooleanExpression::Logical(op, e1, e2) => {
                P::BinOp(Box::new(self.from_expression(*e1)),
                         Box::new(self.from_expression(*e2)),
                         match op {
                             BooleanOperator::And => BinOp::And,
                             BooleanOperator::Or => BinOp::Or
                         })
            },
            BooleanExpression::Relational(op, e1, e2) => {
                P::BinOp(Box::new(self.from_arithmetic_expression(*e1)),
                         Box::new(self.from_arithmetic_expression(*e2)),
                         match op {
                             RelationOperator::Equal => BinOp::Eq,
                             RelationOperator::NotEqual => BinOp::Neq,
                             RelationOperator::LessThan => BinOp::Lt,
                             RelationOperator::GreaterThan => BinOp::Gt
                         })
            },
            BooleanExpression::Unary(op, e) => {
                P::UnOp(Box::new(self.from_expression(*e)),
                        match op {
                            UnaryBooleanOperator::Negation => UnOp::Not
                        })
            }
        }
    }

    pub fn from_arithmetic_expression(&mut self, expression: ArithmeticExpression) -> P {
        match expression {
            ArithmeticExpression::Variable(variable) => {
                P::Var(Var::from_variable(variable))
            },
            ArithmeticExpression::Integer(n) => {
                P::Num(Num::Int(n))
            },
            ArithmeticExpression::Float(n) => {
                P::Num(Num::Float(n))
            },
            ArithmeticExpression::Unary(op, e) => {
                P::UnOp(Box::new(self.from_arithmetic_expression(*e)),
                        match op {
                            UnaryArithmeticOperator::Negation => UnOp::Neg
                        })
            },
            ArithmeticExpression::Operation(op, e1, e2) => {
                P::BinOp(Box::new(self.from_arithmetic_expression(*e1)),
                         Box::new(self.from_arithmetic_expression(*e2)),
                         match op {
                             ArithmeticOperator::Addition => BinOp::Add,
                             ArithmeticOperator::Subtraction => BinOp::Sub,
                             ArithmeticOperator::Multiplication => BinOp::Mul,
                             ArithmeticOperator::Division => BinOp::Div,
                             ArithmeticOperator::Exponentiation => BinOp::Exp
                         })
            }
        }
    }

    pub fn from_block(&mut self, block: Vec<Statement>) -> P {
        // if only one statement, return that statement
        if block.len() == 1 {
            self.from_statement(block.first().unwrap().clone())
        } else {
            // if multiple statements, return a sequence of statements
            let mut sequence = self.from_statement(block.last().unwrap().clone());
            for statement in block.iter().rev().skip(1) {
                sequence = P::Seq(Box::new(self.from_statement(statement.clone())), Box::new(sequence));
            }
            sequence
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum R {
    P(P),
    Lab(Lab),
    Value(Num),
    Bool(bool),
    Var(Var)
}

impl R {
    pub fn unwrap_p(&self) -> &P {
        match self {
            R::P(p) => p,
            _ => panic!("Expected Program, got something else")
        }
    }

    pub fn unwrap_lab(&self) -> &Lab {
        match self {
            R::Lab(l) => l,
            _ => panic!("Expected Label, got something else")
        }
    }

    pub fn unwrap_value(&self) -> &Num {
        match self {
            R::Value(n) => n,
            _ => panic!("Expected Value, got something else")
        }
    }

    pub fn unwrap_bool(&self) -> bool {
        match self {
            R::Bool(b) => *b,
            _ => panic!("Expected Boolean, got something else")
        }
    }

    pub fn unwrap_var(&self) -> &Var {
        match self {
            R::Var(v) => v,
            _ => panic!("Expected Variable, got something else")
        }
    }
    
    pub fn is_truthy(&self) -> bool {
        match self {
            R::Value(n) => {
                match n {
                    Num::Int(n) => *n != 0,
                    Num::Float(n) => n.into_inner() != 0.0
                }
            }
            R::Bool(b) => *b,
            _ => false
        }
    }
    
    pub fn is_falsy(&self) -> bool {
        match self {
            R::Value(n) => {
                match n {
                    Num::Int(n) => *n == 0,
                    Num::Float(n) => n.into_inner() == 0.0
                }
            }
            R::Bool(b) => !*b,
            _ => false
        }
    }
}

impl Display for R {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            R::P(p) => write!(f, "{}", p),
            R::Lab(l) => write!(f, "{}", l),
            R::Value(v) => write!(f, "{}", v),
            R::Bool(b) => write!(f, "{}", b),
            R::Var(v) => write!(f, "{}", v)
        }
    }
}

pub type ResultStack = Stack<R>;

// TODO: actually count whiles
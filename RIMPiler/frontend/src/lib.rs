#![allow(non_snake_case)]

use crate::AST::Program;

pub mod AST;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod post_parse;
pub mod JVM;

pub mod abstract_machine;

pub trait Backend {
    fn compile(program: &Program) -> String;
}
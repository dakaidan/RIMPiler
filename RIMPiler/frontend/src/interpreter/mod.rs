mod memory_store;

use memory_store::MemoryStore;
use crate::AST::{ArithmeticExpression, ArithmeticOperator, Assignment, Block, BooleanExpression, BooleanOperator, Program, RelationOperator, Statement, UnaryArithmeticOperator, UnaryBooleanOperator};

pub struct InterpreterEngine {
    memory_store: MemoryStore,
}

impl InterpreterEngine {
    pub fn new() -> Self {
        Self {
            memory_store: MemoryStore::new(),
        }
    }

    pub fn interpret(&mut self, program: &Program) -> Result<(), String> {
        match program {
            Program::Statements(statement) => {
                for statement in statement {
                    let res = self.interpret_statement(statement);

                    if res.is_err() {
                        return Err(res.unwrap_err());
                    }
                }
            }
        }

        println!("\n---- Final memory store ----\n{}\n---- Final memory store ----", self.memory_store);

        Ok(())
    }

    fn interpret_statement(&mut self, statement: &Statement) -> Result<(), String> {
        match statement {
            Statement::Skip => {}
            Statement::Assignment(assignment) => {
                let result = self.interpret_assignment(assignment);

                if result.is_err() {
                    return Err(result.unwrap_err());
                }
            }
            Statement::ReverseAssignment(assignment) => {
                let result = self.interpret_reverse_assignment(assignment);

                if result.is_err() {
                    return Err(result.unwrap_err());
                }
            }
            Statement::ReversePoint => {
                println!("\n---- Reverse point ----\n{}\n---- Reverse point ----", self.memory_store)
            }
            Statement::If(boolean_expression, if_block, else_block) => {
                let result = self.interpret_if(boolean_expression, if_block, else_block);

                if result.is_err() {
                    return Err(result.unwrap_err());
                }
            }
            Statement::While(boolean_expression, block) => {
                let result = self.interpret_while(boolean_expression, block);

                if result.is_err() {
                    return Err(result.unwrap_err());
                }
            }
        }

        Ok(())
    }

    fn interpret_assignment(&mut self, assignment: &Assignment) -> Result<(), String> {
        match assignment {
            Assignment::Integer(variable, expression) => {
                let result = self.interpret_arithmetic_expression(expression);

                if result.is_err() {
                    return Err(result.unwrap_err());
                }

                let value = result.unwrap();

                self.memory_store.assign(variable, value);
            }
        }

        Ok(())
    }

    fn interpret_reverse_assignment(&mut self, assignment: &Assignment) -> Result<(), String> {
        match assignment {
            Assignment::Integer(variable, expression) => {
                let result = self.interpret_arithmetic_expression(expression);

                if result.is_err() {
                    return Err(result.unwrap_err());
                }

                let value = result.unwrap();

                self.memory_store.un_assign(variable, value);
            }
        }

        Ok(())
    }

    fn interpret_if(&mut self, boolean_expression: &BooleanExpression, if_block: &Block, else_block: &Block) -> Result<(), String> {
        let result = self.interpret_boolean_expression(boolean_expression);

        if result.is_err() {
            return Err(result.unwrap_err());
        }

        let value = result.unwrap();

        if value {
            let result = self.interpret_block(if_block);

            if result.is_err() {
                return Err(result.unwrap_err());
            }
        } else {
            let result = self.interpret_block(else_block);

            if result.is_err() {
                return Err(result.unwrap_err());
            }
        }

        Ok(())
    }

    fn interpret_while(&mut self, boolean_expression: &BooleanExpression, block: &Block) -> Result<(), String> {
        let result = self.interpret_boolean_expression(boolean_expression);

        if result.is_err() {
            return Err(result.unwrap_err());
        }

        let mut value = result.unwrap();

        while value {
            let result = self.interpret_block(block);

            if result.is_err() {
                return Err(result.unwrap_err());
            }

            let result = self.interpret_boolean_expression(boolean_expression);

            if result.is_err() {
                return Err(result.unwrap_err());
            }

            value = result.unwrap();
        }

        Ok(())
    }

    fn interpret_block(&mut self, block: &Block) -> Result<(), String> {
        for statement in block {
            let result = self.interpret_statement(statement);

            if result.is_err() {
                return Err(result.unwrap_err());
            }
        }

        Ok(())
    }

    fn interpret_boolean_expression(&mut self, boolean_expression: &BooleanExpression) -> Result<bool, String> {
        match boolean_expression {
            BooleanExpression::Relational(operator, left_hand_side, right_hand_side) => {
                let left_hand_side = self.interpret_arithmetic_expression(left_hand_side);

                if left_hand_side.is_err() {
                    return Err(left_hand_side.unwrap_err());
                }

                let right_hand_side = self.interpret_arithmetic_expression(right_hand_side);

                if right_hand_side.is_err() {
                    return Err(right_hand_side.unwrap_err());
                }

                match operator {
                    RelationOperator::GreaterThan => {
                        Ok(left_hand_side.unwrap() > right_hand_side.unwrap())
                    }
                    RelationOperator::LessThan => {
                        Ok(left_hand_side.unwrap() < right_hand_side.unwrap())
                    }
                    RelationOperator::Equal => {
                        Ok(left_hand_side.unwrap() == right_hand_side.unwrap())
                    }
                    RelationOperator::NotEqual => {
                        Ok(left_hand_side.unwrap() != right_hand_side.unwrap())
                    }
                }
            },
            BooleanExpression::Logical(operator, left_hand_side, right_hand_side) => {
                let left_hand_side = self.interpret_boolean_expression(left_hand_side);

                if left_hand_side.is_err() {
                    return Err(left_hand_side.unwrap_err());
                }

                let right_hand_side = self.interpret_boolean_expression(right_hand_side);

                if right_hand_side.is_err() {
                    return Err(right_hand_side.unwrap_err());
                }

                match operator {
                    BooleanOperator::And => {
                        Ok(left_hand_side.unwrap() && right_hand_side.unwrap())
                    }
                    BooleanOperator::Or => {
                        Ok(left_hand_side.unwrap() || right_hand_side.unwrap())
                    }
                }
            },
            BooleanExpression::Unary(operator, operand) => {
                let operand = self.interpret_boolean_expression(operand);

                if operand.is_err() {
                    return Err(operand.unwrap_err());
                }

                match operator {
                    UnaryBooleanOperator::Negation => {
                        Ok(!operand.unwrap())
                    }
                }
            }
        }
    }

    fn interpret_arithmetic_expression(&mut self, arithmetic_expression: &ArithmeticExpression) -> Result<i32, String> {
        match arithmetic_expression {
            ArithmeticExpression::Variable(variable) => {
                let value = self.memory_store.get(variable);

                if value.is_none() {
                    return Err(format!("Variable {} is not defined", variable));
                }

                Ok(value.unwrap().get())
            }
            ArithmeticExpression::Integer(i) => {
                Ok(i.clone())
            }
            ArithmeticExpression::Unary(operator, operand) => {
                let operand = self.interpret_arithmetic_expression(operand);

                if operand.is_err() {
                    return Err(operand.unwrap_err());
                }

                match operator {
                    UnaryArithmeticOperator::Negation => {
                        Ok(-operand.unwrap())
                    }
                }
            }
            ArithmeticExpression::Operation(operator, left_hand_side, right_hand_side) => {
                let left_hand_side = self.interpret_arithmetic_expression(left_hand_side);

                if left_hand_side.is_err() {
                    return Err(left_hand_side.unwrap_err());
                }

                let right_hand_side = self.interpret_arithmetic_expression(right_hand_side);

                if right_hand_side.is_err() {
                    return Err(right_hand_side.unwrap_err());
                }

                match operator {
                    ArithmeticOperator::Addition => {
                        Ok(left_hand_side.unwrap() + right_hand_side.unwrap())
                    }
                    ArithmeticOperator::Subtraction => {
                        Ok(left_hand_side.unwrap() - right_hand_side.unwrap())
                    }
                    ArithmeticOperator::Multiplication => {
                        Ok(left_hand_side.unwrap() * right_hand_side.unwrap())
                    }
                    ArithmeticOperator::Division => {
                        Ok(left_hand_side.unwrap() / right_hand_side.unwrap())
                    }
                    ArithmeticOperator::Exponentiation => {
                        if right_hand_side.clone().unwrap() > 0 {
                            Ok(left_hand_side.unwrap().pow(right_hand_side.unwrap() as u32))
                        } else {
                            Err(format!("Cannot raise {} to the power of {}", left_hand_side.unwrap(), right_hand_side.unwrap()))
                        }
                    }
                }
            }
        }
    }
}
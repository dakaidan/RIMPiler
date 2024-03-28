pub mod memory_store;
#[cfg(test)]
mod tests;

use crate::interpreter::memory_store::{Integer, MemoryStoreElement, Value};
use crate::AST::{ArithmeticExpression, ArithmeticOperator, Block, BooleanExpression, BooleanOperator, Program, RelationOperator, Statement, UnaryArithmeticOperator, UnaryBooleanOperator, Variable};
use memory_store::MemoryStore;

pub struct InterpreterEngine {
    memory_store: MemoryStore,
    reverse_point_snapshot: Option<MemoryStore>,
    final_memory_point_snapshot: Option<MemoryStore>,
}

impl InterpreterEngine {
    pub fn new() -> Self {
        Self {
            memory_store: MemoryStore::new(),
            reverse_point_snapshot: None,
            final_memory_point_snapshot: None,
        }
    }

    pub fn get_final_memory_point_snapshot(&self) -> &Option<MemoryStore> {
        &self.final_memory_point_snapshot
    }

    pub fn get_reverse_point_snapshot(&self) -> &Option<MemoryStore> {
        &self.reverse_point_snapshot
    }

    pub fn get_result(&self, variable: &String) -> Option<MemoryStoreElement> {
        match &self.reverse_point_snapshot {
            None => None,
            Some(memory_store) => {
                let value = memory_store.get(variable);

                if value.is_none() {
                    return None;
                }

                Some(value.unwrap().clone())
            }
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

        self.final_memory_point_snapshot = Some(self.memory_store.clone());

        Ok(())
    }

    fn interpret_statement(&mut self, statement: &Statement) -> Result<(), String> {
        match statement {
            Statement::Skip => {}
            Statement::Assignment(variable, expression) => {
                let result = self.interpret_assignment(variable, expression);

                if result.is_err() {
                    return Err(result.unwrap_err());
                }
            }
            Statement::ReverseAssignment(variable, expression) => {
                let result = self.interpret_reverse_assignment(variable, expression);

                if result.is_err() {
                    return Err(result.unwrap_err());
                }
            }
            Statement::ReversePoint => {
                self.reverse_point_snapshot = Some(self.memory_store.clone());
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

    fn interpret_assignment(&mut self, variable: &Variable, expression: &ArithmeticExpression) -> Result<(), String> {
        match variable {
            Variable::Integer(variable) => {
                let result = self.interpret_arithmetic_expression(expression);

                if result.is_err() {
                    return Err(result.unwrap_err());
                }

                let value = result.unwrap();

                self.memory_store.assign(variable, value);
            }
            Variable::Float(variable) => {
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

    fn interpret_reverse_assignment(&mut self, variable: &Variable, expression: &ArithmeticExpression) -> Result<(), String> {
        match variable {
            Variable::Integer(variable) => {
                let result = self.interpret_arithmetic_expression(expression);

                if result.is_err() {
                    return Err(result.unwrap_err());
                }

                let value = result.unwrap();

                self.memory_store.un_assign(variable, value);
            }
            Variable::Float(variable) => {
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

    fn interpret_if(
        &mut self,
        boolean_expression: &BooleanExpression,
        if_block: &Block,
        else_block: &Block,
    ) -> Result<(), String> {
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

    fn interpret_while(
        &mut self,
        boolean_expression: &BooleanExpression,
        block: &Block,
    ) -> Result<(), String> {
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

    fn interpret_boolean_expression(
        &mut self,
        boolean_expression: &BooleanExpression,
    ) -> Result<bool, String> {
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
            }
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
                    BooleanOperator::And => Ok(left_hand_side.unwrap() && right_hand_side.unwrap()),
                    BooleanOperator::Or => Ok(left_hand_side.unwrap() || right_hand_side.unwrap()),
                }
            }
            BooleanExpression::Unary(operator, operand) => {
                let operand = self.interpret_boolean_expression(operand);

                if operand.is_err() {
                    return Err(operand.unwrap_err());
                }

                match operator {
                    UnaryBooleanOperator::Negation => Ok(!operand.unwrap()),
                }
            }
        }
    }

    fn interpret_arithmetic_expression(
        &mut self,
        arithmetic_expression: &ArithmeticExpression,
    ) -> Result<Value, String> {
        match arithmetic_expression {
            ArithmeticExpression::Variable(variable) => {
                let value = match variable {
                    Variable::Integer(variable) => self.memory_store.get(variable),
                    Variable::Float(variable) => self.memory_store.get(variable),
                };

                if value.is_none() {
                    return Err(format!("Variable {} is not defined", variable));
                }

                Ok(value.unwrap().get())
            }
            ArithmeticExpression::Integer(i) => Ok(Value::Integer(*i)),
            ArithmeticExpression::Float(f) => Ok(Value::Float(f.into_inner() as f32)),
            ArithmeticExpression::Unary(operator, operand) => {
                let operand = self.interpret_arithmetic_expression(operand);

                if operand.is_err() {
                    return Err(operand.unwrap_err());
                }

                match operator {
                    UnaryArithmeticOperator::Negation => Ok(-operand.unwrap()),
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
                        if right_hand_side.clone().unwrap() > Value::Integer(0) {
                            match left_hand_side.unwrap() {
                                Value::Integer(lhs) => {
                                    match right_hand_side.unwrap() {
                                        Value::Integer(rhs) => {
                                            Ok(Value::Integer(lhs.pow(rhs as u32)))
                                        }
                                        Value::Float(rhs) => {
                                            Ok(Value::Float((lhs as f32).powf(rhs)))
                                        }
                                    }
                                }
                                Value::Float(lhs) => {
                                    match right_hand_side.unwrap() {
                                        Value::Integer(rhs) => {
                                            Ok(Value::Float(lhs.powf(rhs as f32)))
                                        }
                                        Value::Float(rhs) => {
                                            Ok(Value::Float(lhs.powf(rhs)))
                                        }
                                    }
                                }
                            }
                        } else {
                            Err(format!(
                                "Cannot raise {} to the power of {}",
                                left_hand_side.unwrap(),
                                right_hand_side.unwrap()
                            ))
                        }
                    }
                }
            }
        }
    }
}

use std::collections::HashMap;
use std::fmt::Display;
use ordered_float::NotNan;

pub trait MemoryStoreTrait<T> {
    fn get(&self) -> T;
    fn get_history(&self) -> Vec<T>;
    fn assign(&mut self, value: T);
    fn un_assign(&mut self, value: T);
}

#[derive(Debug, Clone, PartialEq)]
pub struct Integer {
    value: i32,
    history: Vec<i32>,
}

impl Default for Integer {
    fn default() -> Self {
        Self {
            value: 0,
            history: Vec::from([0]),
        }
    }
}

impl Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut history = String::new();
        for (index, value) in self.history.iter().enumerate() {
            if index == 0 {
                history.push_str(&format!("{}", value));
            } else {
                history.push_str(&format!(" + {}", value));
            }
        }
        write!(f, "{}:    {}", self.value, history)
    }
}

impl MemoryStoreTrait<i32> for Integer {
    fn get(&self) -> i32 {
        self.value
    }

    fn get_history(&self) -> Vec<i32> {
        self.history.clone()
    }

    fn assign(&mut self, value: i32) {
        self.history.push(value - self.value);
        self.value = value;
    }

    fn un_assign(&mut self, _: i32) {
        self.value = self.value - self.history.last().unwrap();
        self.history.pop();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Float {
    value: NotNan<f32>,
    history: Vec<NotNan<f32>>,
}

impl Default for Float {
    fn default() -> Self {
        Self {
            value: NotNan::new(0.0).unwrap(),
            history: Vec::from([NotNan::new(0.0).unwrap()]),
        }
    }
}

impl Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut history = String::new();
        for (index, value) in self.history.iter().enumerate() {
            if index == 0 {
                history.push_str(&format!("{}", value));
            } else {
                history.push_str(&format!(" + {}", value));
            }
        }
        write!(f, "{}:    {}", self.value, history)
    }
}

impl MemoryStoreTrait<f32> for Float {
    fn get(&self) -> f32 {
        self.value.into_inner()
    }

    fn get_history(&self) -> Vec<f32> {
        self.history.clone().iter().map(|value| value.into_inner()).collect()
    }

    fn assign(&mut self, value: f32) {
        self.history.push(NotNan::new(value - self.value.into_inner()).unwrap());
        self.value = NotNan::new(value).unwrap();
    }

    fn un_assign(&mut self, _: f32) {
        self.value = self.value - self.history.last().unwrap();
        self.history.pop();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MemoryStoreElement {
    Integer(Integer),
    Float(Float),
}

impl From<Value> for MemoryStoreElement {
    fn from(value: Value) -> Self {
        match value {
            Value::Integer(value) => MemoryStoreElement::Integer(Integer {
                value,
                history: Vec::from([value]),
            }),
            Value::Float(value) => MemoryStoreElement::Float(Float {
                value: NotNan::new(value).unwrap(),
                history: Vec::from([NotNan::new(value).unwrap()]),
            }),
        }
    }
}

impl Display for MemoryStoreElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryStoreElement::Integer(integer) => write!(f, "{}", integer),
            MemoryStoreElement::Float(float) => write!(f, "{}", float),
        }
    }
}

impl MemoryStoreElement {
    pub fn get(&self) -> Value {
        match self {
            MemoryStoreElement::Integer(integer) => Value::Integer(integer.get()),
            MemoryStoreElement::Float(float) => Value::Float(float.get()),
        }
    }

    pub fn get_history(&self) -> Vec<Value> {
        match self {
            MemoryStoreElement::Integer(integer) => {
                integer.get_history().iter().map(|value| Value::Integer(*value)).collect()
            }
            MemoryStoreElement::Float(float) => {
                float.get_history().iter().map(|value| Value::Float(*value)).collect()
            }
        }
    }

    pub fn assign(&mut self, value: Value) {
        match self {
            MemoryStoreElement::Integer(integer) => {
                if let Value::Integer(value) = value {
                    integer.assign(value);
                } else {
                    panic!("Value is not an integer");
                }
            }
            MemoryStoreElement::Float(float) => {
                if let Value::Float(value) = value {
                    float.assign(value);
                } else {
                    panic!("Value is not a float");
                }
            }
        }
    }

    pub fn un_assign(&mut self, value: Value) {
        match self {
            MemoryStoreElement::Integer(integer) => {
                if let Value::Integer(value) = value {
                    integer.un_assign(value);
                } else {
                    panic!("Value is not an integer");
                }
            }
            MemoryStoreElement::Float(float) => {
                if let Value::Float(value) = value {
                    float.un_assign(value);
                } else {
                    panic!("Value is not a float");
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i32),
    Float(f32),
}

// implement the following operators for Value
// <, >, ==, !=
// +, -, *, /, ^
// unary -
impl std::ops::Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        match self {
            Value::Integer(lhs) => match rhs {
                Value::Integer(rhs) => Value::Integer(lhs + rhs),
                Value::Float(rhs) => Value::Float(lhs as f32 + rhs),
            },
            Value::Float(lhs) => match rhs {
                Value::Integer(rhs) => Value::Float(lhs + rhs as f32),
                Value::Float(rhs) => Value::Float(lhs + rhs),
            },
        }
    }
}

impl std::ops::Sub for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        match self {
            Value::Integer(lhs) => match rhs {
                Value::Integer(rhs) => Value::Integer(lhs - rhs),
                Value::Float(rhs) => Value::Float(lhs as f32 - rhs),
            },
            Value::Float(lhs) => match rhs {
                Value::Integer(rhs) => Value::Float(lhs - rhs as f32),
                Value::Float(rhs) => Value::Float(lhs - rhs),
            },
        }
    }
}

impl std::ops::Mul for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        match self {
            Value::Integer(lhs) => match rhs {
                Value::Integer(rhs) => Value::Integer(lhs * rhs),
                Value::Float(rhs) => Value::Float(lhs as f32 * rhs),
            },
            Value::Float(lhs) => match rhs {
                Value::Integer(rhs) => Value::Float(lhs * rhs as f32),
                Value::Float(rhs) => Value::Float(lhs * rhs),
            },
        }
    }
}

impl std::ops::Div for Value {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        match self {
            Value::Integer(lhs) => match rhs {
                Value::Integer(rhs) => Value::Integer(lhs / rhs),
                Value::Float(rhs) => Value::Float(lhs as f32 / rhs),
            },
            Value::Float(lhs) => match rhs {
                Value::Integer(rhs) => Value::Float(lhs / rhs as f32),
                Value::Float(rhs) => Value::Float(lhs / rhs),
            },
        }
    }
}

impl std::ops::Neg for Value {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Value::Integer(value) => Value::Integer(-value),
            Value::Float(value) => Value::Float(-value),
        }
    }
}

impl std::cmp::PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Value::Integer(lhs) => match other {
                Value::Integer(rhs) => lhs == rhs,
                Value::Float(rhs) => *lhs as f32 == *rhs,
            },
            Value::Float(lhs) => match other {
                Value::Integer(rhs) => *lhs == *rhs as f32,
                Value::Float(rhs) => *lhs == *rhs,
            },
        }
    }
}

impl std::cmp::PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            Value::Integer(lhs) => match other {
                Value::Integer(rhs) => lhs.partial_cmp(rhs),
                Value::Float(rhs) => (*lhs as f32).partial_cmp(rhs),
            },
            Value::Float(lhs) => match other {
                Value::Integer(rhs) => lhs.partial_cmp(&(*rhs as f32)),
                Value::Float(rhs) => lhs.partial_cmp(rhs),
            },
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(value) => write!(f, "{}", value),
            Value::Float(value) => write!(f, "{}", value),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemoryStore {
    memory: HashMap<String, MemoryStoreElement>,
}

impl Display for MemoryStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut memory = String::new();
        for (index, (variable, value)) in self.memory.iter().enumerate() {
            if index == 0 {
                memory.push_str(&format!("{}: {}", variable, value));
            } else {
                memory.push_str(&format!("\n{}: {}", variable, value));
            }
        }
        write!(f, "{}", memory)
    }
}

impl MemoryStore {
    pub fn new() -> Self {
        Self {
            memory: HashMap::new(),
        }
    }

    pub fn get(&self, variable: &String) -> Option<&MemoryStoreElement> {
        self.memory.get(variable)
    }

    pub fn assign(&mut self, variable: &String, value: Value) {
        if let Some(element) = self.memory.get_mut(variable) {
            element.assign(value.into());
        } else {
            match value {
                Value::Integer(integer) => {
                    let mut element = Integer::default();
                    element.assign(integer);
                    self.memory.insert(variable.clone(), MemoryStoreElement::Integer(element));
                }
                Value::Float(float) => {
                    let mut element = Float::default();
                    element.assign(float);
                    self.memory.insert(variable.clone(), MemoryStoreElement::Float(element));
                }
            }
        }
    }

    pub fn un_assign(&mut self, variable: &String, value: Value) {
        if let Some(element) = self.memory.get_mut(variable) {
            element.un_assign(value);
        } else {
            panic!("Variable {} not found in memory", variable);
        }
    }
}

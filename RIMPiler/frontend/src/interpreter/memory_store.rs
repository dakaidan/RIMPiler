use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Integer {
    value: i32,
    history: Vec<i32>,
}

impl Default for Integer {
    fn default() -> Self {
        Self {
            value: 0,
            history: Vec::from([0])
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

impl Integer {
    pub fn get(&self) -> i32 {
        self.value
    }

    pub fn get_history(&self) -> Vec<i32> {
        self.history.clone()
    }

    pub fn assign(&mut self, value: i32) {
        self.history.push(value - self.value);
        self.value = value;
    }

    pub fn un_assign(&mut self, value: i32) {
        self.value = self.value - self.history.last().unwrap();
        self.history.pop();
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MemoryStore {
    memory: HashMap<String, Integer>,
}

impl Display for MemoryStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut memory = String::new();
        for (index, (variable, integer)) in self.memory.iter().enumerate() {
            if index == 0 {
                memory.push_str(&format!("{}: {}", variable, integer));
            } else {
                memory.push_str(&format!("\n{}: {}", variable, integer));
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

    pub fn get(&self, variable: &String) -> Option<&Integer> {
        self.memory.get(variable)
    }

    pub fn assign(&mut self, variable: &String, value: i32) {
        if let Some(integer) = self.memory.get_mut(variable) {
            integer.assign(value);
        } else {
            let mut integer = Integer::default();
            integer.assign(value);
            self.memory.insert(variable.clone(), integer);
        }
    }

    pub fn un_assign(&mut self, variable: &String, value: i32) {
        if let Some(integer) = self.memory.get_mut(variable) {
            integer.un_assign(value);
        } else {
            panic!("Variable {} not found in memory", variable);
        }
    }
}
use std::rc::Rc;
use crate::tree::{FuncNode};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Bool(bool),
    I32(i32),
    F32(f32),
    Chars(String),
    Func(Rc<FuncNode>, usize),
}

impl Value {
    pub fn print(&self) {
        println!("{self:?}");
    }
}

// this is needed for logical operators
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Nil, Value::Nil) => true,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::I32(a), Value::I32(b)) => a == b,
            (Value::F32(a), Value::F32(b)) => a == b,
            (Value::Chars(a), Value::Chars(b)) => a == b,
            _ => false
        }
    }
}

// this is needed for logical operators
impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Nil, Value::Nil) => Some(std::cmp::Ordering::Equal),
            (Value::Bool(a), Value::Bool(b)) => a.partial_cmp(b),
            (Value::I32(a), Value::I32(b)) => a.partial_cmp(b),
            (Value::F32(a), Value::F32(b)) => a.partial_cmp(b),
            (Value::Chars(a), Value::Chars(b)) => a.partial_cmp(b),
            _ => None
        }
    }
}
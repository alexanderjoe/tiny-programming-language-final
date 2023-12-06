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
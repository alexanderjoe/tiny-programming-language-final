use std::alloc::System;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::logger::Logger;
use crate::symbols::Symbols;
use crate::tree::Parameter;
use crate::value::Value;

pub struct Frame {
    globals: Option<Rc<RefCell<Frame>>>,
    values: HashMap<String, Value>,
    parent: Option<Rc<RefCell<Frame>>>,
}

impl Frame {
    pub fn new(global: Option<Rc<RefCell<Frame>>>, parent:Option<Rc<RefCell<Frame>>>) -> Frame {
        Frame {
            globals: global,
            values: HashMap::new(),
            parent: parent,
        }
    }

    pub fn get_globals(&self) -> Option<Rc<RefCell<Frame>>> {
        self.globals.clone()
    }

    pub fn init_symbols(&mut self, symbols: &Symbols) {
        for (name, symbol) in &symbols.map {
            self.values.insert(name.clone(), symbol.value.clone());
        }
    }

    pub fn init_parameters(&mut self, parameters: &Vec<Parameter>, arguments: Vec<Value>) {
        assert_eq!(parameters.len(), arguments.len());

        let mut iter_args = arguments.into_iter();

        for rc_param in parameters {
            let name = rc_param.name.clone();
            let arg = iter_args.next().unwrap();
            self.values.insert(name, arg);
        }
    }

    pub fn assign(&mut self, name: &String, value: Value) {
        // if self.lookup(name)!=Value::Nil{
        //     print!("YIPPEE");
        // }
        self.values.insert(name.clone(), value);
    }

    pub fn lookup(&self, name: &String) -> Value {
        match self.values.get(name) {
            None => { if self.parent.is_some(){
                self.lookup_parent(name)
            } else{
                self.lookup_global(name)
            } }
            Some(value) => { value.clone() }
        }
    }

    pub fn lookup_global(&self, name: &String) -> Value {
        match &self.globals {
            None => { Value::Nil }
            Some(rc_globals) => {
                rc_globals.borrow().lookup(name)
            }
        }
    }

    pub fn lookup_parent(&self, name: &String) -> Value {
        // print!("LOOKING UP PARENT");
        match &self.parent{
            None => { Value::Nil }
            Some(parent) => {
                parent.borrow().lookup(name)
            }
        }
    }

    pub fn print(&self) {
        for (name, value) in &self.values {
            Logger::debug(&format!("    {name} = {value:?}", name = name, value = value));
        }
    }
}
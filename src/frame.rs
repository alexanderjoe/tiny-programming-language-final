use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::logger::Logger;
use crate::symbols::Symbols;
use crate::tree::Parameter;
use crate::value::Value;

pub struct Frame {
    global: Option<Rc<RefCell<Frame>>>,
    values: HashMap<String, Value>,
}

impl Frame {

    pub fn new(global: Option<Rc<RefCell<Frame>>>) -> Frame {
        Frame {
            global,
            values: HashMap::new(),
        }
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
        self.values.insert(name.clone(), value);
    }

    pub fn lookup(&self,  name: &String) -> Value {
        match self.values.get(name) {
            None => { Value::Nil }
            Some(value) => { value.clone() }
        }
    }

    pub fn lookup_global(&self,  name: &String) -> Value {
        match &self.global {
            None => { Value::Nil }
            Some(rc_globals) => {
                rc_globals.borrow().lookup(name)
            }
        }
    }

    pub fn print(& self) {
        for (name, value) in &self.values {
            Logger::debug(&format!("    {name} = {value:?}",
                name = name,
                value = value,
            ));
        }
    }
}
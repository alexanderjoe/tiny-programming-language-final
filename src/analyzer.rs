use std::rc::Rc;
use crate::logger::Logger;
use crate::symbols::{Symbol};
use crate::tree::{ProgramNode};
use crate::value::Value;


pub struct Analyzer {
    program: Rc<ProgramNode>,
}

impl Analyzer {

    pub fn new(program: Rc<ProgramNode>) -> Analyzer {
        Analyzer { program }
    }

    pub fn analyze(&self) {
        Logger::info("Analyze.");
        self.collect_symbols_program();
        self.reference_symbols_program();
    }

    fn collect_symbols_program(&self) {

        // get program node symbol table
        let rc_symbols = self.program.symbols.clone();
        let mut symbols = rc_symbols.borrow_mut();

        // collect function node symbols
        for rc_func in &self.program.func_nodes {
            let name = &rc_func.name;
            let num_params = rc_func.parameters.len();
            let symbol = Symbol::new(
                name.clone(),
                Value::Func(rc_func.clone(), num_params),
                num_params);
            match symbols.map.insert(name.clone(), symbol) {
                None => { /* all good */  }
                Some(_) => { panic!("Duplicate identifier '{:}'!", name) }
            }
        }

        // collect let node symbols
        for rc_let in &self.program.let_nodes {
            let name = &rc_let.name;
            let symbol = Symbol::new(name.clone(), Value::Nil, 0);
            match symbols.map.insert(name.clone(), symbol) {
                None => { /* all good */  }
                Some(_) => { panic!("Duplicate identifier '{:}'!", name) }
            }
        }

    }

    fn reference_symbols_program(&self) {
        // TODO
    }
}

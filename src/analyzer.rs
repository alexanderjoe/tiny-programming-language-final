use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use crate::logger::Logger;
use crate::symbols::{Symbol, Symbols};
use crate::tree::{FuncNode, ProgramNode, StmtNode};
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
                None => {
                    /* all good */
                    Self::collect_symbols_block_function(
                        rc_func.clone(),
                        rc_symbols.clone(),
                    );
                }
                Some(_) => { panic!("Duplicate identifier '{:}'!", name) }
            }
        }

        // collect let node symbols
        for rc_let in &self.program.let_nodes {
            let name = &rc_let.name;
            let symbol = Symbol::new(name.clone(), Value::Nil, 0);
            match symbols.map.insert(name.clone(), symbol) {
                None => { /* all good */ }
                Some(_) => { panic!("Duplicate identifier '{:}'!", name) }
            }
        }
    }

    fn collect_symbols_block_function(rc_func: Rc<FuncNode>, rc_symbols_global: Rc<RefCell<Symbols>>) {

        // get function node symbol table
        let rc_symbols = rc_func.block_node.symbols.clone();
        let mut symbols = rc_symbols.borrow_mut();

        // link to global symbols table
        symbols.parent = Some(rc_symbols_global);

        // collect parameter symbols
        for param in &rc_func.parameters {
            let name = &param.name;
            let symbol = Symbol::new(name.clone(), Value::Nil, 0);
            match symbols.map.insert(name.clone(), symbol) {
                None => { /* all good */ }
                Some(_) => { panic!("Duplicate parameter name '{:}' in function {:}!", name, rc_func.name) }
            }
        }

        // collect let node symbols
        for rc_stmt in &rc_func.block_node.statements {
            if let StmtNode::Let(letNode) = rc_stmt.deref() {
                let name = &letNode.name;
                let symbol = Symbol::new(name.clone(), Value::Nil, 0);
                match symbols.map.insert(name.clone(), symbol) {
                    None => { /* all good */ }
                    Some(_) => { panic!("Duplicate parameter name '{:}' in function {:}!", name, rc_func.name) }
                }
            }
        }
    }

    fn reference_symbols_program(&self) {
        // TODO
    }
}

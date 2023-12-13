use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use crate::logger::Logger;
use crate::symbols::{Symbol, Symbols};
use crate::tree::{ExprNode, FuncNode, ProgramNode, StmtNode};
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
        self.check_unused_variables_program();
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
            Logger::debug(&format!("Collecting symbol '{:}'.", name));
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
        for rc_func in &self.program.func_nodes {
            self.reference_symbols_block_function(rc_func.clone());
        }
    }

    fn reference_symbols_block_function(&self, rc_func: Rc<FuncNode>) {
        let rc_symbols = rc_func.block_node.symbols.clone();
        let mut symbols = rc_symbols.borrow_mut();

        for rc_stmt in &rc_func.block_node.statements {
            match rc_stmt.deref() {
                StmtNode::Let(letNode) => {
                    if !symbols.map.contains_key(&letNode.name) {
                        panic!("Variable '{:}' used before declaration in function {:}!", letNode.name, rc_func.name);
                    }
                }
                StmtNode::Assign(assignNode) => {
                    if !symbols.map.contains_key(&assignNode.name) {
                        panic!("Variable '{:}' used before declaration in function {:}!", assignNode.name, rc_func.name);
                    }
                    if let Some(symbol) = symbols.map.get_mut(&assignNode.name) {
                        symbol.is_used = true;
                    }
                    self.reference_symbols_expression(&assignNode.expr, &mut symbols);
                }
                StmtNode::IfElse(ifNode) => {
                    self.reference_symbols_expression(&ifNode.condition, &mut symbols);
                }
                StmtNode::Return(returnNode) => {
                    self.reference_symbols_expression(&returnNode.expr, &mut symbols);
                }
                StmtNode::While(whileNode) => {
                    self.reference_symbols_expression(&whileNode.condition, &mut symbols);
                }
                _ => {}
            }
        }
    }

    fn reference_symbols_expression(&self, expr: &ExprNode, symbols: &mut Symbols) {
        match expr {
            ExprNode::Var(varNode) => {
                if let Some(symbol) = symbols.map.get_mut(varNode) {
                    symbol.is_used = true;
                }
            }
            ExprNode::Call(callNode, args) => {
                if let Some(symbol) = symbols.map.get_mut(callNode) {
                    symbol.is_used = true;
                }
                for expr in args {
                    self.reference_symbols_expression(expr, symbols);
                }
            }
            ExprNode::Add(expr1, expr2) => {
                self.reference_symbols_expression(expr1, symbols);
                self.reference_symbols_expression(expr2, symbols);
            }
            ExprNode::Sub(expr1, expr2) => {
                self.reference_symbols_expression(expr1, symbols);
                self.reference_symbols_expression(expr2, symbols);
            }
            ExprNode::Mul(expr1, expr2) => {
                self.reference_symbols_expression(expr1, symbols);
                self.reference_symbols_expression(expr2, symbols);
            }
            ExprNode::EqualTo(expr1, expr2) => {
                self.reference_symbols_expression(expr1, symbols);
                self.reference_symbols_expression(expr2, symbols);
            }
            ExprNode::NotEqualTo(expr1, expr2) => {
                self.reference_symbols_expression(expr1, symbols);
                self.reference_symbols_expression(expr2, symbols);
            }
            ExprNode::GreaterThan(expr1, expr2) => {
                self.reference_symbols_expression(expr1, symbols);
                self.reference_symbols_expression(expr2, symbols);
            }
            ExprNode::LessThan(expr1, expr2) => {
                self.reference_symbols_expression(expr1, symbols);
                self.reference_symbols_expression(expr2, symbols);
            }
            ExprNode::GreaterThanEq(expr1, expr2) => {
                self.reference_symbols_expression(expr1, symbols);
                self.reference_symbols_expression(expr2, symbols);
            }
            ExprNode::LessThanEq(expr1, expr2) => {
                self.reference_symbols_expression(expr1, symbols);
                self.reference_symbols_expression(expr2, symbols);
            }
            _ => {}
        }
    }

    fn check_unused_variables_program(&self) {
        for rc_func in &self.program.func_nodes {
            self.check_unused_variables_block_function(rc_func.clone());
        }
    }

    fn check_unused_variables_block_function(&self, rc_func: Rc<FuncNode>) {
        let rc_symbols = rc_func.block_node.symbols.clone();
        let symbols = rc_symbols.borrow();

        for (name, symbol) in symbols.map.iter() {
            if !symbol.is_used {
                Logger::warn(&format!("Warning: Variable '{name:}' declared but not used in function {func:}!", name = name, func = rc_func.name));
            }
        }
    }
}

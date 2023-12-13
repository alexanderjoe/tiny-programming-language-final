use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use crate::evaluator::Evaluator;
use crate::frame::Frame;
use crate::logger::Logger;
use crate::tree::{BlockNode, FuncNode, ProgramNode, StmtNode};
use crate::value::Value;

enum Control {
    Next,
    Return,
    Break,
    Continue,
}

pub struct Executor {
    program: Rc<ProgramNode>,
}

impl Executor {
    pub fn new(program: Rc<ProgramNode>) -> Executor {
        Executor { program }
    }

    pub fn execute(&self) {
        Logger::info("Execute.");
        self.execute_program();
    }

    fn execute_program(&self) {
        Logger::info("Execute Program.");

        // get program node symbol table
        let rc_symbols = self.program.symbols.clone();
        let symbols = rc_symbols.borrow();

        // find main function node
        let rc_main = if let Some(main) = symbols.map.get("main") {
            match &main.value {
                Value::Func(rc_main, _) => { rc_main.clone() }
                _ => { panic!("Symbol 'main' is not a function!"); }
            }
        } else {
            panic!("Cannot find 'main' symbol!");
        };

        // create global stack frame
        let mut global = Frame::new(None);
        global.init_symbols(symbols.deref());
        let rc_global = Rc::new(RefCell::new(global));

        // execute main function
        // todo: could probably actually accept arguments here to pass to main
        let arguments = vec![Value::I32(1)];
        Self::execute_function(rc_main, rc_global, arguments);

        Logger::info("Program finished.");
    }

    pub fn execute_function(rc_func: Rc<FuncNode>, globals: Rc<RefCell<Frame>>, arguments: Vec<Value>) -> Value {
        let name = &rc_func.name;
        Logger::debug(&format!("calling function '{name}'.", name = name));

        // create local stack frame
        let mut locals = Frame::new(Some(globals));

        // initialize parameters
        let name = &rc_func.name;
        if rc_func.numParameters() > arguments.len() {
            panic!("Not enough arguments for function {name}!");
        }
        if rc_func.numParameters() < arguments.len() {
            panic!("To many arguments for function {name}!");
        }
        locals.init_parameters(&rc_func.parameters, arguments);

        // execute function block
        let rc_block = rc_func.block_node.clone();
        let rc_locals = Rc::new(RefCell::new(locals));
        let (_, value) = Self::execute_block_without_scope(rc_block, rc_locals);

        value
    }

    fn execute_block_without_scope(rc_block: Rc<BlockNode>, rc_locals: Rc<RefCell<Frame>>) -> (Control, Value) {
        // execute statements
        for statement in &rc_block.statements {
            let (control, value) = Self::execute_statement(
                statement.clone(),
                rc_locals.clone(),
            );
            match control {
                Control::Next => {}
                Control::Return => { return (Control::Return, value); }
                Control::Break => {}
                Control::Continue => {}
            }
        }

        (Control::Next, Value::Nil)
    }

    fn execute_statement(
        rc_statement: Rc<StmtNode>,
        rc_locals: Rc<RefCell<Frame>>,
    ) -> (Control, Value)
    {
        match rc_statement.deref() {
            StmtNode::Let(_) => {
                Logger::debug("ignoring let statement");
                (Control::Next, Value::Nil)
            }
            StmtNode::Assign(assign) => {
                Logger::debug("executing assign statement");
                let name = &assign.name;
                let value = Evaluator::evaluate(assign.expr.clone(), rc_locals.clone());
                rc_locals.borrow_mut().assign(name, value);
                (Control::Next, Value::Nil)
            }
            StmtNode::Return(ret) => {
                Logger::debug("executing return statement");
                let value = Evaluator::evaluate(ret.expr.clone(), rc_locals.clone());
                (Control::Return, value)
            }
            StmtNode::Print(print) => {
                Logger::debug("executing print statement");
                let value = Evaluator::evaluate(print.expr.clone(), rc_locals.clone());
                value.print();
                (Control::Next, Value::Nil)
            }
            StmtNode::While(while_node) => {
                Logger::debug("executing while statement");
                while Evaluator::evaluate(while_node.condition.clone(), rc_locals.clone()) == Value::Bool(true) {
                    Self::execute_block_without_scope(while_node.body.clone(), rc_locals.clone());
                }
                (Control::Next, Value::Nil)
            }
            StmtNode::IfElse(if_else_node) => {
                Logger::debug("executing if else statement");
                let condition = Evaluator::evaluate(if_else_node.condition.clone(), rc_locals.clone());
                if let Value::Bool(b) = condition {
                    if b {
                        Logger::debug("executing if body");
                        return Self::execute_block_without_scope(if_else_node.ifBody.clone(), rc_locals.clone());
                    }
                    if !b && if_else_node.elseBody.is_some() {
                        Logger::debug("executing else body");
                        return Self::execute_block_without_scope(if_else_node.elseBody.clone().unwrap(), rc_locals.clone());
                    }
                    (Control::Next, Value::Nil)
                } else {
                    panic!("If-then-else statement condition must be of type boolean!");
                }
            }
        }
    }
}
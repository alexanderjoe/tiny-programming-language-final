use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use crate::evaluator::Evaluator;
use crate::frame::Frame;
use crate::tree::{BlockNode, FuncNode, ProgramNode, StmtNode};
use crate::value::Value;

pub struct Executor {
    program: Rc<ProgramNode>,
}

impl Executor {
    pub fn new(program: Rc<ProgramNode>) -> Executor {
        Executor { program }
    }

    pub fn execute(&self) {
        println!("[info] Execute.");
        self.execute_program();
    }

    fn execute_program(&self) {
        println!("[info] Execute Program.");

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
        let arguments = vec![Value::I32(1)];
        Self::execute_function(rc_main, rc_global, arguments);
    }

    pub fn execute_function(
        rc_func: Rc<FuncNode>,
        frame: Rc<RefCell<Frame>>,
        arguments: Vec<Value>,
    ) -> Value
    {
        let name = &rc_func.name;
        println!("[debug] calling function '{name}'.");

        // create local stack frame
        let mut locals = Frame::new(Some(frame));

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
        let return_value = Self::execute_block(rc_block, rc_locals);

        return_value
    }

    fn execute_block(
        rc_block: Rc<BlockNode>,
        rc_locals: Rc<RefCell<Frame>>,
    ) -> Value {
        // get block node symbol table
        let rc_symbols = rc_block.symbols.clone();
        let symbols = rc_symbols.borrow();

        // initialize local frame
        rc_locals.borrow_mut().init_symbols(&symbols);

        println!("[debug] Block Symbols:");
        rc_locals.borrow_mut().print();

        // execute statements
        for statement in &rc_block.statements {
            let (done, value) = Self::execute_statement(
                statement.clone(),
                rc_locals.clone(),
            );
            if done {
                return value;
            }
        }

        Value::Nil
    }

    fn execute_statement(
        rc_statement: Rc<StmtNode>,
        rc_locals: Rc<RefCell<Frame>>,
    ) -> (bool, Value)
    {
        match rc_statement.deref() {
            StmtNode::Let(_) => {
                println!("[debug] ignoring let statement");
                (false, Value::Nil)
            }
            StmtNode::Assign(assign) => {
                println!("[debug] executing assign statement");
                let name = &assign.name;
                let value = Evaluator::evaluate(assign.expr.clone(), rc_locals.clone());
                rc_locals.borrow_mut().assign(name, value);
                (false, Value::Nil)
            }
            StmtNode::Return(ret) => {
                println!("[debug] executing return statement");
                let value = Evaluator::evaluate(ret.expr.clone(), rc_locals.clone());
                (true, value)
            }
            StmtNode::Print(print) => {
                println!("[debug] executing print statement");
                let value = Evaluator::evaluate(print.expr.clone(), rc_locals.clone());
                value.print();
                (false, Value::Nil)
            }
            StmtNode::While(while_node) => {
                println!("[debug] executing while statement");
                // TODO: i tested this a little bit, should work, might wanna double check
                while Evaluator::evaluate(while_node.condition.clone(), rc_locals.clone()) == Value::Bool(true) {
                    Self::execute_block(while_node.body.clone(), rc_locals.clone());
                }
                (false, Value::Nil)
            }
            StmtNode::IfElse(if_else_node) => {
                println!("[debug] executing if else statement");
                // TODO: pretty much copy-pasted from While, so it should work, but needs more testing
                let condition = Evaluator::evaluate(if_else_node.condition.clone(), rc_locals.clone()) == Value::Bool(true);
                if condition {
                    println!("[debug] executing if branch");
                    Self::execute_block(if_else_node.ifBody.clone(), rc_locals.clone());
                }
                if !condition && if_else_node.elseBody.is_some() {
                    println!("[debug] executing else branch");
                    Self::execute_block(if_else_node.elseBody.clone().unwrap(), rc_locals.clone());
                }

                (false, Value::Nil)
            }
        }
    }
}
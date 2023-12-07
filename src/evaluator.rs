use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use crate::executor::Executor;
use crate::frame::Frame;
use crate::tree::ExprNode;
use crate::value::Value;

pub struct Evaluator {

}

impl Evaluator {

    pub fn evaluate(expr: Rc<ExprNode>, rc_frame: Rc<RefCell<Frame>>) -> Value {
        match expr.deref() {
            ExprNode::Var(name) => {
                rc_frame.borrow().lookup(name)
            }
            ExprNode::Val(value) => {
                value.clone()
            }
            ExprNode::String(value) => {
                Value::Chars(value.clone())
            }
            ExprNode::Add(expr_a, expr_b) => {
                let value_a = Self::evaluate(expr_a.clone(), rc_frame.clone());
                let value_b = Self::evaluate(expr_b.clone(), rc_frame.clone());
                Self::add(value_a, value_b)
            }
            ExprNode::Call(name, rc_exprs) => {
                println!("[debug] evaluating call '{name}'");
                match rc_frame.borrow().lookup_global(name) {
                    Value::Func(rc_func, argc) => {
                        assert_eq!(argc,rc_exprs.len());
                        let mut arguments = vec![];
                        for rc_expr in rc_exprs {
                            let arg = Self::evaluate(rc_expr.clone(), rc_frame.clone());
                            arguments.push(arg);
                        }
                        Executor::execute_function(rc_func, rc_frame.clone(), arguments)
                    }
                    _ => {
                        println!("[warn] function '{name}' not found");
                        Value::Nil
                    }
                }
            }
            ExprNode::LessThan(expr_a, expr_b) => {
                let value_a = Self::evaluate(expr_a.clone(), rc_frame.clone());
                let value_b = Self::evaluate(expr_b.clone(), rc_frame.clone());
                Value::Bool(value_a < value_b)
            }
            // TODO: implement >, <=, >=, ==
        }
    }

    fn add(value_a: Value, value_b: Value) -> Value {
        match value_a {
            Value::Nil => { Value::Nil }
            Value::Bool(a) => {
                match value_b {
                    Value::Nil => { Value::Nil }
                    Value::Bool(b) => { Value::I32(if a {1} else {0} + if b {1} else {0}) }
                    Value::I32(b) => { Value::I32(if a {1} else {0} + b) }
                    Value::Chars(b) => { Value::Chars(format!("{}{}", a, b)) }
                    _ => { Value::Nil }
                }
            }
            Value::I32(a) => {
                match value_b {
                    Value::Nil => { Value::Nil }
                    Value::Bool(b) => { Value::I32(a + if b {1} else {0}) }
                    Value::I32(b) => { Value::I32(a + b) }
                    Value::Chars(b) => { Value::Chars(format!("{}{}", a, b)) }
                    _ => { Value::Nil }
                }
            }
            Value::F32(_) => { todo!() }
            Value::Chars(a) => {
                match value_b {
                    Value::Nil => { Value::Nil }
                    Value::Bool(b) => { Value::Chars(format!("{}{}", a, if b {"true"} else {"false"})) }
                    Value::I32(b) => { Value::Chars(format!("{}{}", a, b)) }
                    Value::Chars(b) => { Value::Chars(format!("{}{}", a, b)) }
                    _ => { Value::Nil }
                }
            }
            Value::Func(_, _) => { todo!() }
        }
    }

}
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use crate::executor::Executor;
use crate::frame::Frame;
use crate::logger::Logger;
use crate::tree::ExprNode;
use crate::value::Value;

#[derive(Debug, Clone)]
enum ArithmeticOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
enum RelationalOp {
    Equal,
    LessThan,
    GreaterThan,
    NotEqual,
    LessThanEqual,
    GreaterThanEqual,
}

pub struct Evaluator {}

impl Evaluator {
    pub fn evaluate(expr: Rc<ExprNode>, rc_frame: Rc<RefCell<Frame>>) -> Value {
        match expr.deref() {
            ExprNode::Var(name) => {
                match rc_frame.borrow().lookup(name){
                    Value::Nil => {panic!("Can't find variable '{name}'!");}
                    other=>{other}
                }
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
                Self::arithmetic(value_a, value_b, ArithmeticOp::Add)
            }
            ExprNode::Mul(expr_a, expr_b) => {
                let value_a = Self::evaluate(expr_a.clone(), rc_frame.clone());
                let value_b = Self::evaluate(expr_b.clone(), rc_frame.clone());
                Self::arithmetic(value_a, value_b, ArithmeticOp::Mul)
            }
            ExprNode::Sub(expr_a, expr_b) => {
                let value_a = Self::evaluate(expr_a.clone(), rc_frame.clone());
                let value_b = Self::evaluate(expr_b.clone(), rc_frame.clone());
                Self::arithmetic(value_a, value_b, ArithmeticOp::Sub)
            }
            // todo ExprNode::Div(expr_a, expr_b) => {
            //     let value_a = Self::evaluate(expr_a.clone(), rc_frame.clone());
            //     let value_b = Self::evaluate(expr_b.clone(), rc_frame.clone());
            //     Self::arithmetic(value_a, value_b, ArithmeticOp::Div)
            // }
            ExprNode::Call(name, rc_exprs) => {
                Logger::debug(&format!("evaluating call '{name}'", name = name));
                match rc_frame.borrow().lookup(name) {
                    Value::Func(rc_func, argc) => {
                        assert_eq!(argc, rc_exprs.len());

                        let mut arguments = vec![];
                        for rc_expr in rc_exprs {
                            let arg = Self::evaluate(rc_expr.clone(), rc_frame.clone());
                            arguments.push(arg);
                        }

                        if let Some(globals) = rc_frame.borrow().get_globals() {
                            Executor::execute_function(rc_func, globals, arguments)
                        } else {
                            panic!("Can't find globals in current frame!");
                        }
                    }
                    _ => {
                        panic!("Can't find function '{name}' in globals!");
                    }
                }
            }
            ExprNode::LessThan(expr_a, expr_b) => {
                let value_a = Self::evaluate(expr_a.clone(), rc_frame.clone());
                let value_b = Self::evaluate(expr_b.clone(), rc_frame.clone());
                Self::relational(value_a, value_b, RelationalOp::LessThan)
            }
            ExprNode::GreaterThan(expr_a, expr_b) => {
                let value_a = Self::evaluate(expr_a.clone(), rc_frame.clone());
                let value_b = Self::evaluate(expr_b.clone(), rc_frame.clone());
                Self::relational(value_a, value_b, RelationalOp::GreaterThan)
            }
            ExprNode::EqualTo(expr_a, expr_b) => {
                let value_a = Self::evaluate(expr_a.clone(), rc_frame.clone());
                let value_b = Self::evaluate(expr_b.clone(), rc_frame.clone());
                Value::Bool(value_a == value_b)
            }
            ExprNode::LessThanEq(expr_a, expr_b) => {
                let value_a = Self::evaluate(expr_a.clone(), rc_frame.clone());
                let value_b = Self::evaluate(expr_b.clone(), rc_frame.clone());
                Self::relational(value_a, value_b, RelationalOp::LessThanEqual)
            }
            ExprNode::GreaterThanEq(expr_a, expr_b) => {
                let value_a = Self::evaluate(expr_a.clone(), rc_frame.clone());
                let value_b = Self::evaluate(expr_b.clone(), rc_frame.clone());
                Self::relational(value_a, value_b, RelationalOp::GreaterThanEqual)
            }
            ExprNode::NotEqualTo(expr_a, expr_b) => {
                let value_a = Self::evaluate(expr_a.clone(), rc_frame.clone());
                let value_b = Self::evaluate(expr_b.clone(), rc_frame.clone());
                Self::relational(value_a, value_b, RelationalOp::NotEqual)
            }
        }
    }

    fn arithmetic(value_a: Value, value_b: Value, op: ArithmeticOp) -> Value {
        match value_a {
            Value::Nil => { panic!("Left operand of '{op:?}' is Nil!"); }
            Value::Bool(_a) => { panic!("Left operand of '{op:?}' is Bool!"); }
            Value::I32(a) => {
                match value_b {
                    Value::Nil => { panic!("Right operand of '{op:?}' is Nil!"); }
                    Value::Bool(_b) => { panic!("Right operand of '{op:?}' is Bool!"); }
                    Value::I32(b) => {
                        match op {
                            ArithmeticOp::Add => { Value::I32(a + b) }
                            ArithmeticOp::Sub => { Value::I32(a - b) }
                            ArithmeticOp::Mul => { Value::I32(a * b) }
                            ArithmeticOp::Div => { Value::I32(a / b) }
                        }
                    }
                    Value::F32(_) => { todo!() }
                    Value::Chars(_) => { todo!() }
                    Value::Func(_, _) => { panic!("Right operand of '{op:?}' is Func!"); }
                }
            }
            Value::F32(a) => {
                match value_b {
                    Value::Nil => { panic!("Right operand of '{op:?}' is Nil!"); }
                    Value::Bool(_b) => { panic!("Right operand of '{op:?}' is Bool!"); }
                    Value::I32(b) => {
                        match op {
                            ArithmeticOp::Add => { Value::F32(a + (b as f32)) }
                            ArithmeticOp::Sub => { Value::F32(a - (b as f32)) }
                            ArithmeticOp::Mul => { Value::F32(a * (b as f32)) }
                            ArithmeticOp::Div => { Value::F32(a / (b as f32)) }
                        }
                    }
                    Value::F32(b) => {
                        match op {
                            ArithmeticOp::Add => { Value::F32(a + b) }
                            ArithmeticOp::Sub => { Value::F32(a - b) }
                            ArithmeticOp::Mul => { Value::F32(a * b) }
                            ArithmeticOp::Div => { Value::F32(a / b) }
                        }
                    }
                    Value::Chars(b) => {
                        match op {
                            ArithmeticOp::Add => { Value::Chars(a.to_string() + &b) }
                            _ => { panic!("Can't perform '{op:?}' on Chars!"); }
                        }
                    }
                    Value::Func(_, _) => { panic!("Right operand of '{op:?}' is Func!"); }
                }
            }
            Value::Chars(a) => {
                match value_b {
                    Value::Nil => { panic!("Right operand of '{op:?}' is Nil!"); }
                    Value::Bool(b) => {
                        match op {
                            ArithmeticOp::Add => { Value::Chars(format!("{}{}", a, b)) }
                            _ => { panic!("Can't perform '{op:?}' on Chars!"); }
                        }
                    }
                    Value::I32(b) => {
                        match op {
                            ArithmeticOp::Add => { Value::Chars(a + &b.to_string()) }
                            _ => { panic!("Can't perform '{op:?}' on Chars!"); }
                        }
                    }
                    Value::F32(_) => { todo!() }
                    Value::Chars(b) => {
                        match op {
                            ArithmeticOp::Add => { Value::Chars(a + &b) }
                            _ => { panic!("Can't perform '{op:?}' on Chars!"); }
                        }
                    }
                    Value::Func(_, _) => { panic!("Right operand of '{op:?}' is Func!"); }
                }
            }
            Value::Func(_, _) => { panic!("Left operand of '{op:?}' is Func!"); }
        }
    }

    fn relational(value_a: Value, value_b: Value, op: RelationalOp) -> Value {
        match value_a {
            Value::Nil => { panic!("Left operand of '{op:?}' is Nil!"); }
            Value::Bool(_a) => { panic!("Left operand of '{op:?}' is Bool!"); }
            Value::I32(a) => {
                match value_b {
                    Value::Nil => { panic!("Right operand of '{op:?}' is Nil!"); }
                    Value::Bool(_b) => { panic!("Right operand of '{op:?}' is Bool!"); }
                    Value::I32(b) => {
                        match op {
                            RelationalOp::Equal => { Value::Bool(a == b) }
                            RelationalOp::LessThan => { Value::Bool(a < b) }
                            RelationalOp::GreaterThan => { Value::Bool(a > b) }
                            RelationalOp::NotEqual => { Value::Bool(a != b) }
                            RelationalOp::LessThanEqual => { Value::Bool(a <= b) }
                            RelationalOp::GreaterThanEqual => { Value::Bool(a >= b) }
                        }
                    }
                    Value::F32(_) => { todo!() }
                    Value::Chars(_) => { todo!() }
                    Value::Func(_, _) => { panic!("Right operand of '{op:?}' is Func!"); }
                }
            }
            Value::F32(a) => {
                match value_b {
                    Value::Nil => { panic!("Right operand of '{op:?}' is Nil!"); }
                    Value::Bool(_b) => { panic!("Right operand of '{op:?}' is Bool!"); }
                    Value::I32(b) => {
                        match op {
                            RelationalOp::Equal => { Value::Bool(a == (b as f32)) }
                            RelationalOp::LessThan => { Value::Bool(a < (b as f32)) }
                            RelationalOp::GreaterThan => { Value::Bool(a > (b as f32)) }
                            RelationalOp::NotEqual => { Value::Bool(a != (b as f32)) }
                            RelationalOp::LessThanEqual => { Value::Bool(a <= (b as f32)) }
                            RelationalOp::GreaterThanEqual => { Value::Bool(a >= (b as f32)) }
                        }
                    }
                    Value::F32(b) => {
                        match op {
                            RelationalOp::Equal => { Value::Bool(a == b) }
                            RelationalOp::LessThan => { Value::Bool(a < b) }
                            RelationalOp::GreaterThan => { Value::Bool(a > b) }
                            RelationalOp::NotEqual => { Value::Bool(a != b) }
                            RelationalOp::LessThanEqual => { Value::Bool(a <= b) }
                            RelationalOp::GreaterThanEqual => { Value::Bool(a >= b) }
                        }
                    }
                    Value::Chars(_) => { todo!() }
                    Value::Func(_, _) => { panic!("Right operand of '{op:?}' is Func!"); }
                }
            }
            Value::Chars(_) => { todo!() }
            Value::Func(_, _) => { panic!("Left operand of '{op:?}' is Func!"); }
        }
    }
}
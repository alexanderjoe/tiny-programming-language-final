use std::cell::RefCell;
use std::rc::Rc;
use crate::symbols::Symbols;
use crate::value::Value;

#[derive(Debug, Clone)]
pub struct ProgramNode {
    pub symbols: Rc<RefCell<Symbols>>,
    pub let_nodes: Vec<Rc<LetNode>>,
    pub func_nodes: Vec<Rc<FuncNode>>,
}

impl ProgramNode {
    pub fn new() -> ProgramNode {
        ProgramNode {
            symbols: Rc::new(RefCell::new(Symbols::new(None))),
            let_nodes: vec![],
            func_nodes: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub struct FuncNode {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub block_node: Rc<BlockNode>,
}

impl FuncNode {
    pub fn new(name: String, parameters: Vec<Parameter>, block_node: BlockNode) -> FuncNode {
        FuncNode {
            name,
            parameters,
            block_node: Rc::new(block_node),
        }
    }

    pub fn numParameters(&self) -> usize {
        self.parameters.len()
    }
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
}

impl Parameter {
    pub fn new(name: String) -> Parameter {
        Parameter {
            name
        }
    }
}

#[derive(Debug, Clone)]
pub struct BlockNode {
    pub symbols: Rc<RefCell<Symbols>>,
    pub statements: Vec<Rc<StmtNode>>,
}

impl BlockNode {
    pub fn new() -> BlockNode {
        BlockNode {
            symbols: Rc::new(RefCell::new(Symbols::new(None))),
            statements: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub enum StmtNode {
    Let(LetNode),
    Assign(AssignNode),
    Return(ReturnNode),
    Print(PrintNode),
    While(WhileNode),
    IfElse(IfElseNode),
}


#[derive(Debug, Clone)]
pub struct LetNode {
    pub name: String,
    pub value: Value,
}

impl LetNode {
    pub fn new(name: String, value: Value) -> LetNode {
        LetNode {
            name,
            value,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AssignNode {
    pub name: String,
    pub expr: Rc<ExprNode>,
}

impl AssignNode {
    pub fn new(name: String, expr: ExprNode) -> AssignNode {
        AssignNode {
            name,
            expr: Rc::new(expr),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReturnNode {
    pub expr: Rc<ExprNode>,
}

impl ReturnNode {
    pub fn new(expr: ExprNode) -> ReturnNode {
        ReturnNode {
            expr: Rc::new(expr),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PrintNode {
    pub expr: Rc<ExprNode>,
}

impl PrintNode {
    pub fn new(expr: ExprNode) -> PrintNode {
        PrintNode {
            expr: Rc::new(expr),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WhileNode {
    pub condition: Rc<ExprNode>,
    pub body: Rc<BlockNode>,
}

impl WhileNode {
    pub fn new(condition: ExprNode, body: BlockNode) -> WhileNode {
        WhileNode {
            condition: Rc::new(condition),
            body: Rc::new(body),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IfElseNode {
    pub condition: Rc<ExprNode>,
    pub ifBody: Rc<BlockNode>,
    pub elseBody: Option<Rc<BlockNode>>,
}

impl IfElseNode {
    pub fn new(condition: ExprNode, ifBody: BlockNode, elseBody: Option<BlockNode>) -> IfElseNode {
        IfElseNode {
            condition: Rc::new(condition),
            ifBody: Rc::new(ifBody),
            elseBody: match elseBody {
                Some(block) => Some(Rc::new(block)),
                None => None,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExprNode {
    Var(String),
    Val(Value),
    String(String),
    Add(Rc<ExprNode>, Rc<ExprNode>),
    Call(String, Vec<Rc<ExprNode>>),
    LessThan(Rc<ExprNode>, Rc<ExprNode>),
    GreaterThan(Rc<ExprNode>, Rc<ExprNode>),
    EqualTo(Rc<ExprNode>, Rc<ExprNode>),
    LessThanEq(Rc<ExprNode>, Rc<ExprNode>),
    GreaterThanEq(Rc<ExprNode>, Rc<ExprNode>),
    NotEqualTo(Rc<ExprNode>, Rc<ExprNode>),
}

impl ExprNode {}
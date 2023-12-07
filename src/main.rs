#![allow(non_snake_case)]

use std::{error::Error, fs::read_to_string, path::PathBuf, rc::Rc};

use clap::{ArgGroup, Parser};

use crate::machine::Machine;
use crate::tree::{AssignNode, BlockNode, ExprNode, FuncNode, LetNode, Parameter, PrintNode, ProgramNode, ReturnNode, StmtNode};
use crate::value::Value;

mod tree;
mod executor;
mod machine;
mod analyzer;
mod symbols;
mod frame;
mod value;
mod evaluator;

/*

The test AST corresponds to following code:

let count;
let help;

func add(a,b) [
    return a + b;
]

func main(argc) [
    let sum;
    sum = 3+(5+7);
    print sum;
    sum = add(sum, 1);
    print sum;
]


 */

// TODO: this will be replaced with our own AST generated. For testing.
fn grow_ast_program0() -> Rc<ProgramNode> {
    let mut program = ProgramNode::new();

    // global variables
    let let_count = LetNode::new("count".to_string(), Value::Nil);
    let let_help = LetNode::new("help".to_string(), Value::Nil);
    program.let_nodes.push(Rc::new(let_count));
    program.let_nodes.push(Rc::new(let_help));

    // add function
    let mut parameters_add = vec![];
    parameters_add.push(Parameter::new("a".to_string()));
    parameters_add.push(Parameter::new("b".to_string()));

    let mut block_add = BlockNode::new();
    let stmtAdd1 = StmtNode::Return(
        ReturnNode::new(ExprNode::Add(
            Rc::new(ExprNode::Var("a".to_string())),
            Rc::new(ExprNode::Var("b".to_string())),
        ))
    );
    block_add.statements.push(Rc::new(stmtAdd1));

    let func_add = FuncNode::new(
        "add".to_string(),
        parameters_add,
        block_add);

    program.func_nodes.push(Rc::new(func_add));

    // main function
    let mut parameters_main = vec![];
    parameters_main.push(Parameter::new("argc".to_string()));

    let mut block_main = BlockNode::new();
    let stmtMain1 = StmtNode::Let(LetNode::new("sum".to_string(), Value::Nil));
    let stmtMain2 = StmtNode::Assign(
        AssignNode::new("sum".to_string(), ExprNode::Add(
            Rc::new(ExprNode::Val(Value::I32(3))),
            Rc::new(ExprNode::Add(
                Rc::new(ExprNode::Val(Value::I32(5))),
                Rc::new(ExprNode::Val(Value::I32(7))),
            )),
        ))
    );
    let stmtMain3 = StmtNode::Print(
        PrintNode::new(ExprNode::Var("sum".to_string())));
    let stmtMain4 = StmtNode::Assign(
        AssignNode::new(
            "sum".to_string(),
            ExprNode::Call(
                "add".to_string(), vec![
                    Rc::new(ExprNode::Var("sum".to_string())),
                    Rc::new(ExprNode::Val(Value::I32(1))),
                ]),
        ));
    let stmtMain5 = StmtNode::Print(
        PrintNode::new(ExprNode::Var("sum".to_string())));
    block_main.statements.push(Rc::new(stmtMain1));
    block_main.statements.push(Rc::new(stmtMain2));
    block_main.statements.push(Rc::new(stmtMain3));
    block_main.statements.push(Rc::new(stmtMain4));
    block_main.statements.push(Rc::new(stmtMain5));

    let func_main = FuncNode::new(
        "main".to_string(),
        parameters_main,
        block_main);

    program.func_nodes.push(Rc::new(func_main));


    Rc::new(program)
}


fn run0() {
    // this is the ast from the parser
    // TODO: replace this with our parser
    let rc_program = grow_ast_program0();

    // this is the machine that executes the ast
    // it has the analyzer and the program executor
    // TODO: should be pretty set, but will need to add some further logic later
    let runtime = Machine::new(rc_program);
    runtime.run();
}


fn main() -> Result<(), Box<dyn Error>> {
    // TODO: uncomment when ready to add cli support
    // let args = Cli::parse();
    //
    // if args.tokenize {
    //     println!("Tokenizing file: {:?}", args.file);
    // }
    //
    // if args.execute {
    //     println!("Executing file: {:?}", args.file);
    // }
    //
    // println!("File contents:\n{}", read_to_string(args.file)?);

    run0();

    Ok(())
}

/// Tiny Programming Language Cli
#[derive(Debug, Parser)]
#[clap(group = ArgGroup::new("action").required(false).multiple(true))]
struct Cli {
    /// File to process
    file: PathBuf,

    /// Tokenize the file
    #[clap(short = 't', long = "tokenize", group = "action")]
    tokenize: bool,

    /// Execute the file
    #[clap(short = 'e', long = "execute", group = "action")]
    execute: bool,
}
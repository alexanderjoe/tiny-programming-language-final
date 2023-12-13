#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)] // TODO: remove this

use std::{error::Error, fs::read_to_string, path::PathBuf, rc::Rc};

use clap::{ArgGroup, Parser};
use clap::builder::PossibleValue;
use tree::IfElseNode;
use crate::logger::{Logger, LOGGER};

use crate::machine::Machine;
use crate::parser::DescentParser;
use crate::tree::{AssignNode, BlockNode, ExprNode, FuncNode, LetNode, Parameter, PrintNode, ProgramNode, ReturnNode, StmtNode, WhileNode};
use crate::value::Value;


mod tree;
mod executor;
mod machine;
mod analyzer;
mod symbols;
mod frame;
mod value;
mod evaluator;

// added
mod lexer;
mod parser;
mod token;
mod parser_pratt;
mod logger;

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
    print "While loop" + 50;
    while sum < 20 [
        sum = add(sum, 1);
        print sum;
    ]
    if sum == 20 [
        sum = add(sum, 1);
        print sum;
    ] else [
        sum = add(sum, 2);
        print sum;
    ]
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

    // block for while loop
    let mut whileBlock = BlockNode::new();
    let stmtWhile1 = StmtNode::Assign(
        AssignNode::new(
            "sum".to_string(),
            ExprNode::Call(
                "add".to_string(), vec![
                    Rc::new(ExprNode::Var("sum".to_string())),
                    Rc::new(ExprNode::Val(Value::I32(1))),
                ]),
        ));
    let stmtWhile2 = StmtNode::Print(
        PrintNode::new(ExprNode::Var("sum".to_string())));
    whileBlock.statements.push(Rc::new(stmtWhile1));
    whileBlock.statements.push(Rc::new(stmtWhile2));

    // while loop statement
    let stmtMain6 = StmtNode::While(WhileNode::new(
        ExprNode::LessThan(
            Rc::new(ExprNode::Var("sum".to_string())),
            Rc::new(ExprNode::Val(Value::I32(20))),
        ),
        whileBlock,
    ));

    // block for if
    let mut ifBlock = BlockNode::new();
    let stmtIf1 = StmtNode::Assign(
        AssignNode::new(
            "sum".to_string(),
            ExprNode::Call(
                "add".to_string(), vec![
                    Rc::new(ExprNode::Var("sum".to_string())),
                    Rc::new(ExprNode::Val(Value::I32(1))),
                ]),
        )
    );
    let stmtIf2 = StmtNode::Print(
        PrintNode::new(ExprNode::Var("sum".to_string())));
    ifBlock.statements.push(Rc::new(stmtIf1));
    ifBlock.statements.push(Rc::new(stmtIf2));

    // block for else
    let mut elseBlock = BlockNode::new();
    let stmtElse1 = StmtNode::Assign(
        AssignNode::new(
            "sum".to_string(),
            ExprNode::Call(
                "add".to_string(), vec![
                    Rc::new(ExprNode::Var("sum".to_string())),
                    Rc::new(ExprNode::Val(Value::I32(2))),
                ]),
        )
    );
    let stmtElse2 = StmtNode::Print(
        PrintNode::new(ExprNode::Var("sum".to_string())));
    elseBlock.statements.push(Rc::new(stmtElse1));
    elseBlock.statements.push(Rc::new(stmtElse2));

    // if else statement
    let stmtMain7 = StmtNode::IfElse(IfElseNode::new(
        ExprNode::EqualTo(
            Rc::new(ExprNode::Var("sum".to_string())),
            Rc::new(ExprNode::Val(Value::I32(21))),
        ),
        ifBlock,
        None,
        //elseBlock.into()
    ));

    // add statements to main block
    block_main.statements.push(Rc::new(stmtMain1));
    block_main.statements.push(Rc::new(stmtMain2));
    block_main.statements.push(Rc::new(stmtMain3));
    block_main.statements.push(Rc::new(stmtMain4));
    block_main.statements.push(Rc::new(stmtMain5));
    // debug print statement
    block_main.statements.push(Rc::new(StmtNode::Print(PrintNode::new(ExprNode::Add(Rc::new(ExprNode::String("While loop".to_string())), Rc::new(ExprNode::Val(Value::I32(50))))))));
    block_main.statements.push(Rc::new(stmtMain6));
    block_main.statements.push(Rc::new(stmtMain7));

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

fn run_main(input: String) {
    let mut lexer = lexer::Lexer::new("".to_string());
    lexer.set_input(input);

    let mut parser = DescentParser::new(lexer);
    let ast = parser.analyze();

    // print ast
    Logger::debug(&format!("\n---------------------\nProgram AST:\n {ast:#?}\n---------------------", ast=ast));

    let runtime = Machine::new(Rc::new(ast));
    runtime.run();
}


fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    // TODO: uncomment when ready to add cli support
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

    let log_level = match args.loglevel.as_str() {
        "info" => logger::Level::Info,
        "debug" => logger::Level::Debug,
        "warn" => logger::Level::Warn,
        "none" => logger::Level::None,
        _ => panic!("Invalid log level: {}", args.loglevel)
    };

    *LOGGER.lock().unwrap() = Logger {
        level: log_level,
    };

    Logger::info("Starting Iron Oxide...");

    // run0();

    let input = read_to_string(args.file).expect("Failed to read input file.");
    run_main(input);

    Ok(())
}

/// Iron Oxide Cli
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None, group = ArgGroup::new("action").required(false))]
struct Cli {
    /// File to process
    file: PathBuf,

    /// Logging level
    #[arg(short, long, default_value = "info", value_parser = vec![PossibleValue::new("info"), PossibleValue::new("debug"), PossibleValue::new("warn"), PossibleValue::new("none")], group = "action")]
    loglevel: String,

    // /// Tokenize the file
    // #[clap(short = 't', long = "tokenize", group = "action")]
    // tokenize: bool,
    //
    // /// Execute the file
    // #[clap(short = 'e', long = "execute", group = "action")]
    // execute: bool,
}
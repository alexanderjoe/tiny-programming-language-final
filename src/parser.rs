#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::rc::Rc;

use crate::lexer::Lexer;
use crate::logger::Logger;
use crate::token::Token;
use crate::tree::*;
use crate::value::Value;

const INDENT: usize = 2;

pub struct DescentParser {
    lexer: Lexer,
    indent: usize,
}

impl DescentParser {
    pub fn new(lexer: Lexer) -> DescentParser {
        DescentParser {
            lexer,
            indent: 0,
        }
    }

    pub fn analyze(&mut self) -> ProgramNode {
        self.indent = 0;
        self.advance(); // prime the lexer

        let mut program = ProgramNode::new();

        // parse lexer output until we reach the EOI token
        while !self.peek(Token::EOI) {
            match self.curr() {
                Token::KW_FUNC => {
                    let func_node = self.parse_func();
                    program.func_nodes.push(Rc::new(func_node));
                }
                Token::KW_LET => {
                    let let_node = self.parse_let();
                    program.let_nodes.push(Rc::new(let_node));
                }
                _ => {
                    panic!("Unexpected token `{}` while parsing program.", self.curr())
                }
            }
        }

        self.expect(Token::EOI);

        program
    }

    /*
    * EBNF
    * func = 'func' <identifier> '(' <parameter_list> ')' '->' <type> <block_nest>
    * identifier = ID(String)
    * type = TYPE_INT32 | TYPE_FLT32 | TYPE_CHAR
    */
    fn parse_func(&mut self) -> FuncNode {
        self.indent_print("parse_func()");
        self.indent_increment();

        self.expect(Token::KW_FUNC);

        let func_name = self.expect(Token::id());

        let params_node = self.parse_parameter_list();

        // optional return type todo: implement
        // if self.accept(Token::ARROW_R) {
        //     let return_node = self.help_parse_type();
        //     func_node.push(return_node);
        // }

        let block_node = self.parse_block_nest();

        self.indent_decrement();

        FuncNode::new(func_name.get_id_name(), params_node, block_node)
    }

    /*
    * EBNF
    * parameter_list = '(' <parameter> {',' <parameter>} ')'
    * parameter = <identifier> ':' <type> | <identifier>
    * identifier = ID(String)
    * type = TYPE_INT32 | TYPE_FLT32 | TYPE_CHAR | TYPE_BOOL
    */
    fn parse_parameter_list(&mut self) -> Vec<Parameter> {
        self.indent_print("parse_parameter_list()");
        self.indent_increment();
        let mut params = vec![];

        self.expect(Token::PARENS_L);
        if self.accept(Token::PARENS_R) {
            return params;
        }

        loop {
            let parameter = self.parse_parameter();
            params.push(parameter);

            if !self.accept(Token::COMMA) {
                break;
            }
        }

        self.expect(Token::PARENS_R);

        self.indent_decrement();
        params
    }

    /*
    * EBNF
    * parameter = <identifier> ':' <type>
    * identifier = ID(String)
    * type = TYPE_INT32 | TYPE_FLT32 | TYPE_CHAR
    */
    fn parse_parameter(&mut self) -> Parameter {
        self.indent_print("parse_parameter()");
        self.indent_increment();

        let param_name = self.expect(Token::id());

        // todo: add parameter types
        // self.expect(Token::COLON);
        //
        // // match the type
        // let type_node = self.help_parse_type();
        // param_node.push(type_node);

        self.indent_decrement();
        Parameter::new(param_name.get_id_name())
    }

    /*
    * EBNF
    * type = TYPE_INT32 | TYPE_FLT32 | TYPE_CHAR | TYPE_BOOL
    */
    // todo: reimpl
    // fn help_parse_type(&mut self) -> ParseTree {
    //     let type_node = match self.curr() {
    //         Token::TYPE_INT32 => {
    //             self.expect(Token::TYPE_INT32);
    //             ParseTree::new(Token::TYPE_INT32)
    //         }
    //         Token::TYPE_FLT32 => {
    //             self.expect(Token::TYPE_FLT32);
    //             ParseTree::new(Token::TYPE_FLT32)
    //         }
    //         Token::TYPE_CHAR => {
    //             self.expect(Token::TYPE_CHAR);
    //             ParseTree::new(Token::TYPE_CHAR)
    //         }
    //         Token::TYPE_BOOL => {
    //             self.expect(Token::TYPE_BOOL);
    //             ParseTree::new(Token::TYPE_BOOL)
    //         }
    //         _ => panic!("Expected type but found '{:?}'", self.curr()),
    //     };
    //
    //     type_node
    // }

    /*
    * EBNF
    * block_nest = '[' <block_list> ']'
    * block_list = <block_nest> | <let> <block_list> |
    */
    fn parse_block_nest(&mut self) -> BlockNode {
        self.indent_print("parse_block_nest()");
        self.indent_increment();
        let mut block_node = BlockNode::new();

        self.expect(Token::BRACKET_L);
        while !self.peek(Token::BRACKET_R) {
            match self.curr() {
                Token::BRACKET_L => {
                    let nested_block = self.parse_block_nest();
                    block_node.statements.push(Rc::new(StmtNode::Block(nested_block.into()))); //todo: there is no StmtNode::Block
                }
                Token::KW_LET => {
                    let let_node = self.parse_let();
                    block_node.statements.push(Rc::new(StmtNode::Let(let_node)));
                }
                Token::KW_IF => {
                    let if_node = self.parse_if_then_else();
                    block_node.statements.push(Rc::new(StmtNode::IfElse(if_node)))
                }
                Token::KW_RETURN => {
                    let return_node = self.parse_return();
                    block_node.statements.push(Rc::new(StmtNode::Return(return_node)))
                }
                Token::KW_WHILE => {
                    let while_node = self.parse_while();
                    block_node.statements.push(Rc::new(StmtNode::While(while_node)))
                }
                Token::KW_PRINT => {
                    let print_node = self.parse_print();
                    block_node.statements.push(Rc::new(StmtNode::Print(print_node)))
                }
                Token::ID(_) => {
                    let assign_node = self.parse_assign();
                    block_node.statements.push(Rc::new(StmtNode::Assign(assign_node)))
                }
                _ => panic!("Unexpected token in block: '{:?}'", self.curr()),
            }
        }
        self.expect(Token::BRACKET_R);

        self.indent_decrement();
        block_node
    }

    /*
    * EBNF
    * let = 'let' <identifier> ':' <type> '=' <value>';' | 'let' <identifier> ':' <type>';' | 'let' <identifier>';'
    * identifier = ID(String)
    * type = TYPE_INT32 | TYPE_FLT32 | TYPE_CHAR
    * expression = <term> <expression_tail>
    * term = <factor> <term_tail>
    * value = <identifier> | <literal>
    * literal = LIT_INT32(i32) | LIT_FLT32(f32) | LIT_CHAR(char) | LIT_STRING(String)
    */
    fn parse_let(&mut self) -> LetNode {
        self.indent_print("parse_let()");
        self.indent_increment();


        self.expect(Token::KW_LET);
        let let_name = self.expect(Token::id());

        // if self.accept(Token::COLON) {
        //     // match the type
        //     let let_type = self.help_parse_type();
        //     let_node.push(let_type);
        // }
        // todo: add value assignment
        // optional assignment
        let mut val_node = Value::Nil;
        if self.accept(Token::OP_ASSIGN) {
            let expr_node = self.parse_expr();
            val_node = match expr_node {
                ExprNode::Val(val) => val,
                _ => panic!("Expected value but found '{:?}'", self.curr()),
            };
        }

        self.expect(Token::SEMICOLON);

        self.indent_decrement();
        LetNode::new(let_name.get_id_name(), val_node)
    }

    /*
    * EBNF
    * if_then_else = 'if' <bool> '[' <block_nest> ']' 'else' '[' <block_nest> ']' | 'if' <bool> '[' <block_nest> ']'
    * bool = true | false
    * block_nest = '[' <block_list> ']'
    */
    fn parse_if_then_else(&mut self) -> IfElseNode {
        self.indent_print("parse_if_then_else()");
        self.indent_increment();

        self.expect(Token::KW_IF);

        let condition_expr = self.parse_expr();

        let then_node_block = self.parse_block_nest();

        // optional else block
        let mut else_node_block: Option<BlockNode> = None;
        if self.accept(Token::KW_ELSE) {
            else_node_block = self.parse_block_nest().into();
        }

        self.indent_decrement();
        IfElseNode::new(condition_expr, then_node_block, else_node_block)
    }

    /*
    * EBNF
    * return = 'return' <value> ';'
    * value = <identifier> | <literal>
    * identifier = ID(String)
    * literal = LIT_INT32(i32) | LIT_FLT32(f32) | LIT_CHAR(char) | LIT_STRING(String) | LIT_BOOL(bool)
    */
    fn parse_return(&mut self) -> ReturnNode {
        self.indent_print("parse_return()");
        self.indent_increment();

        self.expect(Token::KW_RETURN);
        let expr_node = self.parse_expr();
        self.expect(Token::SEMICOLON);

        self.indent_decrement();
        ReturnNode::new(expr_node)
    }

    fn parse_while(&mut self) -> WhileNode {
        self.indent_print("parse_while()");
        self.indent_increment();

        self.expect(Token::KW_WHILE);
        let expr_node = self.parse_expr();
        let block_node = self.parse_block_nest();

        self.indent_decrement();
        WhileNode::new(expr_node, block_node)
    }

    fn parse_print(&mut self) -> PrintNode {
        self.indent_print("parse_print()");
        self.indent_increment();

        self.expect(Token::KW_PRINT);
        let expr_node = self.parse_expr();
        self.expect(Token::SEMICOLON);

        self.indent_decrement();
        PrintNode::new(expr_node)
    }

    fn parse_assign(&mut self) -> AssignNode {
        self.indent_print("parse_assign()");
        self.indent_increment();

        let id_node = self.expect(Token::id());
        self.expect(Token::OP_ASSIGN);
        let expr_node = self.parse_expr();
        self.expect(Token::SEMICOLON);

        self.indent_decrement();
        AssignNode::new(id_node.get_id_name(), expr_node)
    }

    /*
    * EBNF
    * value = <identifier> | <literal>
    * identifier = ID(String)
    * literal = LIT_INT32(i32) | LIT_FLT32(f32) | LIT_CHAR(char) | LIT_STRING(String) | LIT_BOOL(bool)
    */
    //todo: this is very basic, just a temp as we have no pratt parser

    fn parse_expr(&mut self) -> ExprNode {
        self.indent_print("parse_expr()");
        self.indent_increment();
        let token = self.curr();

        let expr_node = match token {
            Token::ID(_) => {
                let id_node = self.expect(Token::id());
                if self.peek(Token::PARENS_L) {
                    self.parse_func_call(id_node.get_id_name())
                } else {
                    ExprNode::Var(id_node.get_id_name())
                }
            }
            Token::LIT_INT32(_) => {
                let lit_node = self.expect(Token::lit_i32());
                ExprNode::Val(Value::I32(lit_node.get_lit_i32()))
            }
            Token::LIT_FLT32(_) => {
                let lit_node = self.expect(Token::lit_f32());
                ExprNode::Val(Value::F32(lit_node.get_lit_f32()))
            }
            Token::LIT_CHAR(_) => {
                let lit_node = self.expect(Token::lit_char());
                ExprNode::Val(Value::Chars(lit_node.get_lit_char().to_string())) // todo: fix this
            }
            Token::LIT_STRING(_) => {
                let lit_node = self.expect(Token::lit_string());
                ExprNode::Val(Value::Chars(lit_node.get_lit_string()))
            }
            Token::LIT_BOOL(_) => {
                let lit_node = self.expect(Token::lit_bool());
                ExprNode::Val(Value::Bool(lit_node.get_lit_bool()))
            }
            _ => panic!("Expected value but found '{:?}'", self.curr()),
        };

        let is_end_of_expr = match self.curr() {
            Token::BRACKET_L => true,
            Token::SEMICOLON => true,
            Token::COMMA => true,
            Token::PARENS_R => true,
            _ => false,
        };

        if is_end_of_expr {
            self.indent_decrement();
            return expr_node;
        }

        let expr_tail_node = self.parse_expr_tail(expr_node);
        self.indent_decrement();
        expr_tail_node
    }

    fn parse_expr_tail(&mut self, left_denotation: ExprNode) -> ExprNode {
        self.indent_print("parse_expr_tail()");
        self.indent_increment();
        let token = self.curr();

        let expr_node = match token {
            Token::OP_ADD => {
                self.expect(Token::OP_ADD);
                let right_denotation = self.parse_expr();
                ExprNode::Add(Rc::new(left_denotation), Rc::new(right_denotation))
            }
            Token::OP_MUL => {
                self.expect(Token::OP_MUL);
                let right_denotation = self.parse_expr();
                ExprNode::Mul(Rc::new(left_denotation), Rc::new(right_denotation))
            }
            Token::OP_SUB => {
                self.expect(Token::OP_SUB);
                let right_denotation = self.parse_expr();
                ExprNode::Sub(Rc::new(left_denotation), Rc::new(right_denotation))
            }
            Token::OP_LT => {
                self.expect(Token::OP_LT);
                let right_denotation = self.parse_expr();
                ExprNode::LessThan(Rc::new(left_denotation), Rc::new(right_denotation))
            }
            Token::OP_GT => {
                self.expect(Token::OP_GT);
                let right_denotation = self.parse_expr();
                ExprNode::GreaterThan(Rc::new(left_denotation), Rc::new(right_denotation))
            }
            Token::OP_EQ => {
                self.expect(Token::OP_EQ);
                let right_denotation = self.parse_expr();
                ExprNode::EqualTo(Rc::new(left_denotation), Rc::new(right_denotation))
            }
            Token::OP_NGT => {
                self.expect(Token::OP_NGT);
                let right_denotation = self.parse_expr();
                ExprNode::LessThanEq(Rc::new(left_denotation), Rc::new(right_denotation))
            }
            Token::OP_NLT => {
                self.expect(Token::OP_NLT);
                let right_denotation = self.parse_expr();
                ExprNode::GreaterThanEq(Rc::new(left_denotation), Rc::new(right_denotation))
            }
            Token::OP_NEQ => {
                self.expect(Token::OP_NEQ);
                let right_denotation = self.parse_expr();
                ExprNode::NotEqualTo(Rc::new(left_denotation), Rc::new(right_denotation))
            }
            _ => panic!("Expected value but found '{:?}'", self.curr()),
        };

        let is_end_of_expr = match self.curr() {
            Token::BRACKET_L => true,
            Token::SEMICOLON => true,
            Token::COMMA => true,
            Token::PARENS_R => true,
            _ => false,
        };

        if is_end_of_expr {
            self.indent_decrement();
            return expr_node;
        }

        let expr_tail_node = self.parse_expr_tail(expr_node);
        self.indent_decrement();
        expr_tail_node
    }

    fn parse_func_call(&mut self, func_name: String) -> ExprNode {
        self.expect(Token::PARENS_L);
        let mut args = vec![];
        while !self.peek(Token::PARENS_R) {
            let arg = self.parse_expr();
            args.push(Rc::new(arg));
            if !self.accept(Token::COMMA) {
                break;
            }
        }
        self.expect(Token::PARENS_R);
        ExprNode::Call(func_name, args)
    }
}

// utility functions for lexer
impl DescentParser {
    fn curr(&mut self) -> Token {
        self.lexer.current()
    }

    fn advance(&mut self) {
        self.lexer.advance();
    }

    fn expect(&mut self, expected: Token) -> Token {
        if self.curr() == expected {
            let curr = self.curr().clone();
            Logger::debug(&format!("{:<indent$}expect({:?})", "", curr, indent = self.indent));
            self.advance();
            curr
        } else {
            panic!("Expected '{:?}' but found '{:?}'", expected, self.curr());
        }
    }

    fn accept(&mut self, symbol: Token) -> bool {
        if self.curr() == symbol {
            Logger::debug(&format!("{:<indent$}accept({:?})", "", symbol, indent = self.indent));
            self.advance();
            true
        } else {
            false
        }
    }

    fn peek(&mut self, symbol: Token) -> bool {
        self.lexer.current() == symbol
    }
}

// utility functions for pretty print
impl DescentParser {
    fn indent_print(&mut self, msg: &'static str) {
        Logger::debug(&format!("{:<indent$}{}", "", msg, indent = self.indent));
    }

    fn indent_increment(&mut self) {
        self.indent += INDENT;
    }
    fn indent_decrement(&mut self) {
        self.indent -= INDENT;
    }
}

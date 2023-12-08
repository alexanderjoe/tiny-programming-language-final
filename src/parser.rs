// Author: Alexander Diaz
// Class: CS 1720
// Assignment: Recursive Descent Parser

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::rc::Rc;
use crate::lexer::Lexer;
use crate::tree::*;
use crate::token::Token;
use crate::value::Value;

const INDENT: usize = 2;

pub struct DescentParser {
    lexer: Lexer,
    indent: usize,
}

// simple recursive descend parser
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

        // todo: parse content outside of functions (peek and see if it's a func keyword)
        // repeatedly parse functions until EOI
        while !self.peek(Token::EOI) {
            let func_node = self.parse_func();
            program.func_nodes.push(Rc::new(func_node));
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
                // Token::BRACKET_L => {
                //     let nested_block = self.parse_block_nest();
                //     block_node.statements.push(Rc::new(StmtNode::Block(nested_block))); todo: there is no StmtNode::Block
                // }
                Token::KW_LET => {
                    let let_node = self.parse_let();
                    block_node.statements.push(Rc::new(StmtNode::Let(let_node)));
                }
                // Token::KW_IF => {
                //     let if_node = self.parse_if_then_else();
                //     block_node.statements.push(Rc::new(StmtNode::IfElse(if_node)))
                // }
                // Token::KW_RETURN => {
                //     let return_node = self.parse_return();
                //     block_node.statements.push(Rc::new(StmtNode::Return(return_node)))
                // }
                // Token::KW_WHILE => {
                //     let while_node = self.parse_while();
                //     block_node.statements.push(Rc::new(StmtNode::While(while_node)))
                // }
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
        // // optional assignment
        // if self.accept(Token::OP_ASSIGN) {
        //     let value_node = self.parse_value();
        //     let_node.push(value_node);
        // }

        self.expect(Token::SEMICOLON);

        self.indent_decrement();
        LetNode::new_no_value(let_name.get_id_name())
    }

    /*
    * EBNF
    * if_then_else = 'if' <bool> '[' <block_nest> ']' 'else' '[' <block_nest> ']' | 'if' <bool> '[' <block_nest> ']'
    * bool = true | false
    * block_nest = '[' <block_list> ']'
    */
    // fn parse_if_then_else(&mut self) -> IfElseNode {
    //     self.indent_print("parse_if_then_else()");
    //     self.indent_increment();
    //     let mut if_node = ParseTree::new(Token::KW_IF);
    //     {
    //         self.expect(Token::KW_IF);
    //
    //         let condition_node = self.parse_value();
    //         if_node.push(condition_node);
    //
    //         let then_node_block = self.parse_block_nest();
    //         let mut then_node = ParseTree::new(Token::THEN_NODE);
    //         then_node.push(then_node_block);
    //         if_node.push(then_node);
    //
    //         if self.accept(Token::KW_ELSE) {
    //             let else_node_block = self.parse_block_nest();
    //             let mut else_node = ParseTree::new(Token::ELSE_NODE);
    //             else_node.push(else_node_block);
    //             if_node.push(else_node);
    //         }
    //     }
    //     self.indent_decrement();
    //     if_node
    // }

    /*
    * EBNF
    * return = 'return' <value> ';'
    * value = <identifier> | <literal>
    * identifier = ID(String)
    * literal = LIT_INT32(i32) | LIT_FLT32(f32) | LIT_CHAR(char) | LIT_STRING(String) | LIT_BOOL(bool)
    */
    // fn parse_return(&mut self) -> ReturnNode {
    //     self.indent_print("parse_return()");
    //     self.indent_increment();
    //     let mut return_node = ParseTree::new(Token::KW_RETURN);
    //     {
    //         self.expect(Token::KW_RETURN);
    //         let value_node = self.parse_value();
    //         return_node.push(value_node);
    //         self.expect(Token::SEMICOLON);
    //     }
    //     self.indent_decrement();
    //     ReturnNode::new()
    // }
    //
    // fn parse_while(&mut self) -> WhileNode {
    //     self.indent_print("parse_while()");
    //     self.indent_increment();
    //
    //     self.expect(Token::KW_WHILE);
    //     self.parse_value();
    // }

    /*
    * EBNF
    * value = <identifier> | <literal>
    * identifier = ID(String)
    * literal = LIT_INT32(i32) | LIT_FLT32(f32) | LIT_CHAR(char) | LIT_STRING(String) | LIT_BOOL(bool)
    */
    // fn parse_expr(&mut self) -> ExprNode {
    //     self.indent_print("parse_expr()");
    //     self.indent_increment();
    //     let token = self.curr();
    //     match token {
    //         Token::id() => {
    //             let identifier = self.expect(Token::id());
    //             let mut expr_node = ParseTree::new(Token::id());
    //             expr_node.push(ParseTree::new(identifier));
    //             self.indent_decrement();
    //             ExprNode::new(expr_node)
    //         }
    //         Token::lit_int32() => {
    //             let literal = self.expect(Token::lit_int32());
    //             let mut expr_node = ParseTree::new(Token::lit_int32());
    //             expr_node.push(ParseTree::new(literal));
    //             self.indent_decrement();
    //             ExprNode::new(expr_node)
    //         }
    //         Token::lit_flt32() => {
    //             let literal = self.expect(Token::lit_flt32());
    //             let mut expr_node = ParseTree::new(Token::lit_flt32());
    //             expr_node.push(ParseTree::new(literal));
    //             self.indent_decrement();
    //             ExprNode::new(expr_node)
    //         }
    //         Token::lit_char() => {
    //             let literal = self.expect(Token::lit_char());
    //             let mut expr_node = ParseTree::new(Token::lit_char());
    //             expr_node.push(ParseTree::new(literal));
    //             self.indent_decrement();
    //             ExprNode::new(expr_node)
    //         }
    //         Token::lit_string() => {
    //             let literal = self.expect(Token::lit_string());
    //             let mut expr_node = ParseTree::new(Token::lit_string());
    //             expr_node.push(ParseTree::new(literal));
    //             self.indent_decrement();
    //             ExprNode::new(expr_node)
    //         }
    //         Token::lit_bool() => {
    //             let literal = self.expect(Token::lit_bool());
    //             let mut expr_node = ParseTree::new(Token::lit_bool());
    //             expr_node.push(ParseTree::new(literal));
    //             self.indent_decrement();
    //             ExprNode::new(expr_node)
    //         }
    //         _ => panic!("Expected value but found '{:?}'", self.curr()),
    //     }
    // }
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
            println!("{:<indent$}expect({expected:?})", "", indent = self.indent);
            self.advance();
            curr
        } else {
            panic!("Expected '{:?}' but found '{:?}'", expected, self.curr());
        }
    }

    fn accept(&mut self, symbol: Token) -> bool {
        if self.curr() == symbol {
            println!("{:<indent$}accept({symbol:?})", "", indent = self.indent);
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
        println!("{:<indent$}{:}", "", msg, indent = self.indent);
    }

    fn indent_increment(&mut self) {
        self.indent += INDENT;
    }
    fn indent_decrement(&mut self) {
        self.indent -= INDENT;
    }
}

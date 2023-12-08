#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::fmt;
use std::fmt::Display;
use std::mem::discriminant;

#[derive(Debug, Clone)]
pub enum Token {
    // nesting
    PARENS_L,
    PARENS_R,
    BRACKET_L,
    BRACKET_R,
    BRACE_L,
    BRACE_R,

    // separators
    POINT,
    COMMA,
    COLON,
    SEMICOLON,
    ARROW_R,

    // arithmetic ops
    OP_ADD,
    OP_SUB,
    OP_MUL,
    OP_DIV,

    // relational ops
    OP_EQ,
    // equal
    OP_LT,
    // less than
    OP_GT,
    // greater than
    OP_NEQ,
    // not equal
    OP_NLT,
    // not less than (greater than or equal)
    OP_NGT, // not greater than (less than or equal)

    // logical ops
    OP_NOT,
    OP_AND,
    OP_OR,

    // other ops
    OP_ASSIGN,

    // keywords
    KW_FUNC,
    KW_LET,
    KW_IF,
    KW_ELSE,
    KW_WHILE,
    KW_RETURN,
    KW_PRINT,

    // types
    TYPE_INT32,
    TYPE_FLT32,
    TYPE_CHAR,
    TYPE_BOOL,

    // atoms
    ID(String),
    LIT_INT32(i32),
    LIT_FLT32(f32),
    LIT_CHAR(char),
    LIT_STRING(String),
    LIT_BOOL(bool),

    // general
    UNDEFINED,
    ERROR,
    EOI,

    // parse tree todo: remove in favor of tree.rs
    PROGRAM_NODE,
    FUNC_NODE,
    PARAMS_LIST_NODE,
    PARAM_NODE,
    BLOCK_NODE,
    THEN_NODE,
    ELSE_NODE,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl Eq for Token {}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Token {
    pub fn id() -> Token {
        Token::ID(String::new())
    }
    pub fn lit_i32() -> Token { Token::LIT_INT32(0) }
    pub fn lit_f32() -> Token { Token::LIT_FLT32(0.0) }
    pub fn lit_char() -> Token { Token::LIT_CHAR(' ') }
    pub fn lit_string() -> Token { Token::LIT_STRING(String::new()) }
    pub fn lit_bool() -> Token { Token::LIT_BOOL(false) }

    pub fn get_id_name(&self) -> String {
        match self {
            Token::ID(name) => name.clone(),
            _ => panic!("Expected ID token, found {:?}", self)
        }
    }
}
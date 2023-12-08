// Author: Alexander Diaz
// Class: CS 1720
// Assignment: Recursive Descent Parser

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::token::Token;

#[derive(Debug)]
pub enum LexerState {
    Initial,
    Separator,
    Operator,
    Assignment,
    Identifier,
    Number,
    String,
    Char,
}

pub struct Lexer {
    pub input_string: String,
    pub input_position: usize,
    pub current_state: LexerState,
    pub current_token: Token,
    pub buffer_string: String,
    pub token_list: Vec<Token>,
}

impl Lexer {
    pub fn set_input(&mut self, new_input: String) {
        self.input_string = new_input;
        self.input_position = 0;
        self.current_state = LexerState::Initial;
        self.current_token = Token::UNDEFINED;
        self.buffer_string.clear();
        self.token_list.clear();
    }

    pub fn new(input: String) -> Lexer {
        Lexer {
            input_string: input,
            input_position: 0,
            current_state: LexerState::Initial,
            current_token: Token::UNDEFINED,
            buffer_string: String::new(),
            token_list: Vec::new(),
        }
    }

    pub fn advance(&mut self) {
        self.buffer_string.clear();

        while self.input_string.len() > self.input_position {
            let c: char = self.input_string.chars().nth(self.input_position).unwrap();
            self.input_position += 1;

            match self.current_state {
                LexerState::Initial => {
                    match c {
                        '(' => {
                            self.current_token = Token::PARENS_L;
                            self.current_state = LexerState::Initial;
                            return;
                        }
                        ')' => {
                            self.current_token = Token::PARENS_R;
                            self.current_state = LexerState::Initial;
                            return;
                        }
                        '[' => {
                            self.current_token = Token::BRACKET_L;
                            self.current_state = LexerState::Initial;
                            return;
                        }
                        ']' => {
                            self.current_token = Token::BRACKET_R;
                            self.current_state = LexerState::Initial;
                            return;
                        }
                        '{' => {
                            self.current_token = Token::BRACE_L;
                            self.current_state = LexerState::Initial;
                            return;
                        }
                        '}' => {
                            self.current_token = Token::BRACE_R;
                            self.current_state = LexerState::Initial;
                            return;
                        }
                        '.' => {
                            self.current_token = Token::POINT;
                            self.current_state = LexerState::Initial;
                            return;
                        }
                        ',' => {
                            self.current_token = Token::COMMA;
                            self.current_state = LexerState::Initial;
                            return;
                        }
                        ':' => {
                            self.current_token = Token::COLON;
                            self.current_state = LexerState::Initial;
                            return;
                        }
                        ';' => {
                            self.current_token = Token::SEMICOLON;
                            self.current_state = LexerState::Initial;
                            return;
                        }
                        '+' => {
                            self.current_token = Token::OP_ADD;
                            self.current_state = LexerState::Initial;
                            return;
                        }
                        '*' => {
                            self.current_token = Token::OP_MUL;
                            self.current_state = LexerState::Initial;
                            return;
                        }
                        '/' => {
                            self.current_token = Token::OP_DIV;
                            self.current_state = LexerState::Initial;
                            return;
                        }
                        // special cases
                        '<' => {
                            self.buffer_string.push(c);
                            self.current_token = Token::OP_LT;
                            self.current_state = LexerState::Operator;
                        }
                        '>' => {
                            self.buffer_string.push(c);
                            self.current_token = Token::OP_GT;
                            self.current_state = LexerState::Operator;
                        }
                        '!' => {
                            self.buffer_string.push(c);
                            self.current_token = Token::OP_NOT;
                            self.current_state = LexerState::Operator;
                        }
                        '=' => {
                            self.current_token = Token::OP_ASSIGN;
                            self.current_state = LexerState::Assignment;
                        }
                        '-' => {
                            self.current_token = Token::OP_SUB;
                            self.current_state = LexerState::Separator;
                        }
                        '"' => {
                            self.current_token = Token::LIT_STRING(String::new());
                            self.current_state = LexerState::String;
                        }
                        '\'' => {
                            self.current_token = Token::LIT_CHAR(' ');
                            self.current_state = LexerState::Char;
                        }
                        // ------------------
                        _ if c.is_whitespace() => {
                            self.current_state = LexerState::Initial;
                        }
                        _ if c.is_ascii_alphabetic() => {
                            self.buffer_string.push(c);
                            self.current_state = LexerState::Identifier;
                        }
                        _ if c.is_ascii_digit() => {
                            self.buffer_string.push(c);
                            self.current_state = LexerState::Number;
                        }
                        _ => {
                            self.current_token = Token::UNDEFINED;
                            self.input_position += 1;
                        }
                    }
                }
                LexerState::Separator => {
                    if c == '>' {
                        self.current_token = Token::ARROW_R;
                        self.current_state = LexerState::Initial;
                    } else {
                        self.current_token = Token::OP_SUB;
                        self.current_state = LexerState::Initial;
                        self.input_position -= 1;
                    }
                    return;
                }
                LexerState::Operator => {
                    if c != ' ' && !(self.input_position == self.input_string.len()) {
                        self.buffer_string.push(c);
                    } else {
                        if self.input_position == self.input_string.len() && c != ' ' {
                            self.buffer_string.push(c);
                        }
                        let operator = self.buffer_string.clone();
                        self.input_position -= 1;

                        match operator.as_str() {
                            "!=" => {
                                self.current_token = Token::OP_NEQ;
                                self.current_state = LexerState::Initial;
                                return;
                            }
                            ">=" => {
                                self.current_token = Token::OP_NLT;
                                self.current_state = LexerState::Initial;
                                return;
                            }
                            "<=" => {
                                self.current_token = Token::OP_NGT;
                                self.current_state = LexerState::Initial;
                                return;
                            }
                            "<" => {
                                self.current_token = Token::OP_LT;
                                self.current_state = LexerState::Initial;
                                return;
                            }
                            ">" => {
                                self.current_token = Token::OP_GT;
                                self.current_state = LexerState::Initial;
                                return;
                            }
                            _ => {
                                self.current_token = Token::UNDEFINED;
                                self.current_state = LexerState::Initial;
                                return;
                            }
                        };
                    }
                }
                LexerState::Assignment => {
                    if c == '=' {
                        self.current_token = Token::OP_EQ;
                        self.current_state = LexerState::Initial;
                    } else {
                        self.current_token = Token::OP_ASSIGN;
                        self.current_state = LexerState::Initial;
                        self.input_position -= 1;
                    }
                    return;
                }
                LexerState::Identifier => {
                    if (c.is_ascii_alphanumeric() || c == '_') && !(self.input_position == self.input_string.len()) {
                        self.buffer_string.push(c);
                    } else {
                        if self.input_position == self.input_string.len() && c.is_ascii_alphanumeric() {
                            self.buffer_string.push(c);
                        }
                        let identifier = self.buffer_string.clone();
                        self.input_position -= 1;

                        match identifier.as_str() {
                            "func" => {
                                self.current_token = Token::KW_FUNC;
                                self.current_state = LexerState::Initial;
                                return;
                            }
                            "let" => {
                                self.current_token = Token::KW_LET;
                                self.current_state = LexerState::Initial;
                                return;
                            }
                            "if" => {
                                self.current_token = Token::KW_IF;
                                self.current_state = LexerState::Initial;
                                return;
                            }
                            "else" => {
                                self.current_token = Token::KW_ELSE;
                                self.current_state = LexerState::Initial;
                                return;
                            }
                            "while" => {
                                self.current_token = Token::KW_WHILE;
                                self.current_state = LexerState::Initial;
                                return;
                            }
                            "return" => {
                                self.current_token = Token::KW_RETURN;
                                self.current_state = LexerState::Initial;
                                return;
                            }
                            "print" => {
                                self.current_token = Token::KW_PRINT;
                                self.current_state = LexerState::Initial;
                                return;
                            }
                            "int32" => {
                                self.current_token = Token::TYPE_INT32;
                                self.current_state = LexerState::Initial;
                                return;
                            }
                            "flt32" => {
                                self.current_token = Token::TYPE_FLT32;
                                self.current_state = LexerState::Initial;
                                return;
                            }
                            "char" => {
                                self.current_token = Token::TYPE_CHAR;
                                self.current_state = LexerState::Initial;
                                return;
                            }
                            "bool" => {
                                self.current_token = Token::TYPE_BOOL;
                                self.current_state = LexerState::Initial;
                                return;
                            }
                            "true" => {
                                self.current_token = Token::LIT_BOOL(true);
                                self.current_state = LexerState::Initial;
                                return;
                            }
                            "false" => {
                                self.current_token = Token::LIT_BOOL(false);
                                self.current_state = LexerState::Initial;
                                return;
                            }
                            "and" => {
                                self.current_token = Token::OP_AND;
                                self.current_state = LexerState::Initial;
                                return;
                            }
                            "or" => {
                                self.current_token = Token::OP_OR;
                                self.current_state = LexerState::Initial;
                                return;
                            }
                            "not" => {
                                self.current_token = Token::OP_NOT;
                                self.current_state = LexerState::Initial;
                                return;
                            }
                            _ => {
                                self.current_token = Token::ID(identifier);
                                self.current_state = LexerState::Initial;
                                return;
                            }
                        };
                    }
                }
                LexerState::Number => {
                    match c {
                        '.' => {
                            self.buffer_string.push(c);
                        }
                        '0'..='9' => {
                            self.buffer_string.push(c);
                        }
                        _ => {
                            let maybe_number = self.buffer_string.clone();

                            if maybe_number.contains('.') {
                                let float_val: f32 = maybe_number.parse().unwrap();
                                self.current_token = Token::LIT_FLT32(float_val);
                            } else {
                                let int_val: i32 = maybe_number.parse().unwrap();
                                self.current_token = Token::LIT_INT32(int_val);
                            }

                            self.current_state = LexerState::Initial;
                            self.input_position -= 1;
                            return;
                        }
                    }
                }
                LexerState::String => {
                    if c == '"' {
                        let literal = self.buffer_string.clone();
                        self.current_token = Token::LIT_STRING(literal);
                        self.current_state = LexerState::Initial;
                        return;
                    } else {
                        self.buffer_string.push(c);
                    }
                }
                LexerState::Char => {
                    if c == '\'' {
                        let literal = self.buffer_string.clone();
                        self.current_token = Token::LIT_CHAR(literal.chars().nth(0).unwrap());
                        self.current_state = LexerState::Initial;
                        return;
                    } else {
                        self.buffer_string.push(c);
                    }
                }
            }
        }

        if self.input_position >= self.input_string.len() {
            self.current_token = Token::EOI;
        }
    }

    pub fn current(&self) -> Token {
        self.current_token.clone()
    }

    pub fn lex(&mut self) {
        while self.current_token != Token::EOI {
            self.advance();
            self.token_list.push(self.current_token.clone());
            println!("{:?}", self.current_token);
        }
    }
}

/*
* Small test suite added partially for test driven development,
* but mostly for sanity to make sure I didn't break anything.
*/
#[cfg(test)]
mod tests {
    #[test]
    fn it_can_lex_an_empty_input() {
        let mut lexer = super::Lexer::new("".to_string());
        lexer.lex();
        let expected = vec![super::Token::EOI];

        assert_eq!(lexer.token_list, expected);
    }

    #[test]
    fn it_can_lex_a_single_parenthesis() {
        let mut lexer = super::Lexer::new("(".to_string());
        lexer.lex();
        let expected = vec![super::Token::PARENS_L, super::Token::EOI];

        assert_eq!(lexer.token_list, expected);
    }

    #[test]
    fn it_can_lex_a_function_declaration() {
        let mut lexer = super::Lexer::new("func add(x : int32) -> int32".to_string());
        lexer.lex();
        let expected = vec![
            super::Token::KW_FUNC,
            super::Token::ID("add".to_string()),
            super::Token::PARENS_L,
            super::Token::ID("x".to_string()),
            super::Token::COLON,
            super::Token::TYPE_INT32,
            super::Token::PARENS_R,
            super::Token::ARROW_R,
            super::Token::TYPE_INT32,
            super::Token::EOI,
        ];

        assert_eq!(lexer.token_list, expected);
    }

    #[test]
    fn it_can_lex_brackets() {
        let mut lexer = super::Lexer::new("()[]{}".to_string());
        lexer.lex();
        let expected = vec![
            super::Token::PARENS_L,
            super::Token::PARENS_R,
            super::Token::BRACKET_L,
            super::Token::BRACKET_R,
            super::Token::BRACE_L,
            super::Token::BRACE_R,
            super::Token::EOI,
        ];

        assert_eq!(lexer.token_list, expected);
    }

    #[test]
    fn it_can_lex_separators() {
        let mut lexer = super::Lexer::new(".,:;->".to_string());
        lexer.lex();
        let expected = vec![
            super::Token::POINT,
            super::Token::COMMA,
            super::Token::COLON,
            super::Token::SEMICOLON,
            super::Token::ARROW_R,
            super::Token::EOI,
        ];

        assert_eq!(lexer.token_list, expected);
    }

    #[test]
    fn it_can_lex_arithmetic_ops() {
        let mut lexer = super::Lexer::new("+ - * /".to_string());
        lexer.lex();
        let expected = vec![
            super::Token::OP_ADD,
            super::Token::OP_SUB,
            super::Token::OP_MUL,
            super::Token::OP_DIV,
            super::Token::EOI,
        ];

        assert_eq!(lexer.token_list, expected);
    }

    #[test]
    fn it_can_lex_relational_ops() {
        let mut lexer = super::Lexer::new("== < > != >= <=".to_string());
        lexer.lex();
        let expected = vec![
            super::Token::OP_EQ,
            super::Token::OP_LT,
            super::Token::OP_GT,
            super::Token::OP_NEQ,
            super::Token::OP_NLT,
            super::Token::OP_NGT,
            super::Token::EOI,
        ];

        assert_eq!(lexer.token_list, expected);
    }

    #[test]
    fn it_can_lex_logical_ops() {
        let mut lexer = super::Lexer::new("and or not".to_string());
        lexer.lex();
        let expected = vec![
            super::Token::OP_AND,
            super::Token::OP_OR,
            super::Token::OP_NOT,
            super::Token::EOI,
        ];

        assert_eq!(lexer.token_list, expected);
    }

    #[test]
    fn it_can_lex_assignment() {
        let mut lexer = super::Lexer::new("let x = y;".to_string());
        lexer.lex();
        let expected = vec![
            super::Token::KW_LET,
            super::Token::ID("x".to_string()),
            super::Token::OP_ASSIGN,
            super::Token::ID("y".to_string()),
            super::Token::SEMICOLON,
            super::Token::EOI,
        ];

        assert_eq!(lexer.token_list, expected);
    }

    #[test]
    fn it_can_lex_keywords() {
        let mut lexer = super::Lexer::new("func let if else while print return".to_string());
        lexer.lex();
        let expected = vec![
            super::Token::KW_FUNC,
            super::Token::KW_LET,
            super::Token::KW_IF,
            super::Token::KW_ELSE,
            super::Token::KW_WHILE,
            super::Token::KW_PRINT,
            super::Token::KW_RETURN,
            super::Token::EOI,
        ];

        assert_eq!(lexer.token_list, expected);
    }

    #[test]
    fn it_can_lex_identifiers() {
        let mut lexer = super::Lexer::new("let ident;".to_string());
        lexer.lex();
        let expected = vec![
            super::Token::KW_LET,
            super::Token::ID("ident".to_string()),
            super::Token::SEMICOLON,
            super::Token::EOI,
        ];

        assert_eq!(lexer.token_list, expected);
    }

    #[test]
    fn it_can_lex_types() {
        let mut lexer = super::Lexer::new("int32 flt32 char".to_string());
        lexer.lex();
        let expected = vec![
            super::Token::TYPE_INT32,
            super::Token::TYPE_FLT32,
            super::Token::TYPE_CHAR,
            super::Token::EOI,
        ];

        assert_eq!(lexer.token_list, expected);
    }

    #[test]
    fn it_can_lex_literals() {
        let mut lexer = super::Lexer::new("123 123.456 'a' \"hello\"".to_string());
        lexer.lex();
        let expected = vec![
            super::Token::LIT_INT32(123),
            super::Token::LIT_FLT32(123.456),
            super::Token::LIT_CHAR('a'),
            super::Token::LIT_STRING("hello".to_string()),
            super::Token::EOI,
        ];

        assert_eq!(lexer.token_list, expected);
    }

    #[test]
    fn it_can_lex_a_function_declaration_with_body() {
        let mut lexer = super::Lexer::new("func add(x : int32) -> int32 [ let value : int32 = 35; value = value + x; return value; ]".to_string());
        lexer.lex();
        let expected = vec![
            super::Token::KW_FUNC,
            super::Token::ID("add".to_string()),
            super::Token::PARENS_L,
            super::Token::ID("x".to_string()),
            super::Token::COLON,
            super::Token::TYPE_INT32,
            super::Token::PARENS_R,
            super::Token::ARROW_R,
            super::Token::TYPE_INT32,
            super::Token::BRACKET_L,
            super::Token::KW_LET,
            super::Token::ID("value".to_string()),
            super::Token::COLON,
            super::Token::TYPE_INT32,
            super::Token::OP_ASSIGN,
            super::Token::LIT_INT32(35),
            super::Token::SEMICOLON,
            super::Token::ID("value".to_string()),
            super::Token::OP_ASSIGN,
            super::Token::ID("value".to_string()),
            super::Token::OP_ADD,
            super::Token::ID("x".to_string()),
            super::Token::SEMICOLON,
            super::Token::KW_RETURN,
            super::Token::ID("value".to_string()),
            super::Token::SEMICOLON,
            super::Token::BRACKET_R,
            super::Token::EOI,
        ];

        assert_eq!(lexer.token_list, expected);
    }
}
// use std::rc::Rc;
// use crate::token::Token;
// use crate::lexer_mockup::Lexer;
// use crate::tree::ExprNode;
// use crate::value::Value;
//
// impl Token {
//     fn binding_powers(token: &Token) -> (i32, i32) {
//         match token {
//             // atom level
//             Token::LIT_BOOL(_) => (100, 100),
//             Token::LIT_I32(_) => (100, 100),
//             Token::ID(_) => (100, 100),
//             // arithmetic level
//             Token::OP_MUL => (40, 21),
//             Token::OP_ADD => (30, 11),
//             Token::OP_SUB => (30, 11),
//             // relational level
//             Token::OP_LT => (20, 11),
//             // logical level
//             Token::OP_AND => (12, 13),
//             Token::OP_OR => (10, 11),
//             // separators
//             Token::COMMA => (0, 0),
//             Token::PAREN_L => (100, 0),
//             Token::PAREN_R => (0, 100),
//             Token::EOI => (0, 0),
//             _ => {
//                 panic!("Missing binding powers for token {:?}", token);
//             }
//         }
//     }
//
//     fn left_bp(&self) -> i32 { Token::binding_powers(self).0 }
//     fn right_bp(&self) -> i32 { Token::binding_powers(self).1 }
// }
//
// pub struct PrattParser {
//     lexer: Lexer,
// }
//
// impl PrattParser {
//     pub fn new(lexer: Lexer) -> PrattParser {
//         PrattParser { lexer }
//     }
//
//     pub fn parse_expression(&mut self) -> ExprNode {
//         self.pratt_driver(Token::EOI.right_bp())
//     }
//
//     fn pratt_driver(&mut self, requested_bp: i32) -> ExprNode {
//         let mut current_token = self.current();
//         self.advance();
//         let mut left_denotation = self.func_prefix(current_token);
//         loop {
//             current_token = self.current();
//             // compare binding powers
//             if requested_bp >= current_token.left_bp() {
//                 // finish subexpression (requested rbp >= curr lbp)
//                 return left_denotation;
//             }
//             // go on with subexpression (requested rbp < curr lbp)
//             self.advance();
//             left_denotation = self.func_infix(current_token, left_denotation);
//         }
//     }
//
//     fn func_prefix(&mut self, token: Token) -> ExprNode {
//         match token {
//             Token::LIT_BOOL(b) => { ExprNode::Value(Value::Bool(b)) }
//             Token::LIT_I32(i) => { ExprNode::Value(Value::I32(i)) }
//             Token::ID(name) => {
//                 if self.peek(Token::PAREN_L) {
//                     self.parse_call(name)
//                 } else {
//                     ExprNode::Variable(name)
//                 }
//             }
//             Token::PARENS_L => {
//                 let expr = self.parse_expression();
//                 self.expect(Token::PARENS_R);
//                 expr
//             }
//             Token::OP_SUB => {
//                 let expr = self.pratt_driver(Token::ID("".to_string()).left_bp());
//                 ExprNode::Neg(Rc::new(expr))
//             }
//             Token::OP_NOT => {
//                 let expr = self.pratt_driver(Token::ID("".to_string()).left_bp());
//                 ExprNode::Not(Rc::new(expr))
//             }
//             Token::EOI => { todo!() }
//             _ => {
//                 panic!("Token {:?} is not a prefix operator!", token);
//             }
//         }
//     }
//
//     fn func_infix(&mut self, token: Token, left_denotation: ExprNode) -> ExprNode {
//         match token {
//             Token::LIT_INT32(_) => { todo!() }
//             Token::OP_MUL => {
//                 let right_denotation = self.pratt_driver(token.right_bp());
//                 ExprNode::Mul(
//                     Rc::new(left_denotation),
//                     Rc::new(right_denotation),
//                 )
//             }
//             Token::OP_ADD => {
//                 let right_denotation = self.pratt_driver(token.right_bp());
//                 ExprNode::Add(
//                     Rc::new(left_denotation),
//                     Rc::new(right_denotation),
//                 )
//             }
//             Token::OP_SUB => {
//                 let right_denotation = self.pratt_driver(token.right_bp());
//                 ExprNode::Sub(
//                     Rc::new(left_denotation),
//                     Rc::new(right_denotation),
//                 )
//             }
//             Token::OP_LT => {
//                 let right_denotation = self.pratt_driver(token.right_bp());
//                 ExprNode::LessThan(
//                     Rc::new(left_denotation),
//                     Rc::new(right_denotation),
//                 )
//             }
//             Token::OP_AND => {
//                 let right_denotation = self.pratt_driver(token.right_bp());
//                 ExprNode::And(
//                     Rc::new(left_denotation),
//                     Rc::new(right_denotation),
//                 )
//             }
//             Token::OP_OR => {
//                 let right_denotation = self.pratt_driver(token.right_bp());
//                 ExprNode::Or(
//                     Rc::new(left_denotation),
//                     Rc::new(right_denotation),
//                 )
//             }
//             Token::EOI => { todo!() }
//             _ => {
//                 panic!("Token {:?} is not an infix operator!", token);
//             }
//         }
//     }
//
//     fn parse_call(&mut self, name: String) -> ExprNode {
//         let mut exprs = vec![];
//         self.expect(Token::PARENS_L);
//         if !self.peek(Token::PARENS_R) {
//             exprs.push(Rc::new(self.parse_expression()));
//             while self.accept(Token::COMMA) {
//                 exprs.push(Rc::new(self.parse_expression()))
//             }
//         }
//         self.expect(Token::PARENS_R);
//
//         ExprNode::Call(name, exprs)
//     }
// }
//
//
// impl PrattParser { // utility functions for lexer
//
//     fn current(&mut self) -> Token {
//         self.lexer.current()
//     }
//
//     fn advance(&mut self) {
//         self.lexer.advance();
//     }
//
//     fn expect(&mut self, token: Token) {
//         if self.current() == token {
//             self.advance();
//         } else {
//             let curr = self.current();
//             panic!("Did expect '{token:?}' but got token {curr:?}!");
//         }
//     }
//
//     fn accept(&mut self, symbol: Token) -> bool {
//         if self.current() == symbol {
//             self.advance();
//             true
//         } else {
//             false
//         }
//     }
//
//     fn peek(&mut self, symbol: Token) -> bool {
//         self.lexer.current() == symbol
//     }
// }
// #![allow(dead_code)]
//
// use std::rc::Rc;
// use crate::token::Token;
// use crate::lexer::Lexer;
// use crate::tree::ExprNode;
//
// struct PrattParser {
//     lexer: Lexer,
// }
//
// impl PrattParser {
//     fn new(lexer : Lexer) -> PrattParser {
//         PrattParser { lexer }
//     }
//
//     fn analyze(&mut self) -> ExprNode {
//         self.pratt_driver(Token::EOI.right_bp() )
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
//     pub fn func_prefix(&mut self, token: Token) -> ExprNode {
//         match token {
//             Token::ID(_) => {
//                 ExprNode::Var(token.get_id_name())
//             }
//             Token::OP_ADD => {
//                 let right_denotation = self.pratt_driver(token.right_bp());
//                 match  {  }
//                 ExprNode::Add(Rc::new(), Rc::new(right_denotation))
//             }
//             Token::OP_ASSIGN => { todo!() }
//             Token::EOI => { todo!() }
//             _ => {
//                 panic!("Missing prefix function for token {:?}", token);
//             }
//         }
//     }
//
//     fn func_infix(&mut self, token: Token, left_denotation: ExprNode) -> ExprNode {
//         match token {
//             Token::LIT_INT32(_) => { todo!() }
//             Token::OP_ADD => {
//                 let mut node = ParseTree::new(token.clone());
//                 let right_denotation = self.pratt_driver(token.right_bp());
//                 node.push(left_denotation);
//                 node.push(right_denotation);
//                 node
//             }
//             Token::OP_ASSIGN => {
//                 let mut node = ParseTree::new(token.clone());
//                 let right_denotation = self.pratt_driver(token.right_bp());
//                 node.push(left_denotation);
//                 node.push(right_denotation);
//                 node
//             }
//             Token::EOI => { todo!() }
//             _ => {
//                 panic!("Missing infix function for token {:?}", token);
//             }
//         }
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
// }
//
//
// impl Token {
//
//     fn binding_powers(token : &Token) -> (i32, i32) {
//         match token {
//             Token::ID(_) => (3,3),
//             Token::OP_ADD => (2,3),
//             Token::OP_ASSIGN=> (2,1),
//             Token::EOI => (0,0),
//             _ => {
//                 panic!("Missing binding powers for token {:?}", token);
//             }
//         }
//     }
//
//     fn left_bp(&self) -> i32 { Token::binding_powers(self).0 }
//     fn right_bp(&self) -> i32 { Token::binding_powers(self).1 }
//
//     fn get_id_name(&self) -> String {
//         match self {
//             Token::ID(name) => name.clone(),
//             _ => panic!("Expected ID token, found {:?}", self)
//         }
//     }
// }
//
//

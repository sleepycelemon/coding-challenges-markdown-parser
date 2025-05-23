use crate::{parser::parser::{Node, NodeType}, tokenizer::types::{Token, TokenType}};

use super::text;

pub fn parse(mut tokens: Vec<Token>) -> (Vec<Token>, Node) {
  // Pop the opening char, we don't need to store it.
  tokens.pop();

  let (mut tokens, rest_text) = text::parse(tokens, TokenType::Bold);

  // Pop the terminating char, we don't need to store it either.
  tokens.pop();

  return (tokens, Node {
    ntype: NodeType::Bold(rest_text),
    children: vec![],
  });
}

// #[cfg(test)]
// mod tests {
//   use crate::{parser::{handlers, parser::{NodeType, TextFormatting}}, tokenizer::types::{Token, TokenType}};

//   pub fn make_token(ttype: TokenType, c: char) -> Token {
//     Token { t: ttype, c: c }
//   }

//   pub fn make_tokens(ttype: TokenType, str: &str) -> Vec<Token> {
//     str.chars().map(|c| make_token(ttype, c)).collect()
//   }

//   #[test]
//   pub fn text_parse_should_parse_text_until_new_line() {
//     let mut tokens = make_tokens(TokenType::Character, "Hello");

//     tokens.push(make_token(TokenType::NewLine, '\n'));

//     tokens.reverse();

//     let (tokens, node) = handlers::text::parse(tokens, TextFormatting::Normal, TokenType::NewLine);

//     print!("{:?}", node);

//     assert!(node.ntype == NodeType::Text("Hello".to_string(), TextFormatting::Normal));

//     // It should not consume the new line, but should have consumed the rest.
//     assert!(tokens.len() == 1);
//     assert!(tokens[0].t == TokenType::NewLine);
//     assert!(tokens[0].c == '\n');
//   }

//    #[test]
//   pub fn text_parse_should_parse_bold_text_until_new_line() {
//     let mut tokens = make_tokens(TokenType::Character, "Hello");

//     tokens.push(make_token(TokenType::NewLine, '\n'));

//     tokens.reverse();

//     let (tokens, node) = handlers::text::parse(tokens, TextFormatting::Normal, TokenType::NewLine);

//     print!("{:?}", node);

//     assert!(node.ntype == NodeType::Text("Hello".to_string(), TextFormatting::Normal));

//     // It should not consume the new line, but should have consumed the rest.
//     assert!(tokens.len() == 1);
//     assert!(tokens[0].t == TokenType::NewLine);
//     assert!(tokens[0].c == '\n');
//   }
// }
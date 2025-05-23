use crate::{parser::parser::{Node, NodeType}, tokenizer::types::{Token, TokenType}};

use super::text;

pub fn parse(mut tokens: Vec<Token>) -> (Vec<Token>, Node) {
  // Pop the opening char, we don't need to store it.
  tokens.pop();

  let (mut tokens, rest_text) = text::parse(tokens, TokenType::Italic);

  // Pop the terminating char, we don't need to store it either.
  tokens.pop();

  return (tokens, Node {
    ntype: NodeType::Italic(rest_text),
    children: vec![],
  });
}
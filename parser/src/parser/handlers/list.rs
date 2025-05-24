use crate::{parser::parser::{Node, NodeType}, tokenizer::types::{Token, TokenType}};

use super::text;

fn parse_list_item(mut tokens: Vec<Token>) -> (Vec<Token>, Node) {
    // Pop the opening char, we don't need to store it.
    tokens.pop();

   let mut txt = String::new();
   
   'outer: while !tokens.is_empty() {
      match tokens.last().unwrap().t {
        TokenType::Character | TokenType::Space => {
          let (new_tokens, new_text) = text::parse(tokens, TokenType::NewLine);

          tokens = new_tokens;
          txt = new_text;
        },
        _ => break 'outer
      }
  }

  return (tokens, Node {
    ntype: NodeType::ListItem(txt),
    children: vec![]
  });
}

pub fn parse(mut tokens: Vec<Token>) -> (Vec<Token>, Node) {
  let mut list = Node {
    children: vec![],
    ntype: NodeType::List
  };

  'outer: while !tokens.is_empty() {
    match tokens.last().unwrap().t {
      TokenType::ListItem => {
        let (new_tokens, node) = parse_list_item(tokens);

        tokens = new_tokens;
        list.children.push(node);
      },
      TokenType::NewLine => {
        // We can ignore the newline character as we always take
        // a new line after a list anyway.
        tokens.pop();

        // if next character is another list item, we continue, otherwise break
        if tokens.len() == 0 || tokens.last().unwrap().t != TokenType::ListItem {
          break 'outer;
        }
      },
      _ => break 'outer
    }
  }

  return (tokens, list);
}
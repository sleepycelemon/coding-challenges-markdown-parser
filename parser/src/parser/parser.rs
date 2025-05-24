use std::vec;

use crate::tokenizer::types::{Token, TokenType};

use super::handlers::{bold, heading, italic, list, text};

#[derive(Debug, PartialEq, Eq)]
pub enum NodeType {
  Doc,
  Text(String),
  Heading(u8, String),
  Bold(String),
  Italic(String),
  NewLine,
  Character(char),
  List,
  ListItem(String),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
  pub children: Vec<Node>,
  pub ntype: NodeType,
}

impl Node {
   pub fn pretty_print(&self, depth: usize) {
    println!("{}", self.to_pretty_string(depth))
  }

  pub fn to_pretty_string(&self, depth: usize) -> String {
    let indent = "  ".repeat(depth);
    let mut out = String::new();

    out += &format!("{indent}{{\n");
    out += &format!("{indent}  type: {:?},\n", self.ntype);

    if self.children.is_empty() {
      out += &format!("{indent}  children: []\n");
    } else {
      out += &format!("{indent}  children: [\n");
      for child in &self.children {
        out += &child.to_pretty_string(depth + 2);
      }
      out += &format!("{indent}  ]\n");
    }

    out += &format!("{indent}}}\n");
    out
  }
}

pub fn parse(mut tokens: Vec<Token>) -> Node {
  let mut doc = Node {
    ntype: NodeType::Doc,
    children: vec![]
  };

  while !tokens.is_empty() {
    match tokens.last().unwrap().t {
      TokenType::Heading => {
          let (new_tokens, node) = heading::parse(tokens);
          tokens = new_tokens;
          doc.children.push(node);
      },
      TokenType::Bold => {
        let (new_tokens, node) = bold::parse(tokens);

        tokens = new_tokens;
        doc.children.push(node);
      },
      TokenType::Italic => {
        let (new_tokens, node) = italic::parse(tokens);
        tokens = new_tokens;
        doc.children.push(node);
      },
      TokenType::ListItem => {
        let (new_tokens, node) = list::parse(tokens);
        tokens = new_tokens;
        doc.children.push(node); 
      },
       TokenType::NewLine => {
        doc.children.push(Node { ntype: NodeType::NewLine, children: vec![] });
        tokens.pop();
      },
      TokenType::Character => {
        let (new_tokens, txt) = text::parse_characters(tokens);

        tokens = new_tokens;
        doc.children.push(Node { children: vec![], ntype: NodeType::Text(txt) });
      },
      TokenType::Space => {
        doc.children.push(Node { children: vec![], ntype: NodeType::Character(tokens.pop().unwrap().c) });
      },
    }
  }

  return doc
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::types::{Token, TokenType};

    use super::parse;

  #[test]
  pub fn test() {
    let tokens = vec![
      Token { t: TokenType::Heading, c: '#' }
    ];

    parse(tokens);
  }
}
use crate::{parser::parser::{Node, NodeType}, tokenizer::types::{Token, TokenType}};

use super::text;

pub fn parse(mut tokens: Vec<Token>) -> (Vec<Token>, Node) {
  let mut order = 0;
  let children = vec![];
  let mut txt = String::new();

  'outer: while !tokens.is_empty() {
    match tokens.last().unwrap().t {
      TokenType::Heading => {
        tokens.pop();

        order += 1;
      },
      TokenType::Character | TokenType::Space => {
        let (new_tokens, new_text) = text::parse(tokens, TokenType::NewLine);

        tokens = new_tokens;
        txt = new_text;
      },
      TokenType::NewLine => {
        tokens.pop();
        txt.push('\n');
        break 'outer;
      }
      _ => {
        break 'outer;
      }
    }
  }

  return (tokens, Node {
    ntype: NodeType::Heading(order, txt),
    children,
  });
}

#[cfg(test)]
mod test {
    use crate::{parser::{handlers, parser::NodeType}, tokenizer::types::{Token, TokenType}};

    pub fn make_token(ttype: TokenType, c: char) -> Token {
    Token { t: ttype, c: c }
  }

  pub fn make_tokens(ttype: TokenType, str: &str) -> Vec<Token> {
    str.chars().map(|c| make_token(ttype, c)).collect()
  }

  #[test]
  pub fn heading_parse_should_parse_heading() {
    let mut tokens = vec![];

    tokens.extend(make_tokens(TokenType::Heading, "###"));
    tokens.push(make_token(TokenType::Space, ' '));
    tokens.extend(make_tokens(TokenType::Character, "Hello"));
    tokens.push(make_token(TokenType::NewLine, '\n'));

    tokens.reverse();

    let (_, node) = handlers::heading::parse(tokens);

    node.pretty_print(0);

    assert!(node.ntype == NodeType::Heading(3, " Hello\n".to_string()));
  }
}
  

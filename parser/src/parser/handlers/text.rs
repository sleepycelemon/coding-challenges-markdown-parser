use crate::{tokenizer::types::{Token, TokenType}};

// This function is used to parse free text.
// It *only* parses until a non-character / non-space token is found, then returns.
pub fn parse_characters(mut tokens: Vec<Token>) -> (Vec<Token>, String) {
  let mut txt = String::new();

  'outer: while !tokens.is_empty() {
      match tokens.last().unwrap().t {
        TokenType::Character | TokenType::Space => {
          // we continue parsing.
          txt.push(tokens.pop().unwrap().c);
        },
        _ => break 'outer
      }
    } 

  return (tokens, txt); 
}

pub fn parse(mut tokens: Vec<Token>, terminator: TokenType) -> (Vec<Token>, String) {
  let mut txt = String::new();

  'outer: while !tokens.is_empty() {
      match tokens.last().unwrap().t {
        t if t == terminator => break 'outer,
        // New lines should always break a text block
        TokenType::NewLine => break 'outer,
        _ => txt.push(tokens.pop().unwrap().c),
      }
    } 

  return (tokens, txt);
}

#[cfg(test)]
mod tests {
  use crate::{parser::{handlers}, tokenizer::types::{Token, TokenType}};

  pub fn make_token(ttype: TokenType, c: char) -> Token {
    Token { t: ttype, c: c }
  }

  pub fn make_tokens(ttype: TokenType, str: &str) -> Vec<Token> {
    str.chars().map(|c| make_token(ttype, c)).collect()
  }

  #[test]
  pub fn text_parse_should_parse_text_until_terminator() {
    let mut tokens = make_tokens(TokenType::Character, "Hello");

    tokens.push(make_token(TokenType::NewLine, '\n'));

    tokens.reverse();

    let (_, txt) = handlers::text::parse(tokens, TokenType::NewLine);

    assert!(txt == "Hello");
  }
}
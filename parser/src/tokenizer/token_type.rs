use super::types::{Token, TokenType};

pub fn get_token(c: char) -> Token {
  match c {
    '#' => Token::make(c, TokenType::Heading),
    '*' => Token::make(c, TokenType::Bold),
    ' ' => Token::make(c, TokenType::Space),
    '_' => Token::make(c, TokenType::Italic),
    '\n' => Token::make(c, TokenType::NewLine),
    '-' => Token::make(c, TokenType::ListItem),
    _ => Token::make(c, TokenType::Character)
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_token(t: TokenType, c: char, actual: Token) {
      if actual.t != t {
        panic!(
          "Expected token type for {} to be {:?} but was {:?}",
          c,
          t,
          actual.t
        );
      }

      if actual.c != c {
        panic!(
          "Expected character for {:?} to be {:?} but was {:?}",
          t,
          c,
          actual.c
        ); 
      }
    }

    #[test]
    fn get_type_returns_correct_type_for_char() {
      let under_test = vec![
        ('c', TokenType::Character),
        ('#', TokenType::Heading),
        ('*', TokenType::Bold),
        (' ', TokenType::Space),
        ('_', TokenType::Italic),
        ('\n', TokenType::NewLine)
      ];

      for i in under_test {
        check_token(i.1, i.0, get_token(i.0));
      }
    }
}
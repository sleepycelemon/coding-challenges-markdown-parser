use super::{token_type::get_token, types::{Token}};

pub fn tokenize(raw_str: &str) -> Vec<Token> {
  let mut items = Vec::new();

  for c in raw_str.chars() {
    items.push(get_token(c));
  }

  for t in &items {
    println!("{}", t.str());
  }

  return items;
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::types::*;

    use super::*;

    #[test]
    fn tokenize_returns_correct_list_of_char_tokens() {
      let inp: &str = "hello";

      let tokens = tokenize(inp);

      for (i, t) in tokens.iter().enumerate() {
        assert!(t.c == inp.chars().nth(i).unwrap());
        assert!(t.t == TokenType::Character)
      }
    }

     #[test]
    fn tokenize_returns_correct_list_of_tokens_heading() {
      let inp: &str = "# hello\n_What?_\n*Ok*";

      let tokens = tokenize(inp);

      let exp = vec![
        Token { t: TokenType::Heading, c: '#' },
        Token { t: TokenType::Space, c: ' ' },
        Token { t: TokenType::Character, c: 'h' },
        Token { t: TokenType::Character, c: 'e' },
        Token { t: TokenType::Character, c: 'l' },
        Token { t: TokenType::Character, c: 'l' },
        Token { t: TokenType::Character, c: 'o' },
        Token { t: TokenType::NewLine, c: '\n' },
        Token { t: TokenType::Italic, c: '_' },
        Token { t: TokenType::Character, c: 'W' },
        Token { t: TokenType::Character, c: 'h' },
        Token { t: TokenType::Character, c: 'a' },
        Token { t: TokenType::Character, c: 't' },
        Token { t: TokenType::Character, c: '?' },
        Token { t: TokenType::Italic, c: '_' },
        Token { t: TokenType::NewLine, c: '\n' },
        Token { t: TokenType::Bold, c: '*' },
        Token { t: TokenType::Character, c: 'O' },
        Token { t: TokenType::Character, c: 'k' },
        Token { t: TokenType::Bold, c: '*' },
      ];

      for (i, t) in tokens.iter().enumerate() {
        let expected = &exp[i];

        if expected.t != t.t {
          panic!("Expected[{}]: Expected {:?} for character {} but got {:?}", i, expected.t, expected.c, t.t);
        }

        if expected.c != t.c {
         panic!("Expected[{}]: Expected char {} for {:?} but got {}", i, expected.c, t.t, t.c); 
        }
      }
    }
}
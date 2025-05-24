#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum TokenType {
  Character,
  Heading,
  Bold,
  Space,
  Italic,
  NewLine,
  ListItem,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Token {
  pub t: TokenType,
  pub c: char
}

impl Token {
  pub fn make(c: char, t: TokenType) -> Self {
    Self {
      c,
      t
    }
  }

  pub fn str(&self) -> String {
    format!("Token (char: {:?}, type: {:?})", self.c, self.t)
  }
}

pub struct Tokens {
  pub items: Vec<Token>,
  pub cursor: u8
}
use nom::*;

use core::iter::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
  pub kind: TokenKind,
  pub lexeme: Vec<u8>,
  pub start_line: u32,
  pub end_line: u32,
  pub start_col: u32,
  pub end_col: u32,
}

impl Token {
  pub fn new() -> Token {
    Token{
      kind: TokenKind::Other,
      lexeme: vec![],
      start_line: 0,
      end_line: 0,
      start_col: 0,
      end_col: 0,
    }
  }

  pub fn get_kind(&self) -> TokenKind {
    self.kind
  }

  pub fn set_kind(&mut self, new_kind: TokenKind) {
    self.kind = new_kind;
  }

}


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
  // Keywords
  True,
  False,
  Fn,
  Return,
  Let,
  If,
  // ElseIf,
  Else,
  While,
  //------
  Break,
  Continue,
  Alpha,
  Digit,
  LeftParen,
  RightParen,
  LeftCurly,
  RightCurly,
  Equal,
  EqualEqual,
  NotEqual,
  GreaterThan,
  LessThan,
  GreaterThanOrEqual,
  LessThanOrEqual,
  LogicalAnd,
  LogicalOr,
  Not,
  Plus,
  Dash,
  Multiply,
  Exponent,
  Modulus,
  Quote,
  WhiteSpace,
  Semicolon,
  Comma,
  Slash,
  Other,
  EOF,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Tokens {
  pub tokens: Vec<Token>,
}

impl Tokens {
  pub fn new() -> Tokens {
    Tokens { tokens: vec![] }
  }

  pub fn from(tokens: Vec<Token>) -> Tokens {
    Tokens { tokens }
  }

  pub fn push(&mut self, token: Token) {
    self.tokens.push(token);
  }

  pub fn len(&self) -> usize {
    self.tokens.len()
  }

  pub fn is_done(&self) -> bool {
    if !self.is_empty() {
      match &self.tokens[0].kind {
        TokenKind::EOF => true,
        _ => false,
      }
    } else {
      true
    }
  }

  pub fn is_empty(&self) -> bool {
    self.tokens.is_empty()
  }

}

impl InputLength for Tokens {
  fn input_len(&self) -> usize {
    self.tokens.len()
  }
}

impl InputTake for Tokens{
  fn take(&self, count: usize) -> Self {
    Tokens {
      tokens: self.tokens.iter().take(count).cloned().collect(),
    }
  }
  fn take_split(&self, count: usize) -> (Self, Self) {
    let (left, right) = self.tokens.split_at(count);
    (Tokens { tokens: left.to_vec() }, Tokens { tokens: right.to_vec() })
  }
}

pub fn split_tokens(input: Tokens) -> IResult<Tokens, Token> {
  if input.is_empty() {
    Err(Err::Error(error::Error::new(input, error::ErrorKind::Eof)))
  } else {
    let first = input.tokens[0].clone();
    let rest = input.tokens.into_iter().skip(1).collect();
    Ok((Tokens::from(rest), first))
  }
}

pub fn check_token(pred: &dyn Fn(&Token) -> bool) -> impl Fn(Tokens) -> IResult<Tokens, Token> + '_ {
  move |input: Tokens| {
    let parse_res = split_tokens(input.clone())?;
    if pred(&(parse_res.1)) {
      Ok(parse_res)
    } else {
      combinator::fail(input)
    }
  }
}

pub fn lex(input: &str) -> Tokens {
  let mut tokens = Tokens::new();
  let list = input.as_bytes();
  let mut i = 0;
  let mut line = 1;
  let mut col = 1;
  let mut diff = 0;
  while i < list.len() {
    let c = list[i];
    let mut kind = match c {
      48..=57 => TokenKind::Digit,
      65..=90 | 97..=122 => TokenKind::Alpha,
      32 | 10 | 9=> TokenKind::WhiteSpace,
      61 => { // '='
        if i + 1 < list.len() && list[i + 1] == b'=' {
          i += 1;
          TokenKind::EqualEqual
        } else {
          TokenKind::Equal
        }
      }
      62 => { // '>'
        if i + 1 < list.len() && list[i + 1] == b'=' {
          i += 1;
          TokenKind::GreaterThanOrEqual
        } else {
          TokenKind::GreaterThan
        }
      },
      60 => { // '<'
        if i + 1 < list.len() && list[i + 1] == b'=' {
          i += 1;
          TokenKind::LessThanOrEqual
        } else {
          TokenKind::LessThan
        }
      },
      38 => { // '&'
        if i + 1 < list.len() && list[i + 1] == b'&' {
          i += 1;
          TokenKind::LogicalAnd
        } else {
          TokenKind::Other
        }
      },
      124 => { // '|'
        if i + 1 < list.len() && list[i + 1] == b'|' {
          i += 1;
          TokenKind::LogicalOr
        } else {
          TokenKind::Other
        }
      },
      33 => { // '!'
        if i + 1 < list.len() && list[i + 1] == b'=' {
          i += 1;
          TokenKind::NotEqual
        } else {
          TokenKind::Not
        }
      }
      59 => TokenKind::Semicolon,
      123 => TokenKind::LeftCurly,
      125 => TokenKind::RightCurly,
      40 => TokenKind::LeftParen,
      41 => TokenKind::RightParen,
      43 => TokenKind::Plus,
      45 => TokenKind::Dash,
      42 => TokenKind::Multiply,
      47 => TokenKind::Slash,
      94 => TokenKind::Exponent,
      37 => TokenKind::Modulus,
      44 => TokenKind::Comma,
      34 => TokenKind::Quote,
      _ => TokenKind::Other,
    };

    // check if if
    if (c == b'i') && i + 1 < list.len() && list[i + 1] == b'f' {
      kind = TokenKind::If;
      i += 1;
      diff = 1;
    }

    // check if else
    if (c == b'e') && i + 3 < list.len() &&
      list[i + 1] == b'l' && list[i + 2] == b's' && list[i + 3] == b'e' {
      kind = TokenKind::Else;
      i += 3;
      diff = 3;
    }

    // check if while
    if (c == b'w') && i + 4 < list.len() &&
      list[i + 1] == b'h' && list[i + 2] == b'i' && list[i + 3] == b'l' && list[i + 4] == b'e' {
      kind = TokenKind::While;
      i += 4;
      diff = 4;
    }

    //check if fn
    if c == b'f' {
      if i + 1 < list.len(){
        if list[i + 1] == b'n' {
          kind = TokenKind::Fn;
          i += 1;
          diff = 1;
        }
      }
    }
    //check if true
    if c == b't' {
      if i + 3 < list.len(){
        if list[i + 1] == b'r' && list[i + 2] == b'u' && list[i + 3] == b'e' {
          kind = TokenKind::True;
          i += 3;
          diff = 3;
        }
      }
    }
    //check if false
    if c == b'f' {
      if i + 4 < list.len(){
        if list[i + 1] == b'a' && list[i + 2] == b'l' && list[i + 3] == b's' && list[i + 4] == b'e' {
          kind = TokenKind::False;
          i += 4;
          diff = 4;
        }
      }
    }

    //check if break
    if c == b'b' {
      if i + 4 < list.len(){
        if list[i + 1] == b'r' && list[i + 2] == b'e' && list[i + 3] == b'a' && list[i + 4] == b'k' {
          kind = TokenKind::Break;
          i += 4;
          diff = 4;
        }
      }
    }

    //check if continue
    if c == b'c' {
      if i + 7 < list.len(){
        if list[i + 1] == b'o' && list[i + 2] == b'n' && list[i + 3] == b't' && list[i + 4] == b'i' && list[i + 5] == b'n' && list[i + 6] == b'u' && list[i + 7] == b'e' {
          kind = TokenKind::Continue;
          i += 7;
          diff = 7;
        }
      }
    }

    //check if let
    if c == b'l' {
      if i + 2 < list.len(){
        if list[i + 1] == b'e' && list[i + 2] == b't' {
          kind = TokenKind::Let;
          i += 2;
          diff = 2;
        }
      }
    }
    //check if return
    if c == b'r' {
      if i + 5 < list.len(){
        if list[i + 1] == b'e' && list[i + 2] == b't' && list[i + 3] == b'u' && list[i + 4] == b'r' && list[i + 5] == b'n' {
          kind = TokenKind::Return;
          i += 5;
          diff = 5;
        }
      }
    }

    //create token struct
    let token = Token {
      kind,
      lexeme: vec![c],
      start_col: col,
      end_col: col + diff,
      start_line: line,
      end_line: line,
    };
    i +=1;
    diff = 0;
    tokens.push(token.clone());
    col +=1;
    if c == 10{
      line +=1;
      col = 1;
    }
  }

  let token = Token {
    kind: TokenKind::EOF,
    lexeme: vec![],
    start_col: col,
    end_col: col ,
    start_line: line,
    end_line: line,
  };
  tokens.push(token);


  let filtered_tokens: Vec<Token> = tokens.tokens.iter().filter(|tkn| tkn.kind != TokenKind::WhiteSpace).cloned().collect();
  Tokens::from(filtered_tokens)
}

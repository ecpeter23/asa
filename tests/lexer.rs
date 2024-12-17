use asa::*;

fn test_lex(input: &str) -> Vec<TokenKind> {
  lex(input).tokens.iter().map(|t| t.get_kind()).collect::<Vec<TokenKind>>()
}

#[test]
fn lexer_test_01() {
  assert_eq!(test_lex("123"),vec![TokenKind::Digit, TokenKind::Digit, TokenKind::Digit, TokenKind::EOF]);
}

#[test]
fn lexer_test_02() {
  assert_eq!(test_lex("abc"),vec![TokenKind::Alpha, TokenKind::Alpha, TokenKind::Alpha, TokenKind::EOF]);
}

#[test]
fn lexer_test_03() {
  assert_eq!(test_lex("hello world"),vec![TokenKind::Alpha, TokenKind::Alpha, TokenKind::Alpha, TokenKind::Alpha, TokenKind::Alpha,  TokenKind::Alpha, TokenKind::Alpha, TokenKind::Alpha, TokenKind::Alpha, TokenKind::Alpha, TokenKind::EOF]);
}

#[test]
fn lexer_test_04() {
  assert_eq!(test_lex("true"),vec![TokenKind::True, TokenKind::EOF]);
}

#[test]
fn lexer_test_05() {
  assert_eq!(test_lex("false"),vec![TokenKind::False, TokenKind::EOF]);
}

#[test]
fn lexer_test_06() {
  assert_eq!(test_lex("let x = 123;"),vec![
    TokenKind::Let, 
    TokenKind::Alpha, 
    TokenKind::Equal,
    TokenKind::Digit,
    TokenKind::Digit,
    TokenKind::Digit,
    TokenKind::Semicolon,
    TokenKind::EOF,
  ]);
}

#[test]
fn lexer_test_08() {
  assert_eq!(test_lex(r#"fn main() {}"#),vec![
    TokenKind::Fn, 
    TokenKind::Alpha, 
    TokenKind::Alpha,
    TokenKind::Alpha,
    TokenKind::Alpha,
    TokenKind::LeftParen,
    TokenKind::RightParen,
    TokenKind::LeftCurly,
    TokenKind::RightCurly,
    TokenKind::EOF,
  ]);
}


#[test]
fn lexer_test_09() {
  assert_eq!(test_lex(r#"fn foo(a,b,c) {
  let x=a+1;
	let y=bar(c-b);
  return x+y;
}"#),vec![
    TokenKind::Fn, 
    TokenKind::Alpha, 
    TokenKind::Alpha,
    TokenKind::Alpha,
    TokenKind::LeftParen,
    TokenKind::Alpha,
    TokenKind::Comma,
    TokenKind::Alpha,
    TokenKind::Comma,
    TokenKind::Alpha,
    TokenKind::RightParen,
    TokenKind::LeftCurly,
    TokenKind::Let, 
    TokenKind::Alpha,
    TokenKind::Equal,
    TokenKind::Alpha,
    TokenKind::Plus,
    TokenKind::Digit,
    TokenKind::Semicolon,
    TokenKind::Let, 
    TokenKind::Alpha,
    TokenKind::Equal,
    TokenKind::Alpha,
    TokenKind::Alpha,
    TokenKind::Alpha,
    TokenKind::LeftParen,
    TokenKind::Alpha,
    TokenKind::Dash,
    TokenKind::Alpha,
    TokenKind::RightParen,
    TokenKind::Semicolon,
    TokenKind::Return, 
    TokenKind::Alpha,
    TokenKind::Plus,
    TokenKind::Alpha,
    TokenKind::Semicolon,
    TokenKind::RightCurly,
    TokenKind::EOF,
  ]);
}
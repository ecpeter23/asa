// Here is where the various combinators are imported. You can find all the combinators here:
// https://docs.rs/nom/7.1.3/nom/
// If you want to use it in your parser, you need to import it here. I've already imported a couple.

use nom::*;
use nom::{
  IResult,
  branch::alt,
  combinator::{opt, map},
  multi::{many1, many0, separated_list0},
  error::{ErrorKind},
  sequence::{tuple, terminated, delimited}
};
use crate::lexer::*;

// Here are the different node types. You will use these to make your parser.
// You may add other nodes as you see fit.

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
  Program { children: Vec<Node> },
  Block { children: Vec<Node> },
  Statement { children: Vec<Node> },
  FunctionDefine {name: Vec<u8>, children: Vec<Node> },
  FunctionArguments { children: Vec<Node> },
  FunctionStatements { children: Vec<Node> },
  IfExpression { children: Vec<Node> },
  WhileLoop { children: Vec<Node> },
  Expression { children: Vec<Node> },
  FunctionCall { name: Vec<u8>, children: Vec<Node> },
  VariableDefine { children: Vec<Node> },
  ArgumentDefine { children: Vec<Node> },
  Assignment { children: Vec<Node> },
  FunctionReturn { children: Vec<Node> },
  UnaryExpression { name: Vec<u8>, children: Vec<Node> },
  BinaryExpression { name: Vec<u8>, children: Vec<Node> },
  Number { value: Vec<u8> },
  Bool { value: bool },
  Identifier { value: Vec<u8> },
  String { value: Vec<u8> },
  Null,
  Break,
  Continue,
}

// Some helper functions to use Tokens instead of a &str with Nom.
// You'll probably have to create more of these as needed.

pub fn t_alpha(input: Tokens) -> IResult<Tokens, Token> {
  let fxn = check_token(& |tk| match tk.kind {
    TokenKind::Alpha => true,
    _ => false,
  });
  fxn(input.clone())
}

pub fn t_digit(input: Tokens) -> IResult<Tokens, Token> {
  let fxn = check_token(& |tk| match tk.kind {
    TokenKind::Digit => true,
    _ => false,
  });
  fxn(input.clone())
}

pub fn t_alpha1(input: Tokens) -> IResult<Tokens, Vec<Token>> {
  many1(t_alpha)(input)
}

pub fn t_alpha0(input: Tokens) -> IResult<Tokens, Vec<Token>> {
  many0(t_alpha)(input)
}

pub fn t_alphanumeric1(input: Tokens) -> IResult<Tokens, Vec<Token>> {
  many1(alt((t_alpha,t_digit)))(input)
}

pub fn t_alphanumeric0(input: Tokens) -> IResult<Tokens, Vec<Token>> {
  many0(alt((t_alpha,t_digit)))(input)
}

// keywords

pub fn t_let(input: Tokens) -> IResult<Tokens, Token> {
  let fxn = check_token(& |tk| match tk.kind {
    TokenKind::Let => true,
    _ => false,
  });
  fxn(input.clone())
}

// identifier = alpha , <alnum> ;
pub fn identifier(input: Tokens) -> IResult<Tokens, Node> {
  let (input, first) = t_alpha(input)?;
  let (input, rest) = t_alphanumeric0(input)?;
  let mut identifier = first.lexeme;
  for mut tk in rest {
    identifier.append(&mut tk.lexeme);
  }
  Ok((input,Node::Identifier{value: identifier}))
}

// number = {digit} ;
pub fn number(input: Tokens) -> IResult<Tokens, Node> {
  let (input, digits) = many1(t_digit)(input)?;
  let mut number = Vec::new();
  for token in digits {
    number.extend(&token.lexeme);
  }
  Ok((input, Node::Number { value: number }))
}

// boolean = "true" | "false" ;
pub fn boolean(input: Tokens) -> IResult<Tokens, Node> {
  let (input, token) = alt((
    map(check_token(&|tk| tk.kind == TokenKind::True), |tk| tk.clone()),
    map(check_token(&|tk| tk.kind == TokenKind::False), |tk| tk.clone()),
  ))(input)?;
  match token.kind {
    TokenKind::True => Ok((input, Node::Bool { value: true })),
    TokenKind::False => Ok((input, Node::Bool { value: false })),
    _ => unreachable!(),
  }
}

// string = "\"" , {alnum | " "} , "\"" ;
pub fn string(input: Tokens) -> IResult<Tokens, Node> {
  let (input, str_token) = check_token(&|tk| tk.kind == TokenKind::StringLiteral)(input)?;
  Ok((input, Node::String { value: str_token.lexeme }))
}

// function_call = identifier , "(" , [arguments] , ")" ;
pub fn function_call(input: Tokens) -> IResult<Tokens, Node> {
  let (input, (name_node, _, args_opt, _)) = tuple((
    identifier,
    check_token(&|tk| tk.kind == TokenKind::LeftParen),
    opt(arguments),
    check_token(&|tk| tk.kind == TokenKind::RightParen)
  ))(input)?;

  let name = if let Node::Identifier { value } = name_node {
    value
  } else {
    return Err(Err::Error(nom::error::Error::new(
      input,
      ErrorKind::Tag,
    )));
  };

  let function_arguments = Node::FunctionArguments {
    children: if let Some(Node::FunctionArguments { children }) = args_opt {
      children
    } else {
      Vec::new()
    },
  };

  Ok((input, Node::FunctionCall { name, children: vec![function_arguments] }))
}

// value = number | identifier | boolean ;
pub fn value(input: Tokens) -> IResult<Tokens, Node> {
  alt((number, identifier, boolean, string))(input) /* TRIES TO PARSE WITH NUMBER, IDENTIFIER, BOOLEAN AND STRING RETURNS WHICHEVER WORKS */
}

// expression = logical_or | boolean | addition | function_call | number | string | identifier ;
pub fn expression(input: Tokens) -> IResult<Tokens, Node> {
  map(
    alt((
      if_expression,
      logical_or,
      boolean,
      function_call,
      addition,
      number,
      string,
      identifier
    )),
    |node| Node::Expression { children: vec![node] }
  )(input)
}

// logical_or = logical_and { "||" logical_and }
pub fn logical_or(input: Tokens) -> IResult<Tokens, Node> {
  let (input, left) = logical_and(input)?;
  let (input, rest) = many0(
    tuple((
      map(check_token(&|tk| tk.kind == TokenKind::LogicalOr), |_| "||".as_bytes().to_vec()),
      logical_and,
    ))
  )(input)?;

  let node = rest.into_iter().fold(left, |acc, (op, right)| {
    Node::BinaryExpression {
      name: op,
      children: vec![acc, right],
    }
  });

  Ok((input, node))
}

// logical_and = equality { "&&" equality }
pub fn logical_and(input: Tokens) -> IResult<Tokens, Node> {
  let (input, left) = equality(input)?;
  let (input, rest) = many0(
    tuple((
      map(check_token(&|tk| tk.kind == TokenKind::LogicalAnd), |_| "&&".as_bytes().to_vec()),
      equality,
    ))
  )(input)?;

  let node = rest.into_iter().fold(left, |acc, (op, right)| {
    Node::BinaryExpression {
      name: op,
      children: vec![acc, right],
    }
  });

  Ok((input, node))
}

// equality = comparison { ("==" | "!=") comparison }
pub fn equality(input: Tokens) -> IResult<Tokens, Node> {
  let (input, left) = comparison(input)?;
  let (input, rest) = many0(
    tuple((
      alt((
        map(check_token(&|tk| tk.kind == TokenKind::EqualEqual), |_| "==".as_bytes().to_vec()),
        map(check_token(&|tk| tk.kind == TokenKind::NotEqual), |_| "!=".as_bytes().to_vec()),
      )),
      comparison,
    ))
  )(input)?;

  let node = rest.into_iter().fold(left, |acc, (op, right)| {
    Node::BinaryExpression {
      name: op,
      children: vec![acc, right],
    }
  });

  Ok((input, node))
}

// comparison = addition { ("<" | ">" | "<=" | ">=") addition }
pub fn comparison(input: Tokens) -> IResult<Tokens, Node> {
  let (input, left) = addition(input)?;
  let (input, rest) = many0(
    tuple((
      alt((
        map(check_token(&|tk| tk.kind == TokenKind::LessThan), |_| "<".as_bytes().to_vec()),
        map(check_token(&|tk| tk.kind == TokenKind::GreaterThan), |_| ">".as_bytes().to_vec()),
        map(check_token(&|tk| tk.kind == TokenKind::LessThanOrEqual), |_| "<=".as_bytes().to_vec()),
        map(check_token(&|tk| tk.kind == TokenKind::GreaterThanOrEqual), |_| ">=".as_bytes().to_vec()),
      )),
      addition,
    ))
  )(input)?;

  let node = rest.into_iter().fold(left, |acc, (op, right)| {
    Node::BinaryExpression {
      name: op,
      children: vec![acc, right],
    }
  });

  Ok((input, node))
}

// if_expression = "if" , "(" , expression , ")" , "{" , <statements> , "}"
//                { "else" "if" "(" expression ")" "{" <statements> "}" }
//                [ "else" "{" <statements> "}" ];
pub fn if_expression(input: Tokens) -> IResult<Tokens, Node> {
  // Parse the initial if
  let (input, _) = check_token(&|tk| tk.kind == TokenKind::If)(input)?;
  let (input, condition) = alt((
    // With parentheses
    delimited(
      check_token(&|tk| tk.kind == TokenKind::LeftParen),
      expression,
      check_token(&|tk| tk.kind == TokenKind::RightParen),
    ),
    // Without parentheses
    expression,
  ))(input)?;
  let (input, _) = check_token(&|tk| tk.kind == TokenKind::LeftCurly)(input)?;
  let (input, then_statements) = block(input)?;
  let (input, _) = check_token(&|tk| tk.kind == TokenKind::RightCurly)(input)?;

  let mut children = vec![condition, then_statements];

  // Parse zero or more else if branches
  let (input, else_if_branches) = many0(tuple((
    check_token(&|tk| tk.kind == TokenKind::Else),
    check_token(&|tk| tk.kind == TokenKind::If),
    alt((
      delimited(
        check_token(&|tk| tk.kind == TokenKind::LeftParen),
        expression,
        check_token(&|tk| tk.kind == TokenKind::RightParen),
      ),
      expression,
    )),
    check_token(&|tk| tk.kind == TokenKind::LeftCurly),
    block,
    check_token(&|tk| tk.kind == TokenKind::RightCurly),
  )))(input)?;

  // For each else if branch parsed, add condition and block to children
  for (_, _, else_if_condition, _, else_if_block, _) in else_if_branches {
    children.push(else_if_condition);
    children.push(else_if_block);
  }

  // Parse optional else branch
  let (input, else_branch) = opt(tuple((
    check_token(&|tk| tk.kind == TokenKind::Else),
    check_token(&|tk| tk.kind == TokenKind::LeftCurly),
    block,
    check_token(&|tk| tk.kind == TokenKind::RightCurly),
  )))(input)?;

  if let Some((_, _, else_stmts, _)) = else_branch {
    children.push(else_stmts);
  }

  Ok((input, Node::IfExpression { children }))
}

// while_loop = "while" , "(" , expression , ")" , "{" , <statements> , "}" ;
pub fn while_loop(input: Tokens) -> IResult<Tokens, Node> {
  let (input, _) = check_token(&|tk| tk.kind == TokenKind::While)(input)?;
  let (input, condition) = alt((
    delimited(
      check_token(&|tk| tk.kind == TokenKind::LeftParen),
      expression,
      check_token(&|tk| tk.kind == TokenKind::RightParen),
    ),
    expression,
  ))(input)?;
  let (input, _) = check_token(&|tk| tk.kind == TokenKind::LeftCurly)(input)?;
  let (input, body_stmts) = block(input)?;
  let (input, _) = check_token(&|tk| tk.kind == TokenKind::RightCurly)(input)?;

  Ok((input, Node::WhileLoop {
    children: vec![
      condition,
      body_stmts,
    ]
  }))
}

pub fn break_statement(input: Tokens) -> IResult<Tokens, Node> {
  let (input, _) = check_token(&|tk| tk.kind == TokenKind::Break)(input)?;
  Ok((input, Node::Break))
}

pub fn continue_statement(input: Tokens) -> IResult<Tokens, Node> {
  let (input, _) = check_token(&|tk| tk.kind == TokenKind::Continue)(input)?;
  Ok((input, Node::Continue))
}

// addition = multiplication , { ("+" | "-", "%") , multiplication } ;
pub fn addition(input: Tokens) -> IResult<Tokens, Node> {
  let (input, first_mul) = multiplication(input)?;
  let (input, rest) = many0(
    tuple((
      alt((
        check_token(&|tk| tk.kind == TokenKind::Plus),
        check_token(&|tk| tk.kind == TokenKind::Dash),
        check_token(&|tk| tk.kind == TokenKind::Modulus),
      )),
      multiplication,
    ))
  )(input)?;

  let node = rest.into_iter().fold(first_mul, |acc, (op_token, mul_node)| {
    let op_name = match op_token.kind {
      TokenKind::Plus => b"+".to_vec(),
      TokenKind::Dash => b"-".to_vec(),
      TokenKind::Modulus => b"%".to_vec(),
      _ => vec![],
    };
    Node::BinaryExpression {
      name: op_name,
      children: vec![acc, mul_node],
    }
  });

  Ok((input, node))
}

// multiplication = exponentiation , { ("*" | "/") , exponentiation } ;
pub fn multiplication(input: Tokens) -> IResult<Tokens, Node> {
  let (input, first_exp) = exponentiation(input)?;
  let (input, rest) = many0(
    tuple((
      alt((
        check_token(&|tk| tk.kind == TokenKind::Multiply),
        check_token(&|tk| tk.kind == TokenKind::Slash),
      )),
      exponentiation,
    ))
  )(input)?;

  let node = rest.into_iter().fold(first_exp, |acc, (op_token, exp_node)| {
    let op_name = match op_token.kind {
      TokenKind::Multiply => b"*".to_vec(),
      TokenKind::Slash => b"/".to_vec(),
      _ => vec![],
    };
    Node::BinaryExpression {
      name: op_name,
      children: vec![acc, exp_node],
    }
  });

  Ok((input, node))
}

// exponentiation = unary , { "^" , unary } ;
pub fn exponentiation(input: Tokens) -> IResult<Tokens, Node> {
  let (input, first_unary) = unary(input)?;
  let (input, rest) = many0(
    tuple((
      check_token(&|tk| tk.kind == TokenKind::Exponent),
      unary,
    ))
  )(input)?;

  let node = rest.into_iter().fold(first_unary, |acc, (_, unary_node)| {
    Node::BinaryExpression {
      name: b"^".to_vec(),
      children: vec![acc, unary_node],
    }
  });

  Ok((input, node))
}

// unary = [ ("+" | "-" | "!") ] , primary ;
pub fn unary(input: Tokens) -> IResult<Tokens, Node> {
  let (input, opt_op_token) = opt(alt((
    check_token(&|tk| tk.kind == TokenKind::Plus),
    check_token(&|tk| tk.kind == TokenKind::Dash),
    check_token(&|tk| tk.kind == TokenKind::Not),
  )))(input)?;

  let (input, prim_node) = primary(input)?;

  if let Some(op_token) = opt_op_token {
    let op_name = match op_token.kind {
      TokenKind::Plus => b"+".to_vec(),
      TokenKind::Dash => b"-".to_vec(),
      TokenKind::Not => b"!".to_vec(),
      _ => vec![],
    };
    Ok((
      input,
      Node::UnaryExpression {
        name: op_name,
        children: vec![prim_node],
      }
    ))
  } else {
    Ok((input, prim_node))
  }
}

// primary = number | identifier | boolean | string | function_call | "(" , addition , ")" ;
pub fn primary(input: Tokens) -> IResult<Tokens, Node> {
  alt((
    map(
      tuple((
        check_token(&|tk| tk.kind == TokenKind::LeftParen),
        expression,
        check_token(&|tk| tk.kind == TokenKind::RightParen),
      )),
      |(_, expr_node, _)| expr_node,
    ),
    function_call,
    number,
    identifier,
    boolean,
    string,
  ))(input)
}

// statement = variable_define , ";" | assignment , ";" | function_return , ";" | if_expression | while_loop ;
pub fn statement(input: Tokens) -> IResult<Tokens, Node> {
  let (input, stmt_node) = alt((
    // if_expression,
    map(terminated(variable_define, check_token(&|tk| tk.kind == TokenKind::Semicolon)), |node| node),
    map(terminated(assignment, check_token(&|tk| tk.kind == TokenKind::Semicolon)), |node| node),
    map(terminated(function_return, check_token(&|tk| tk.kind == TokenKind::Semicolon)), |node| node),
    map(terminated(break_statement, check_token(&|tk| tk.kind == TokenKind::Semicolon)), |node| node),
    map(terminated(continue_statement, check_token(&|tk| tk.kind == TokenKind::Semicolon)), |node| node),
    if_expression,
    while_loop
  ))(input)?;

  let (input, _) = opt(comment)(input)?;
  Ok((input, stmt_node))
}

// function_return = "return" , (function_call | expression | identifier) ;
pub fn function_return(input: Tokens) -> IResult<Tokens, Node> {
  let (input, _) = check_token(&|tk| tk.kind == TokenKind::Return)(input)?;
  let (input, ret_node) = alt((function_call, expression, identifier))(input)?;
  Ok((input, Node::FunctionReturn { children: vec![ret_node] }))
}

// assignment = identifier , "=" , expression ;
pub fn assignment(input: Tokens) -> IResult<Tokens, Node> {
  let (input, id_node) = identifier(input)?;
  let (input, _) = check_token(&|tk| tk.kind == TokenKind::Equal)(input)?;
  let (input, expr_node) = expression(input)?;
  Ok((input, Node::Assignment { children: vec![id_node, expr_node] }))
}

// variable_define = "let" , identifier , "=" , expression ;
pub fn variable_define(input: Tokens) -> IResult<Tokens, Node> {
  let (input, _) = t_let(input)?;
  let (input, id_node) = identifier(input)?;
  let (input, _) = check_token(&|tk| tk.kind == TokenKind::Equal)(input)?;
  let (input, expr_node) = expression(input)?;
  Ok((input, Node::VariableDefine { children: vec![id_node, expr_node] }))
}

// argument = identifier [ "=" expression ]
pub fn argument(input: Tokens) -> IResult<Tokens, Node> {
  let (input, id_node) = identifier(input)?;

  // Check for a default value
  let (input, default_opt) = opt(tuple((
    check_token(&|tk| tk.kind == TokenKind::Equal),
    expression
  )))(input)?;

  let param_node = if let Some((_, default_expr)) = default_opt {
    Node::ArgumentDefine {
      children: vec![id_node, default_expr]
    }
  } else {
    // No default, just store identifier as param
    // We'll reuse ArgumentDefine node shape:
    // children[0] = id_node
    Node::ArgumentDefine {
      children: vec![id_node]
    }
  };
  Ok((input, param_node))
}

// arguments = argument { "," argument }
pub fn arguments(input: Tokens) -> IResult<Tokens, Node> {
  let (input, params) = separated_list0(
    check_token(&|tk| tk.kind == TokenKind::Comma),
    argument
  )(input)?;

  Ok((input, Node::FunctionArguments { children: params }))
}

// function_define = "fn" , identifier , "(" , [arguments] , ")" , "{" , <statement> , "}" ;
pub fn function_define(input: Tokens) -> IResult<Tokens, Node> {
  let (input, _) = check_token(&|tk| tk.kind == TokenKind::Fn)(input)?;
  let (input, name_node) = identifier(input)?;
  let name = if let Node::Identifier { value } = name_node {
    value
  } else {
    return Err(Err::Error(nom::error::Error::new(input, ErrorKind::Tag)));
  };
  let (input, _) = check_token(&|tk| tk.kind == TokenKind::LeftParen)(input)?;
  let (input, args_opt) = opt(arguments)(input)?;
  let args = if let Some(Node::FunctionArguments { children }) = args_opt {
    children
  } else {
    Vec::new()
  };
  let (input, _) = check_token(&|tk| tk.kind == TokenKind::RightParen)(input)?;
  let (input, _) = check_token(&|tk| tk.kind == TokenKind::LeftCurly)(input)?;
  let (input, stmts) = many0(statement)(input)?;
  let (input, _) = check_token(&|tk| tk.kind == TokenKind::RightCurly)(input)?;

  Ok((input, Node::FunctionDefine {
    name,
    children: vec![
      Node::FunctionArguments { children: args },
      Node::FunctionStatements { children: stmts },
    ]
  }))
}

// comment = "//" , (?any-character? - newline);
pub fn comment(input: Tokens) -> IResult<Tokens, Node> {
  let (input, _) = check_token(&|tk| tk.kind == TokenKind::Slash)(input)?;
  let (input, _) = check_token(&|tk| tk.kind == TokenKind::Slash)(input)?;
  let (input, _) = many0(alt((
    check_token(&|tk| tk.kind != TokenKind::Other),
  )))(input)?;
  Ok((input, Node::Null))
}

// program = {function_definition | statement | comment} ;
pub fn program(input: Tokens) -> IResult<Tokens, Node> {
  map(
    many0(alt((
      function_define,
      statement,
      map(expression, |n| n),
      comment,
    ))),
    |nodes| Node::Program { children: nodes }
  )(input)
}

// block = { statement | comment } ;
pub fn block(input: Tokens) -> IResult<Tokens, Node> {
  map(
    many0(alt((statement, comment))),
    |nodes| Node::Block { children: nodes }
  )(input)
}

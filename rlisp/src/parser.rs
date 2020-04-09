use super::{TokenKind, Token, UniOp, BinOp, AstKind, Ast};
use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParseError {
  UnexpectedToken(Token),
  NotExpression(Token),
  NotOperator(Token),
  UnclosedOpenParen(Token),
  RedundantExpression(Token),
  Eof,
}

fn parse_left_binop<Tokens>(
  tokens: &mut Peekable<Tokens>,
  subexpr_parser: fn(&mut Peekable<Tokens>) -> Result<Ast, ParseError>,
  op_parser: fn(&mut Peekable<Tokens>) -> Result<BinOp, ParseError>,
) -> Result<Ast, ParseError>
where
  Tokens: Iterator<Item = Token>,
{
  let mut e = subexpr_parser(tokens)?;
  loop {
    match tokens.peek() {
      Some(_) => {
        let op = match op_parser(tokens) {
          Ok(op) => op,
          Err(_) => break,
        };
        let r = subexpr_parser(tokens)?;
        let loc = e.loc.merge(&r.loc);
        e = Ast::binop(op, e, r, loc)
      }
      _ => break,
    }
  }
  Ok(e)
}

// atom
fn parse_atom<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
  Tokens: Iterator<Item = Token>,
{
  tokens
    .next()
    .ok_or(ParseError::Eof)
    .and_then(|tok| match tok.value {
      // UNUMBER
      TokenKind::Number(n) => Ok(Ast::new(AstKind::Num(n), tok.loc)),
      // | "(", EXPR, ")" ;
      TokenKind::LParen => {
        let e = parse_expr(tokens)?;
        match tokens.next() {
          Some(Token {
            value: TokenKind::RParen,
            ..
          }) => Ok(e),
          Some(t) => Err(ParseError::RedundantExpression(t)),
          _ => Err(ParseError::UnclosedOpenParen(tok)),
        }
      }
      _ => Err(ParseError::NotExpression(tok)),
    })
}

// expr1
fn parse_expr1<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
  Tokens: Iterator<Item = Token>,
{
  match tokens.peek().map(|tok| tok.value) {
    Some(TokenKind::Plus) | Some(TokenKind::Minus) => {
      // ("+" | "-")
      let op = match tokens.next() {
        Some(Token {
          value: TokenKind::Plus,
          loc,
        }) => UniOp::plus(loc),
        Some(Token {
          value: TokenKind::Minus,
          loc,
        }) => UniOp::minus(loc),
        _ => unreachable!(),
      };
      // , ATOM
      let e = parse_atom(tokens)?;
      let loc = op.loc.merge(&e.loc);
      Ok(Ast::uniop(op, e, loc))
    }
    //  | ATOM
    _ => parse_atom(tokens),
  }
}

fn parse_expr2<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
  Tokens: Iterator<Item = Token>,
{
  fn parse_expr2_op<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<BinOp, ParseError>
  where
    Tokens: Iterator<Item = Token>,
  {
    let op = tokens
      .peek()
      .ok_or(ParseError::Eof)
      .and_then(|tok| match tok.value {
        TokenKind::Asterisk => Ok(BinOp::mul(tok.loc.clone())),
        TokenKind::Slash => Ok(BinOp::div(tok.loc.clone())),
        _ => Err(ParseError::NotOperator(tok.clone())),
      })?;
    tokens.next();
    Ok(op)
  }

  parse_left_binop(tokens, parse_expr1, parse_expr2_op)
}

fn parse_expr3<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
  Tokens: Iterator<Item = Token>,
{
  fn parse_expr3_op<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<BinOp, ParseError>
  where
    Tokens: Iterator<Item = Token>,
  {
    let op = tokens
      .peek()
      .ok_or(ParseError::Eof)
      .and_then(|tok| match tok.value {
        TokenKind::Plus => Ok(BinOp::add(tok.loc.clone())),
        TokenKind::Minus => Ok(BinOp::sub(tok.loc.clone())),
        _ => Err(ParseError::NotOperator(tok.clone())),
      })?;
    tokens.next();
    Ok(op)
  }

  parse_left_binop(tokens, parse_expr2, parse_expr3_op)
}

fn parse_expr4<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
  Tokens: Iterator<Item = Token>,
{
  fn parse_expr4_op<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<BinOp, ParseError>
  where
    Tokens: Iterator<Item = Token>,
  {
    let op = tokens
      .peek()
      .ok_or(ParseError::Eof)
      .and_then(|tok| match tok.value {
        TokenKind::Less => Ok(BinOp::lt(tok.loc.clone())),
        TokenKind::Equal => Ok(BinOp::equal(tok.loc.clone())),
        TokenKind::Greater => Ok(BinOp::gt(tok.loc.clone())),
        _ => Err(ParseError::NotOperator(tok.clone())),
      })?;
    tokens.next();
    Ok(op)
  }

  parse_left_binop(tokens, parse_expr3, parse_expr4_op)
}
fn parse_expr5<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
  Tokens: Iterator<Item = Token>,
{
  fn parse_expr5_op<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<BinOp, ParseError>
  where
    Tokens: Iterator<Item = Token>,
  {
    let op = tokens
      .peek()
      .ok_or(ParseError::Eof)
      .and_then(|tok| match tok.value {
        TokenKind::And => Ok(BinOp::and(tok.loc.clone())),
        TokenKind::Or => Ok(BinOp::or(tok.loc.clone())),
        _ => Err(ParseError::NotOperator(tok.clone())),
      })?;
    tokens.next();
    Ok(op)
  }

  parse_left_binop(tokens, parse_expr4, parse_expr5_op)
}

fn parse_expr<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
  Tokens: Iterator<Item = Token>,
{
  parse_expr5(tokens)
}

pub fn parse(tokens: Vec<Token>) -> Result<Ast, ParseError> {
  let mut tokens = tokens.into_iter().peekable();
  let ret = parse_expr(&mut tokens)?;
  match tokens.next() {
    Some(tok) => Err(ParseError::RedundantExpression(tok)),
    None => Ok(ret),
  }
}
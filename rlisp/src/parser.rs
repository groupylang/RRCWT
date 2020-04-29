use super::token::{TokenKind, Token};
use super::ast::{AstKind, Ast, UniOp, BinOp};
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

fn factor<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
  Tokens: Iterator<Item = Token>,
{
  tokens
    .next()
    .ok_or(ParseError::Eof)
    .and_then(|tok| match tok.value {
      // NUMBER
      TokenKind::Number(n) => Ok(Ast::new(AstKind::Num(n), tok.loc)),
      // | "(" EXPR ")"
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

fn uniop<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
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
      // factor
      let e = factor(tokens)?;
      let loc = op.loc.merge(&e.loc);
      Ok(Ast::uniop(op, e, loc))
    }
    // | factor
    _ => factor(tokens),
  }
}

fn arith_term<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
  Tokens: Iterator<Item = Token>,
{
  fn arith_term_op<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<BinOp, ParseError>
  where
    Tokens: Iterator<Item = Token>,
  {
    // ("*" | "/")
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

  parse_left_binop(tokens, uniop, arith_term_op)
}

fn arith_expr<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
  Tokens: Iterator<Item = Token>,
{
  fn arith_expr_op<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<BinOp, ParseError>
  where
    Tokens: Iterator<Item = Token>,
  {
    // ("+" | "-")
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

  parse_left_binop(tokens, arith_term, arith_expr_op)
}

fn logic_term<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
  Tokens: Iterator<Item = Token>,
{
  fn logic_term_op<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<BinOp, ParseError>
  where
    Tokens: Iterator<Item = Token>,
  {
    // ("<" | "=" | ">")
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

  parse_left_binop(tokens, arith_expr, logic_term_op)
}
fn logic_expr<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
  Tokens: Iterator<Item = Token>,
{
  fn logic_expr_op<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<BinOp, ParseError>
  where
    Tokens: Iterator<Item = Token>,
  {
    // ("&" | "|")
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

  parse_left_binop(tokens, logic_term, logic_expr_op)
}

fn parse_expr<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
  Tokens: Iterator<Item = Token>,
{
  logic_expr(tokens)
}

pub fn parse(tokens: Vec<Token>) -> Result<Ast, ParseError> {
  let mut tokens = tokens.into_iter().peekable();
  let expr = parse_expr(&mut tokens)?;
  match tokens.next() {
    Some(tok) => Err(ParseError::RedundantExpression(tok)),
    None => Ok(expr),
  }
}
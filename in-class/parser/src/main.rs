use rsexp_derive::{OfSexp, SexpOf};
use rsexp::{Sexp, SexpOf};
use std::iter::Peekable;

fn main() {
    let s = "1 * (4 + -5) + 12 + 2";
    // let s = "(1 + 2)";
    // let s = "1 + 2 + 3";
    let toks = lex(s).unwrap();
    println!("{:?}", toks);
    let e = parseE(&mut toks.iter().copied().peekable()).unwrap();
    // println!("{:?}", e);
    let s = e.sexp_of();
    println!("{}", s);
    let exp = e.to_expr();
    println!("{:?}", exp);
}


#[derive(Clone, Copy, Debug, PartialEq)]
enum Tok {
    Num(i32), // n
    Plus,     // +
    Minus,    // -
    Mult,     // *
    LParen,   // (
    RParen    // )
}

fn lex(s: &str) -> Option<Vec<Tok>> {
    let mut s = s;

    fn lex_num(s: &str) -> Option<(Tok, &str)> {
    	let t: String = s.chars().take_while(|c| c.is_numeric()).collect();
    	if t.len() > 0 {
    	    let s2 = &s[t.len()..];
    	    Some((Tok::Num(t.parse().ok()?), s2))
    	} else {
    	    None
    	}
    }

    fn lex_others(s: &str) -> Option<(Tok, &str)> {
	match s.chars().next() {
	    Some('(') => Some((Tok::LParen, &s[1..])),
	    Some(')') => Some((Tok::RParen, &s[1..])),
	    Some('+') => Some((Tok::Plus,   &s[1..])),
	    Some('-') => Some((Tok::Minus,  &s[1..])),
	    Some('*') => Some((Tok::Mult,   &s[1..])),
	    _ => None
	}
    }
    
    let mut toks = vec![];
    while !s.is_empty() {
	let (tok, s2) = lex_num(&s).or_else(|| lex_others(&s))?;
	toks.push(tok);
	s = s2.trim_start()
    }
    Some(toks)
}

#[derive(Clone, Debug, SexpOf)]
enum Expr {
    Num(i32),
    Neg(Box<Expr>),
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Mult(Box<Expr>, Box<Expr>),
}

fn eat<T: Iterator<Item=Tok>>(expected: &Tok, toks: &mut T) -> Result<(), String>{
    let t = toks.next().ok_or("eat: no token")?;
    if t == *expected {
	Ok(())
    } else {
	Err(format!("eat: expected {:?}, got {:?}", expected, t))
    }
}

// fn parseExpr<T: Iterator<Item=Tok>>(toks: &mut T) -> Option<Expr> {
//     use Tok::*;
//     match toks.next()? {
// 	Num(n) => Some(Expr::Num(n)),
// 	Minus => {
// 	    let e = parseExpr(toks)?;
// 	    Some(Expr::Neg(Box::new(e)))
// 	}
// 	LParen => {
// 	    let e = parseExpr(toks)?;
// 	    eat(&RParen, toks)?;
// 	    Some(e)
// 	}
// 	_ => {
// 	    let e1 = parseExpr(toks)?;
// 	    eat(&Plus, toks)?;
// 	    let e2 = parseExpr(toks)?;
// 	    Some(Expr::Plus(Box::new(e1), Box::new(e2)))
// 	}
//     }
// }

// enum E {
//     EPlus(Box<E>, T),
//     ET(T)
// }

// enum T {
//     TMult(Box<T>, F),
//     TF(F)
// }

// enum F {
//     FNum(i32),
//     FParen(Box<E>),
//     FNeg(Box<E>)
// }

#[derive(Debug, SexpOf)]
enum E {
    T(T, Ep)
}

#[derive(Debug, SexpOf)]
enum Ep {
    Plus(T, Box<Ep>),
    Epsilon
}

#[derive(Debug, SexpOf)]
enum T {
    F(F, Tp)
}

#[derive(Debug, SexpOf)]
enum Tp {
    Mult(F, Box<Tp>),
    Epsilon
}

#[derive(Debug, SexpOf)]
enum F {
    Num(i32),
    Paren(Box<E>),
    Neg(Box<E>)
}

fn parseE<I: Iterator<Item=Tok>>(toks: &mut Peekable<I>) -> Result<E, String> {
    let t = parseT(toks)?;
    let e = parseEp(toks)?;
    Ok(E::T(t, e))
}

fn parseEp<I: Iterator<Item=Tok>>(toks: &mut Peekable<I>) -> Result<Ep, String> {
    match toks.peek() {
	Some(Tok::Plus) => {
	    eat(&Tok::Plus, toks);
	    let t = parseT(toks)?;
	    let e = parseEp(toks)?;
	    Ok(Ep::Plus(t, Box::new(e)))
	}
	_ => Ok(Ep::Epsilon)
    }
}

fn parseT<I: Iterator<Item=Tok>>(toks: &mut Peekable<I>) -> Result<T, String> {
    let f = parseF(toks)?;
    let t = parseTp(toks)?;
    Ok(T::F(f, t))
}

fn parseTp<I: Iterator<Item=Tok>>(toks: &mut Peekable<I>) -> Result<Tp, String> {
    match toks.peek() {
	Some(Tok::Mult) => {
	    eat(&Tok::Mult, toks);
	    let f = parseF(toks)?;
	    let t = parseTp(toks)?;
	    Ok(Tp::Mult(f, Box::new(t)))
	}
	_ => Ok(Tp::Epsilon)
    }
}

fn parseF<I: Iterator<Item=Tok>>(toks: &mut Peekable<I>) -> Result<F, String> {
    match toks.peek().ok_or("parseF: no token".to_owned())? {
	Tok::Num(n) => {
	    let n = *n;
	    eat(&Tok::Num(n), toks)?;
	    Ok(F::Num(n))
	}
	Tok::LParen => {
	    eat(&Tok::LParen, toks)?;
	    let e = parseE(toks)?;
	    eat(&Tok::RParen, toks)?;
	    Ok(F::Paren(Box::new(e)))
	}
	Tok::Minus => {
	    eat(&Tok::Minus, toks)?;
	    let e = parseE(toks)?;
	    Ok(F::Neg(Box::new(e)))
	}
	_ => Err("parseF".into())
    }
}

trait ToExpr {
    fn to_expr(&self) -> Expr;
}

impl ToExpr for E {
    fn to_expr(&self) -> Expr {
	match self {
	    E::T(t, ep) => {
		let e1 = t.to_expr();
		ep_to_expr(e1, ep)
	    }
	}
    }
}

fn ep_to_expr(e1: Expr, ep: &Ep) -> Expr {
    match ep {
	Ep::Plus(t, ep2) => {
	    let e2 = t.to_expr();
	    ep_to_expr(Expr::Plus(Box::new(e1), Box::new(e2)), ep2)
	}
	Ep::Epsilon => e1
    }
}

impl ToExpr for T {
    fn to_expr(&self) -> Expr {
	match self {
	    T::F(f, tp) => {
		let e1 = f.to_expr();
		tp_to_expr(e1, tp)
	    }
	}
    }
}

fn tp_to_expr(e1: Expr, tp: &Tp) -> Expr {
    match tp {
	Tp::Mult(f, tp2) => {
	    let e2 = f.to_expr();
	    tp_to_expr(Expr::Mult(Box::new(e1), Box::new(e2)), tp2)
	}
	Tp::Epsilon => e1
    }
}

impl ToExpr for F {
    fn to_expr(&self) -> Expr {
	match self {
	    F::Num(n) => Expr::Num(*n),
	    F::Paren(e) => e.to_expr(),
	    F::Neg(e) => Expr::Neg(Box::new(e.to_expr()))
	}
    }
}


// // Both phases combined.
// fn parseE<I: Iterator<Item=Tok>>(toks: &mut Peekable<I>) -> Result<Expr, String> {
//     let e = parseT(toks)?;
//     parseEp(e, toks)
// }

// fn parseEp<I: Iterator<Item=Tok>>(e1: Expr, toks: &mut Peekable<I>) -> Result<Expr, String> {
//     match toks.peek() {
// 	Some(Tok::Plus) => {
// 	    eat(&Tok::Plus, toks);
// 	    let e2 = parseT(toks)?;
// 	    parseEp(Expr::Plus(Box::new(e1), Box::new(e2)), toks)
// 	}
// 	_ => Ok(e1)
//     }
// }

// fn parseT<I: Iterator<Item=Tok>>(toks: &mut Peekable<I>) -> Result<Expr, String> {
//     let e = parseF(toks)?;
//     parseTp(e, toks)
// }

// fn parseTp<I: Iterator<Item=Tok>>(e1: Expr, toks: &mut Peekable<I>) -> Result<Expr, String> {
//     match toks.peek() {
// 	Some(Tok::Mult) => {
// 	    eat(&Tok::Mult, toks);
// 	    let e2 = parseF(toks)?;
// 	    parseTp(Expr::Mult(Box::new(e1), Box::new(e2)), toks)
// 	}
// 	_ => Ok(e1)
//     }
// }

// fn parseF<I: Iterator<Item=Tok>>(toks: &mut Peekable<I>) -> Result<Expr, String> {
//     match toks.peek().ok_or("parseF: no token".to_owned())? {
// 	Tok::Num(n) => {
// 	    let n = *n;
// 	    eat(&Tok::Num(n), toks)?;
// 	    Ok(Expr::Num(n))
// 	}
// 	Tok::LParen => {
// 	    eat(&Tok::LParen, toks)?;
// 	    let e = parseE(toks)?;
// 	    eat(&Tok::RParen, toks)?;
// 	    Ok(e)
// 	}
// 	Tok::Minus => {
// 	    eat(&Tok::Minus, toks)?;
// 	    let e = parseE(toks)?;
// 	    Ok(Expr::Neg(Box::new(e)))
// 	}
// 	_ => Err("parseF".into())
//     }
// }

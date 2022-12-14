/*
224 227 772

Problem:
Given a string s which represents an expression,
evaluate this expression and return its value.

The integer division should truncate toward zero.

Constraint:
- 1 <= s.length <= 10^5
- s consists of digits, '+', '-', '*', '/', '(', ')', and ' '.
- s represents a valid expression.
- '+' is not used as a unary operation (i.e., "+1" and "+(2 + 3)" is invalid).
- '-' could be used as a unary operation (i.e., "-1" and "-(2 + 3)" is valid).
- every number and running calculation will fit in a signed 32-bit integer.
*/

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token<'t> {
    Val(i32),
    Op(&'t str),
    Eof,
}

struct Lexer<'l> {
    tokens: Vec<Token<'l>>,
}

impl<'l> Lexer<'l> {
    fn new(input: &'l str) -> Self {
        lazy_static! {
            // regular expression is compiled exactly once
            static ref RE: Regex = Regex::new(r"(?P<val>\d+)|(?P<op>\+|\-|\*|/|\(|\))").unwrap();
        }

        let mut tokens: Vec<Token> = RE
            .captures_iter(input)
            .map(|cap| {
                if cap.name("val").is_none() {
                    Token::Op(cap.name("op").unwrap().as_str())
                } else {
                    Token::Val(cap.name("val").unwrap().as_str().parse::<i32>().unwrap())
                }
            })
            .collect();

        tokens.reverse();

        Lexer { tokens }
    }

    fn next<'t>(&mut self) -> Token<'t>
    where
        'l: 't,
    {
        self.tokens.pop().unwrap_or(Token::Eof)
    }

    fn peek<'t>(&mut self) -> Token<'t>
    where
        'l: 't,
    {
        self.tokens.last().copied().unwrap_or(Token::Eof)
    }
}

fn expr(input: &str) -> i32 {
    let mut lexer = Lexer::new(input);
    expr_bp(&mut lexer, 0).unwrap_or(0)
}

fn expr_bp(lexer: &mut Lexer, min_bp: u8) -> Option<i32> {
    let mut lhs = None;

    loop {
        let token = match lexer.peek() {
            Token::Eof => return lhs,
            t => t,
        };

        let r_bp = match binding_power(token, lhs.is_none()) {
            Some((l_bp, r_bp)) if min_bp <= l_bp => r_bp,
            _ => return lhs,
        };
        lexer.next();

        let rhs = expr_bp(lexer, r_bp);
        if token == Token::Op("(") {
            assert_eq!(lexer.next(), Token::Op(")"));
            lhs = rhs;
            continue;
        }

        lhs = operation(token, lhs, rhs)
    }
}

fn binding_power(token: Token, is_prefix: bool) -> Option<(u8, u8)> {
    let res = match token {
        Token::Val(_) => (99, 100),
        Token::Op("(") => (99, 0),
        Token::Op("+") | Token::Op("-") if is_prefix => (99, 9),
        Token::Op("+") | Token::Op("-") => (5, 6),
        Token::Op("*") | Token::Op("/") => (7, 8),
        _ => return None,
    };
    Some(res)
}

fn operation(token: Token, lhs: Option<i32>, rhs: Option<i32>) -> Option<i32> {
    let res = match token {
        Token::Val(i) => i,
        Token::Op("+") if lhs.is_none() => rhs.unwrap(),
        Token::Op("-") if lhs.is_none() => -1 * rhs.unwrap(),
        Token::Op("+") => lhs.unwrap() + rhs.unwrap(),
        Token::Op("-") => lhs.unwrap() - rhs.unwrap(),
        Token::Op("*") => lhs.unwrap() * rhs.unwrap(),
        Token::Op("/") => lhs.unwrap() / rhs.unwrap(),
        _ => return None,
    };
    Some(res)
}

fn main() {
    let input = "2147483647";
    assert_eq!(expr(input), 2147483647);

    let input = "1 + 1";
    assert_eq!(expr(input), 2);

    let input = " 2-1 + 2 ";
    assert_eq!(expr(input), 3);

    let input = "(1+(4+5+2)-3)+(6+8)";
    assert_eq!(expr(input), 23);

    let input = "(-1+(2-3)--3)";
    assert_eq!(expr(input), 1);

    let input = "-1+(2-3)--3";
    assert_eq!(expr(input), 1);

    let input = "3+2*2";
    assert_eq!(expr(input), 7);

    let input = " 1 /2 ";
    assert_eq!(expr(input), 0);

    let input = " 3/2 ";
    assert_eq!(expr(input), 1);

    let input = " 3+5 / 2 ";
    assert_eq!(expr(input), 5);

    let input = "-16 +(( 11-2)/--3* ( 1 + 1)) - (- 21)";
    assert_eq!(expr(input), 11);
}

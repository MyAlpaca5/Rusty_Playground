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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Int(i32),
    Op(char),
    Eof,
}

struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    fn new(input: &str) -> Self {
        let mut chars = input
            .chars()
            .filter(|c| !c.is_ascii_whitespace())
            .peekable();
        let mut tokens = Vec::new();

        while let Some(c) = chars.next() {
            match c {
                '0'..='9' => {
                    let mut num = c as i32 - '0' as i32;
                    while matches!(chars.peek(), Some('0'..='9')) {
                        num = num * 10 + chars.next().unwrap().to_digit(10).unwrap() as i32;
                    }
                    tokens.push(Token::Int(num));
                }
                _ => {
                    tokens.push(Token::Op(c));
                }
            }
        }

        tokens.reverse();

        Lexer { tokens }
    }

    fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::Eof)
    }

    fn peek(&mut self) -> Token {
        self.tokens.last().copied().unwrap_or(Token::Eof)
    }
}

fn expr(input: String) -> i32 {
    let mut lexer = Lexer::new(&input);
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
        if token == Token::Op('(') {
            assert_eq!(lexer.next(), Token::Op(')'));
            lhs = rhs;
            continue;
        }

        lhs = operation(token, lhs, rhs)
    }
}

fn binding_power(token: Token, is_prefix: bool) -> Option<(u8, u8)> {
    let res = match token {
        Token::Int(_) => (99, 100),
        Token::Op('(') => (99, 0),
        Token::Op('+') | Token::Op('-') if is_prefix => (99, 9),
        Token::Op('+') | Token::Op('-') => (5, 6),
        Token::Op('*') | Token::Op('/') => (7, 8),
        _ => return None,
    };
    Some(res)
}

fn operation(token: Token, lhs: Option<i32>, rhs: Option<i32>) -> Option<i32> {
    let res = match token {
        Token::Int(i) => i,
        Token::Op('-') if lhs.is_none() => -1 * rhs.unwrap(),
        Token::Op('+') => lhs.unwrap() + rhs.unwrap(),
        Token::Op('-') => lhs.unwrap() - rhs.unwrap(),
        Token::Op('*') => lhs.unwrap() * rhs.unwrap(),
        Token::Op('/') => lhs.unwrap() / rhs.unwrap(),
        _ => return None,
    };
    Some(res)
}

fn main() {
    let input = "2147483647".to_string();
    assert_eq!(expr(input), 2147483647);

    let input = "1 + 1".to_string();
    assert_eq!(expr(input), 2);

    let input = " 2-1 + 2 ".to_string();
    assert_eq!(expr(input), 3);

    let input = "(1+(4+5+2)-3)+(6+8)".to_string();
    assert_eq!(expr(input), 23);

    let input = "(-1+(2-3)--3)".to_string();
    assert_eq!(expr(input), 1);

    let input = "-1+(2-3)--3".to_string();
    assert_eq!(expr(input), 1);

    let input = "3+2*2".to_string();
    assert_eq!(expr(input), 7);

    let input = " 1 /2 ".to_string();
    assert_eq!(expr(input), 0);

    let input = " 3/2 ".to_string();
    assert_eq!(expr(input), 1);

    let input = " 3+5 / 2 ".to_string();
    assert_eq!(expr(input), 5);
}

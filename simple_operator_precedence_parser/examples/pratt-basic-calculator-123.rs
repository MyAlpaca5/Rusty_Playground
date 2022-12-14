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
                    let mut val = c as i32 - '0' as i32;
                    while matches!(chars.peek(), Some('0'..='9')) {
                        val = val * 10 + chars.next().unwrap().to_digit(10).unwrap() as i32;
                    }
                    tokens.push(Token::Int(val));
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
    expr_bp(&mut lexer, 0)
}

fn expr_bp(lexer: &mut Lexer, bp: u8) -> i32 {
    let mut lhs = match lexer.next() {
        Token::Int(i) => i,
        Token::Op('(') => {
            let lhs = expr_bp(lexer, 0);
            assert_eq!(lexer.next(), Token::Op(')'));
            lhs
        }
        Token::Op(op) => {
            let ((), r_bp) = prefix_binding_power(op);
            let lhs = expr_bp(lexer, r_bp);
            unary_operation('-', lhs)
        }
        t => panic!("bad token {:?}", t),
    };

    loop {
        let op = match lexer.peek() {
            Token::Eof => break,
            Token::Op(op) => op,
            t => panic!("bad token {:?}", t),
        };

        if let Some((l_bp, r_bp)) = infix_binding_power(op) {
            if l_bp < bp {
                break;
            }
            lexer.next();

            let rhs = expr_bp(lexer, r_bp);
            lhs = binary_operation(op, lhs, rhs);
            continue;
        }

        break;
    }

    lhs
}

fn unary_operation(op: char, operand: i32) -> i32 {
    match op {
        '-' => operand * -1,
        t => panic!("bad unary op: {:?}", t),
    }
}

fn binary_operation(op: char, operand_1: i32, operand_2: i32) -> i32 {
    match op {
        '+' => operand_1 + operand_2,
        '-' => operand_1 - operand_2,
        '*' => operand_1 * operand_2,
        '/' => operand_1 / operand_2,
        t => panic!("bad unary op: {:?}", t),
    }
}

fn prefix_binding_power(op: char) -> ((), u8) {
    match op {
        '-' => ((), 5),
        t => panic!("bad unary op: {:?}", t),
    }
}

fn infix_binding_power(op: char) -> Option<(u8, u8)> {
    let res = match op {
        '+' | '-' => (1, 2),
        '*' | '/' => (3, 4),
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

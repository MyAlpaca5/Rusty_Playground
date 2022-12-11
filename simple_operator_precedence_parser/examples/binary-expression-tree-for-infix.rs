/*
1597

Problem:
Given a string s which represents an expression,
return any valid binary expression tree, whose in-order traversal
reproduces s after omitting the parenthesis from it.

Constraint:
- 1 <= s.length <= 100
- s consists of digits, '+', '-', '*', '/', '(', ')'.
- operands in s are exactly 1 digit.
- s represents a valid expression.
*/
use std::string::ToString;

struct Node {
    val: char,
    left_child: Option<Box<Node>>,
    right_child: Option<Box<Node>>,
}

impl Node {
    fn new(val: char) -> Self {
        Node {
            val,
            left_child: None,
            right_child: None,
        }
    }
}

impl ToString for Node {
    fn to_string(&self) -> String {
        let mut res = String::new();
        if let Some(left) = &self.left_child {
            res += &left.to_string();
        }
        res.push(self.val);
        if let Some(right) = &self.right_child {
            res += &right.to_string();
        }
        res
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Operand(char),
    Operator(char),
    Eof,
}

struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    fn new(input: &str) -> Self {
        let mut chars = input.chars().peekable();
        let mut tokens = Vec::new();

        while let Some(c) = chars.next() {
            match c {
                '0'..='9' => {
                    tokens.push(Token::Operand(c));
                }
                _ => {
                    tokens.push(Token::Operator(c));
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

fn expr(input: &str) -> Node {
    let mut lexer = Lexer::new(input);
    expr_bp(&mut lexer, 0)
}

fn expr_bp(lexer: &mut Lexer, bp: u8) -> Node {
    let mut lhs = match lexer.next() {
        Token::Operand(val) => Node::new(val),
        Token::Operator('(') => {
            let lhs = expr_bp(lexer, 0);
            assert_eq!(lexer.next(), Token::Operator(')'));
            lhs
        }
        t => panic!("bad token {:?}", t),
    };

    loop {
        let op = match lexer.peek() {
            Token::Eof => break,
            Token::Operator(op) => op,
            t => panic!("bad token {:?}", t),
        };

        if let Some((l_bp, r_bp)) = infix_binding_power(op) {
            if l_bp < bp {
                break;
            }
            lexer.next();

            let rhs = expr_bp(lexer, r_bp);
            let mut mhs = Node::new(op);
            mhs.left_child = Some(Box::new(lhs));
            mhs.right_child = Some(Box::new(rhs));
            lhs = mhs;
            continue;
        }

        break;
    }

    lhs
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
    let mut input = "3*4-2*5".to_string();
    input.retain(|c| !r#"()"#.contains(c));
    assert_eq!(input, expr(&input).to_string());

    let mut input = "2-3/(5*2)+1".to_string();
    input.retain(|c| !r#"()"#.contains(c));
    assert_eq!(input, expr(&input).to_string());

    let mut input = "1+2+3+4+5".to_string();
    input.retain(|c| !r#"()"#.contains(c));
    assert_eq!(input, expr(&input).to_string());
}

/*
227 772

Problem:
Given a string s which represents an expression,
evaluate this expression and return its value.

The integer division should truncate toward zero.

Constraint:
- 1 <= s.length <= 10^5
- s consists of digits, '+', '-', '*', '/', '(', ')', and ' '.
- s represents a valid expression.
- '+' and '-' are not used as a unary operation (i.e., "+1" and "+(2 + 3)" is invalid).
- every number and running calculation will fit in a signed 32-bit integer.
*/

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Operator {
    token: char,
    precedence: u8,
    operation: fn(i32, i32) -> i32,
    left_associative: bool,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    Int(i32),
    Op(Operator),
    LeftParen,
    RightParen,
}
pub struct Lexer {
    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut chars = input
            .chars()
            .filter(|c| !c.is_ascii_whitespace())
            .peekable();
        let mut tokens = Vec::new();

        while let Some(c) = chars.next() {
            let token = match c {
                '0'..='9' => {
                    let mut val = c as i32 - '0' as i32;
                    while matches!(chars.peek(), Some('0'..='9')) {
                        val = val * 10 + chars.next().unwrap().to_digit(10).unwrap() as i32;
                    }
                    Token::Int(val)
                }
                '(' => Token::LeftParen,
                ')' => Token::RightParen,
                '+' => Token::Op(Operator {
                    token: '+',
                    precedence: 3,
                    operation: |x, y| x + y,
                    left_associative: true,
                }),
                '-' => Token::Op(Operator {
                    token: '-',
                    precedence: 3,
                    operation: |x, y| x - y,
                    left_associative: true,
                }),

                '*' => Token::Op(Operator {
                    token: '*',
                    precedence: 5,
                    operation: |x, y| x * y,
                    left_associative: true,
                }),

                '/' => Token::Op(Operator {
                    token: '/',
                    precedence: 5,
                    operation: |x, y| x / y,
                    left_associative: true,
                }),
                t => panic!("bad token: {}", t),
            };

            tokens.push(token);
        }

        tokens.reverse();

        Lexer { tokens }
    }
    pub fn next(&mut self) -> Option<Token> {
        self.tokens.pop()
    }
}

pub fn expr(input: &str) -> i32 {
    let mut lexer = Lexer::new(input);
    calculate(parse(&mut lexer))
}

fn calculate(tokens: Vec<Token>) -> i32 {
    let mut stack = Vec::new();
    for token in tokens {
        match token {
            Token::Int(val) => stack.push(val),
            Token::Op(op) => {
                if let Some(y) = stack.pop() {
                    if let Some(x) = stack.pop() {
                        stack.push((op.operation)(x, y));
                    }
                }
            }
            t => panic!("bad token in output: {:?}", t),
        }
    }

    assert_eq!(stack.len(), 1);
    stack.pop().unwrap()
}

fn pop_til(operators: &mut Vec<Token>, output: &mut Vec<Token>, stop: Token) -> bool {
    while let Some(token) = operators.pop() {
        if token == stop {
            return true;
        }
        output.push(token)
    }
    false
}

fn parse(lexer: &mut Lexer) -> Vec<Token> {
    let mut output: Vec<Token> = Vec::new();
    let mut operators: Vec<Token> = Vec::new();

    while let Some(token) = lexer.next() {
        match token {
            Token::Int(_) => output.push(token),
            Token::LeftParen => operators.push(token),
            Token::Op(op) => {
                while let Some(Token::Op(prev_op)) = operators.last().copied() {
                    let prev_prec = prev_op.precedence;
                    let cur_prec = op.precedence;
                    if (prev_prec > cur_prec) || (prev_prec == cur_prec && op.left_associative) {
                        output.push(operators.pop().unwrap())
                    } else {
                        break;
                    }
                }
                operators.push(token);
            }
            Token::RightParen => {
                if !pop_til(&mut operators, &mut output, Token::LeftParen) {
                    panic!("Mismatched ')'");
                }
            }
        }
    }

    if pop_til(&mut operators, &mut output, Token::LeftParen) {
        panic!("Mismatched '('");
    }

    output
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

    let input = "3+2*2";
    assert_eq!(expr(input), 7);

    let input = " 1 /2 ";
    assert_eq!(expr(input), 0);

    let input = " 3/2 ";
    assert_eq!(expr(input), 1);

    let input = " 3+5 / 2 ";
    assert_eq!(expr(input), 5);
}

// https://en.wikipedia.org/wiki/Shunting_yard_algorithm#The_algorithm_in_detail
// https://rosettacode.org/wiki/Parsing/Shunting-yard_algorithm

use std::fmt;

pub struct Output(Vec<Token>);

#[derive(Copy, Clone, PartialEq)]
pub struct Operator {
    token: char,
    precedence: u8,
    left_associative: bool,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Token {
    Atom(char),
    Op(Operator),
    LeftParen,
    RightParen,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Atom(val) => write!(f, "{}", val),
            Token::Op(op) => write!(f, "{}", op),
            _ => Ok(()),
        }
    }
}

pub struct Lexer {
    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut tokens = input
            .chars()
            .filter(|it| !it.is_ascii_whitespace())
            .map(|t| match t {
                '0'..='9' | 'a'..='z' | 'A'..='Z' => Token::Atom(t),
                '(' => Token::LeftParen,
                ')' => Token::RightParen,
                '=' => Token::Op(Operator {
                    token: '=',
                    precedence: 1,
                    left_associative: true,
                }),
                '+' => Token::Op(Operator {
                    token: '+',
                    precedence: 3,
                    left_associative: true,
                }),
                '-' => Token::Op(Operator {
                    token: '-',
                    precedence: 3,
                    left_associative: true,
                }),

                '*' => Token::Op(Operator {
                    token: '*',
                    precedence: 5,
                    left_associative: true,
                }),

                '/' => Token::Op(Operator {
                    token: '/',
                    precedence: 5,
                    left_associative: true,
                }),

                '.' => Token::Op(Operator {
                    token: '.',
                    precedence: 7,
                    left_associative: false,
                }),

                t => panic!("bad token: {}", t),
            })
            .collect::<Vec<_>>();
        tokens.reverse();
        Lexer { tokens }
    }
    pub fn next(&mut self) -> Option<Token> {
        self.tokens.pop()
    }
}

pub fn expr(input: &str) -> Output {
    let mut lexer = Lexer::new(input);
    Output(parse(&mut lexer))
}

fn tilt_until(operators: &mut Vec<Token>, output: &mut Vec<Token>, stop: Token) -> bool {
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
            Token::Atom(_) => output.push(token),
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
                if !tilt_until(&mut operators, &mut output, Token::LeftParen) {
                    panic!("Mismatched ')'");
                }
            }
        }
    }

    if tilt_until(&mut operators, &mut output, Token::LeftParen) {
        panic!("Mismatched '('");
    }

    output
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut stack = Vec::new();
        for token in &self.0 {
            match token {
                Token::Atom(atom) => stack.push(String::from(*atom)),
                Token::Op(op) => {
                    if let Some(y) = stack.pop() {
                        if let Some(x) = stack.pop() {
                            stack.push(format!("({} {} {})", op.token, x, y))
                        }
                    }
                }
                t => panic!("bad token in output: {}", t),
            }
        }

        assert_eq!(stack.len(), 1);
        write!(f, "{}", stack.pop().unwrap())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let s = expr("1");
        assert_eq!(s.to_string(), "1");

        let s = expr("1 + 2 * 3");
        assert_eq!(s.to_string(), "(+ 1 (* 2 3))");

        let s = expr("1 * 2 + 3");
        assert_eq!(s.to_string(), "(+ (* 1 2) 3)");

        let s = expr("a + b * c * d + e");
        assert_eq!(s.to_string(), "(+ (+ a (* (* b c) d)) e)");
    }

    #[test]
    fn test_right_associative() {
        let s = expr("f . g . h");
        assert_eq!(s.to_string(), "(. f (. g h))");

        let s = expr(" 1 + 2 + f . g . h * 3 * 4");
        assert_eq!(s.to_string(), "(+ (+ 1 2) (* (* (. f (. g h)) 3) 4))");
    }

    #[test]
    fn test_parenthesis() {
        let s = expr("(((0)))");
        assert_eq!(s.to_string(), "0");

        let s = expr("(1 + 2) * 3");
        assert_eq!(s.to_string(), "(* (+ 1 2) 3)");

        let s = expr("1 + (2 * 3)");
        assert_eq!(s.to_string(), "(+ 1 (* 2 3))");
    }

    #[test]
    fn test_assignment() {
        let s = expr("a + b = 0");
        assert_eq!(s.to_string(), "(= (+ a b) 0)");
    }
}

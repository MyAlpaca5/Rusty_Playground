use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Atom(char),
    Op(char),
    Eof,
}

// Lexer
// contains all tokens
pub struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut tokens = input
            .chars()
            .filter(|c| !c.is_ascii_whitespace())
            .map(|c| match c {
                '0'..='9' | 'a'..='z' | 'A'..='Z' => Token::Atom(c),
                _ => Token::Op(c),
            })
            .collect::<Vec<_>>();

        tokens.reverse();

        Lexer { tokens }
    }

    pub fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::Eof)
    }

    pub fn peek(&mut self) -> Token {
        self.tokens.last().copied().unwrap_or(Token::Eof)
    }
}

// S-expression
// example: 1 + 2 * 3 -> (+ 1 (* 2 3))
pub enum S {
    Atom(char),
    Cons(char, Vec<S>),
}

impl fmt::Display for S {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            S::Atom(a) => write!(f, "{}", a),
            S::Cons(head, tail) => {
                write!(f, "({}", head)?;
                for s in tail {
                    write!(f, " {}", s)?;
                }
                write!(f, ")")
            }
        }
    }
}

// parse expression
pub fn expr(input: &str) -> S {
    let mut lexer = Lexer::new(input);
    expr_bp(&mut lexer, 0)
}

pub fn expr_bp(lexer: &mut Lexer, bp: u8) -> S {
    let mut lhs = match lexer.next() {
        Token::Atom(a) => S::Atom(a),
        Token::Op('(') => {
            let lhs = expr_bp(lexer, 0);
            assert_eq!(lexer.next(), Token::Op(')'));
            lhs
        }
        Token::Op(op) => {
            let ((), r_bp) = prefix_binding_power(op);
            let rhs = expr_bp(lexer, r_bp);
            S::Cons(op, vec![rhs])
        }
        t => panic!("bad token: {:?}", t),
    };

    loop {
        let op = match lexer.peek() {
            Token::Eof => break,
            Token::Op(op) => op,
            t => panic!("bad token: {:?}", t),
        };

        if let Some((l_bp, ())) = postfix_binding_power(op) {
            // this operator has less binding power to the left of
            // the current expressions, we can fold the left expression now
            if l_bp < bp {
                break;
            }
            lexer.next();

            lhs = if op == '[' {
                let rhs = expr_bp(lexer, 0);
                assert_eq!(lexer.next(), Token::Op(']'));
                S::Cons(op, vec![lhs, rhs])
            } else {
                S::Cons(op, vec![lhs])
            };
            continue;
        }

        if let Some((l_bp, r_bp)) = infix_binding_power(op) {
            // this operator has less binding power to the left of
            // the current expressions, we can fold the left expression now
            if l_bp < bp {
                break;
            }
            lexer.next();

            lhs = if op == '?' {
                let mhs = expr_bp(lexer, 0);
                assert_eq!(lexer.next(), Token::Op(':'));
                let rhs = expr_bp(lexer, r_bp);
                S::Cons(op, vec![lhs, mhs, rhs])
            } else {
                let rhs = expr_bp(lexer, r_bp);
                S::Cons(op, vec![lhs, rhs])
            };

            continue;
        }

        // will reach this point if op is unrecognized operator, ) ] :
        break;
    }

    lhs
}

fn prefix_binding_power(op: char) -> ((), u8) {
    match op {
        '+' | '-' => ((), 9),
        t => panic!("bad unary op: {:?}", t),
    }
}

// The general rule is that we use an odd priority as base,
// and bump it by one for associativity, if the operator is binary.
// Make this function return option so it will return None when
// encounter ), therefore, we can terminate on (. Otherwise, the only
// termination condition is eof.
fn infix_binding_power(op: char) -> Option<(u8, u8)> {
    let res = match op {
        '=' => (2, 1), // right associative assignment
        '?' => (4, 3), // right associative ternary
        '+' | '-' => (5, 6),
        '*' | '/' => (7, 8),
        '.' => (14, 13), // right associative composition
        _ => return None,
    };
    Some(res)
}

// After we’ve parsed the prefix expression, we can see either a postfix or an infix operator.
// But we bail on unrecognized operators, which is not going to work…
// So, let’s make postfix_binding_power to return an option, for the case where the operator is not postfix.
fn postfix_binding_power(op: char) -> Option<(u8, ())> {
    let res = match op {
        // In general, for the correctness of our algorithm, priorities should never be equal.
        // However, we only compare right bp with left bp. So for two postfix operators it’s OK
        // to have priorities the same, as they are both right.
        '!' | '[' => (11, ()),
        _ => return None,
    };
    Some(res)
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
    fn test_unary() {
        let s = expr("--1 * 2");
        assert_eq!(s.to_string(), "(* (- (- 1)) 2)");

        let s = expr("--f . g");
        assert_eq!(s.to_string(), "(- (- (. f g)))");

        let s = expr("-f * g");
        assert_eq!(s.to_string(), "(* (- f) g)")
    }

    #[test]
    fn test_postfix() {
        let s = expr("-9!");
        assert_eq!(s.to_string(), "(- (! 9))");

        let s = expr("f . g !");
        assert_eq!(s.to_string(), "(! (. f g))");
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
    fn test_subscript() {
        let s = expr("x[0][1]");
        assert_eq!(s.to_string(), "([ ([ x 0) 1)");
    }

    #[test]
    fn test_ternary() {
        let s = expr(
            "a ? b :
         c ? d
         : e",
        );
        assert_eq!(s.to_string(), "(? a b (? c d e))");
    }

    #[test]
    fn test_assignment() {
        let s = expr("a + b = 0");
        assert_eq!(s.to_string(), "(= (+ a b) 0)");

        let s = expr("a = 0 ? b : c = d");
        assert_eq!(s.to_string(), "(= a (= (? 0 b c) d))")
    }
}

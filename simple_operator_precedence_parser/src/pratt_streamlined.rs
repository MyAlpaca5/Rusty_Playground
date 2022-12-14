// in the original post, https://matklad.github.io/2020/04/15/from-pratt-to-dijkstra.html
// the author further simplified the implementation by removing one more return,
// however, that implementation doesn't seem intuitive for me,
// so I will use the two return version in here.

use std::fmt;

pub enum S {
    Cons(char, Vec<S>),
}

impl fmt::Display for S {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            S::Cons(head, rest) => {
                if rest.is_empty() {
                    write!(f, "{}", head)
                } else {
                    write!(f, "({}", head)?;
                    for s in rest {
                        write!(f, " {}", s)?
                    }
                    write!(f, ")")
                }
            }
        }
    }
}

pub struct Lexer {
    pub tokens: Vec<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut tokens = input
            .chars()
            .filter(|it| !it.is_ascii_whitespace())
            .collect::<Vec<_>>();
        tokens.reverse();
        Lexer { tokens }
    }
    pub fn next(&mut self) -> Option<char> {
        self.tokens.pop()
    }
    pub fn peek(&mut self) -> Option<char> {
        self.tokens.last().copied()
    }
}

pub fn expr(input: &str) -> S {
    let mut lexer = Lexer::new(input);
    expr_bp(&mut lexer, 0).unwrap()
}

fn expr_bp(lexer: &mut Lexer, bp: u8) -> Option<S> {
    let mut lhs = None;

    loop {
        let token = match lexer.peek() {
            Some(token) => token,
            None => return lhs,
        };

        let r_bp = match binding_power(token, lhs.is_none()) {
            Some((l_bp, r_bp)) if bp <= l_bp => r_bp,
            _ => return lhs,
        };
        lexer.next();

        let rhs = expr_bp(lexer, r_bp);
        // cannot add a binding power for ) as in shunting_yard. If it can be matched in the
        // binding_power(), it will be remove from the Lexer, because of lexer.next(). Therefore, when
        // you come back from the recursion, the following assert_eq will fail, because lexer.next() will
        // not return ).
        if token == '(' {
            assert_eq!(lexer.next(), Some(')'));
            lhs = rhs;
            continue;
        }

        let mut args = Vec::new();
        args.extend(lhs);
        args.extend(rhs);

        lhs = Some(S::Cons(token, args));
    }
}

fn binding_power(op: char, is_prefix: bool) -> Option<(u8, u8)> {
    let res = match op {
        '(' => (99, 0),
        '=' => (2, 1),
        '+' | '-' if is_prefix => (99, 7),
        '+' | '-' => (3, 4),
        '*' | '/' => (5, 6),
        '!' => (9, 100),
        '.' => (12, 11),
        '0'..='9' | 'a'..='z' | 'A'..='Z' => (99, 100),
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
    fn test_assignment() {
        let s = expr("a + b = 0");
        assert_eq!(s.to_string(), "(= (+ a b) 0)");
    }
}

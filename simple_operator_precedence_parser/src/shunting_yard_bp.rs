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
}

struct Frame {
    bp: u8,
    lhs: Option<S>,
    token: Option<char>,
}

pub fn expr(input: &str) -> S {
    let mut lexer = Lexer::new(input);
    expr_bp(&mut lexer).unwrap()
}

fn expr_bp(lexer: &mut Lexer) -> Option<S> {
    let mut top = Frame {
        bp: 0,
        lhs: None,
        token: None,
    };
    let mut stack = Vec::new();

    loop {
        let token = lexer.next();

        let (token, r_bp) = loop {
            match binding_power(token, top.lhs.is_none()) {
                Some((t, (l_bp, r_bp))) if top.bp <= l_bp => break (t, r_bp),
                _ => {
                    let res = top;
                    top = match stack.pop() {
                        Some(it) => it,
                        None => {
                            eprintln!();
                            return res.lhs;
                        }
                    };

                    let mut args = Vec::new();
                    args.extend(top.lhs);
                    args.extend(res.lhs);

                    let token = res.token.unwrap();
                    top.lhs = Some(S::Cons(token, args));
                }
            };
        };

        if token == ')' {
            assert_eq!(top.token, Some('('));
            let res = top;
            top = stack.pop().unwrap();
            top.lhs = res.lhs;
            continue;
        }

        stack.push(top);
        top = Frame {
            bp: r_bp,
            lhs: None,
            token: Some(token),
        };
    }
}
fn binding_power(op: Option<char>, prefix: bool) -> Option<(char, (u8, u8))> {
    let op = op?;
    let res = match op {
        '0'..='9' | 'a'..='z' | 'A'..='Z' => (99, 100),
        '(' => (99, 0),
        ')' => (0, 100),
        '=' => (2, 1),
        '+' | '-' if prefix => (99, 9),
        '+' | '-' => (5, 6),
        '*' | '/' => (7, 8),
        '!' => (11, 100),
        '.' => (14, 13),
        _ => return None,
    };
    Some((op, res))
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

/*
726

Problem:
The atomic element always starts with an uppercase character, then zero or more lowercase letters, representing the name.

One or more digits representing that element's count may follow if the count is greater than 1.
If the count is 1, no digits will follow.
- For example, "H2O" and "H2O2" are possible, but "H1O2" is impossible.

Two formulas are concatenated together to produce another formula.
- For example, "H2O2He3Mg4" is also a formula.

A formula placed in parentheses, and a count (optionally added) is also a formula.
- For example, "(H2O2)" and "(H2O2)3" are formulas.

Return the count of all elements as a string in the following form:
the first name (in sorted order), followed by its count (if that count is more than 1),
followed by the second name (in sorted order), followed by its count (if that count is more than 1), and so on.

Constraint:
- 1 <= formula.length <= 1000
- formula consists of English letters, digits, '(', and ')'.
- formula is always valid.
- every number and running calculation will fit in a 32-bit integer.
*/

use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Elem(String), // this is the Atom type of the original algorithm
    Count(u32),
    OP, // open parenthsis
    CP, // close parenthsis
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
                '1'..='9' => {
                    let mut count = c as u32 - '0' as u32;
                    while matches!(chars.peek(), Some('0'..='9')) {
                        count = count * 10 + (chars.next().unwrap().to_digit(10).unwrap());
                    }
                    tokens.push(Token::Count(count));
                }
                'A'..='Z' => {
                    let mut elem = c.to_string();
                    while matches!(chars.peek(), Some('a'..='z')) {
                        elem.push(chars.next().unwrap());
                    }
                    tokens.push(Token::Elem(elem));
                }
                '(' => tokens.push(Token::OP),
                ')' => tokens.push(Token::CP),
                t => panic!("bad char: {:?}", t),
            }
        }

        tokens.reverse();

        Lexer { tokens }
    }

    fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::Eof)
    }

    fn peek(&mut self) -> Token {
        self.tokens.last().cloned().unwrap_or(Token::Eof)
    }
}

fn expr(input: String) -> String {
    let mut lexer = Lexer::new(&input);
    let counter = expr_helper(&mut lexer);
    counter.into_iter()
        .map(|(k, v)| if v > 1 { format!("{}{}", k, v) } else { k })
        .collect()
}

// because there is only one operator, the postfix Count,
// we don't need the bp parameter or specify the bp for Count,
// it will always have larger bp
fn expr_helper(lexer: &mut Lexer) -> BTreeMap<String, u32> {
    let mut lhs = match lexer.next() {
        // for a new level, we can have two types of openner
        Token::OP => {
            let lhs = expr_helper(lexer);
            assert_eq!(lexer.next(), Token::CP);
            lhs
        }
        Token::Elem(elem) => BTreeMap::from([(elem, 1)]),
        t => panic!("bad token {:?}", t),
    };

    loop {
        match lexer.peek() {
            Token::Eof | Token::CP => break,
            Token::Elem(_) | Token::OP => {
                // Because we don't have 1 in the formula (meaning one element),
                // we could have two element tokens adjcent to each other.
                // For this kind of scenario, we think the lhs expression is finished,
                // and start a new lhs by recursively calling the expr_bp function.
                // This behave the same as encountering a opening parenthsis.
                let rhs = expr_helper(lexer);
                for (k, v) in rhs {
                    lhs.entry(k).and_modify(|vv| *vv += v).or_insert(v);
                }
            }
            Token::Count(count) => {
                lexer.next();
                for (_, v) in lhs.iter_mut() {
                    *v *= count;
                }
            }
        };
    }

    lhs
}

fn main() {
    let input = "H2O".to_string();
    assert_eq!(expr(input), "H2O".to_string());

    let input = "Mg(OH)2".to_string();
    assert_eq!(expr(input), "H2MgO2".to_string());

    let input = "Mg(OH)2C(OH)4".to_string();
    assert_eq!(expr(input), "CH6MgO6".to_string());

    let input = "K4(ON(SO3)2)2".to_string();
    assert_eq!(expr(input), "K4N2O14S4".to_string());
}

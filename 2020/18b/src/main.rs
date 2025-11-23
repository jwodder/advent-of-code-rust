// Based on <https://norasandler.com/2017/12/15/Write-a-Compiler-3.html>
use adventutil::Input;
use std::iter::Peekable;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Token {
    Number(u64),
    Add,
    Mul,
    OpenParen,
    CloseParen,
}

fn tokenize(s: &str) -> Tokenize<'_> {
    Tokenize {
        inner: s.chars().peekable(),
    }
}

#[derive(Clone, Debug)]
struct Tokenize<'a> {
    inner: Peekable<std::str::Chars<'a>>,
}

impl Iterator for Tokenize<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        loop {
            match self.inner.next()? {
                '+' => return Some(Token::Add),
                '*' => return Some(Token::Mul),
                '(' => return Some(Token::OpenParen),
                ')' => return Some(Token::CloseParen),
                c if c.is_ascii_whitespace() => continue,
                c if c.is_ascii_digit() => {
                    let mut digits = String::from(c);
                    while let Some(d) = self.inner.next_if(char::is_ascii_digit) {
                        digits.push(d);
                    }
                    return Some(Token::Number(digits.parse().unwrap()));
                }
                c => panic!("Unexpected character {c:?}"),
            }
        }
    }
}

fn eval_expr(s: &str) -> u64 {
    let mut iter = tokenize(s).peekable();
    let r = eval_expr_tokens(&mut iter);
    assert!(iter.next().is_none(), "Trailing tokens in expression");
    r
}

fn eval_expr_tokens(iter: &mut Peekable<Tokenize<'_>>) -> u64 {
    let mut accum = eval_add_term(iter);
    while iter.next_if_eq(&Token::Mul).is_some() {
        accum *= eval_add_term(iter);
    }
    accum
}

fn eval_add_term(iter: &mut Peekable<Tokenize<'_>>) -> u64 {
    let mut accum = eval_atom(iter);
    while iter.next_if_eq(&Token::Add).is_some() {
        accum += eval_atom(iter);
    }
    accum
}

fn eval_atom(iter: &mut Peekable<Tokenize<'_>>) -> u64 {
    match iter.next() {
        Some(Token::OpenParen) => {
            let value = eval_expr_tokens(iter);
            assert_eq!(iter.next(), Some(Token::CloseParen));
            value
        }
        Some(Token::Number(value)) => value,
        t => panic!("{t:?}"),
    }
}

fn solve(input: Input) -> u64 {
    input.lines().map(|s| eval_expr(&s)).sum()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1 + 2 * 3 + 4 * 5 + 6", 231)]
    #[case("1 + (2 * 3) + (4 * (5 + 6))", 51)]
    #[case("2 * 3 + (4 * 5)", 46)]
    #[case("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445)]
    #[case("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060)]
    #[case("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340)]
    fn examples(#[case] s: &str, #[case] value: u64) {
        assert_eq!(eval_expr(s), value);
    }
}

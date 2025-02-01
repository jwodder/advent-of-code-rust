use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Disc {
    positions: i64,
    initial: i64,
}

impl std::str::FromStr for Disc {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Disc, ParseError> {
        let mut parser = PullParser::new(s);
        parser.skip("Disc #")?;
        let _ = parser.parse_to::<i32, _>(Token::Whitespace)?;
        parser.skip("has ")?;
        let positions = parser.parse_to::<i64, _>(Token::Whitespace)?;
        parser.skip("positions; at time=0, it is at position ")?;
        let initial = parser.parse_to::<i64, _>('.')?;
        parser.eof()?;
        Ok(Disc { positions, initial })
    }
}

fn solve(input: Input) -> i64 {
    crt(input
        .parse_lines::<Disc>()
        .zip(1..)
        .map(|(Disc { positions, initial }, i)| ((-i - initial).rem_euclid(positions), positions)))
    .unwrap()
    .0
}

/// Chinese Remainder Theorem
///
/// Takes a list of pairs `(a, m)`, each representing a congruence `x ≡ a (mod
/// m)`, and returns a pair `(k, n)` representing the solution set of all
/// integers congruent to `k` *modulo* `n`.  Returns `None` if the `m`'s are
/// not pairwise relatively prime.
fn crt<I: IntoIterator<Item = (i64, i64)>>(iter: I) -> Option<(i64, i64)> {
    let congruences = iter.into_iter().collect::<Vec<_>>();
    let mm = congruences.iter().map(|&(_, m)| m).product();
    let mut total = 0;
    for (a, m) in congruences {
        total += a * (mm / m) * modinverse(mm / m, m)?;
    }
    Some((total % mm, mm))
}

/// `modinverse(a, n)` returns the [modular multiplicative inverse][1] of `a`
/// *modulo* `n`, i.e., the smallest positive integer `x` such that `(a * x) %
/// n == 1`.  Returns `None` if `a` is not relatively prime to `n`.
///
/// [1]: https://en.wikipedia.org/wiki/Modular_multiplicative_inverse
fn modinverse(a: i64, n: i64) -> Option<i64> {
    let (mut upper, mut uc) = (n.abs(), 0);
    let (mut lower, mut lc) = (a.rem_euclid(upper), 1);
    while lower > 1 {
        (upper, uc, lower, lc) = (
            lower,
            lc,
            upper.rem_euclid(lower),
            uc - lc * (upper / lower),
        );
    }
    (lower == 1).then_some(lc.rem_euclid(n))
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = Input::from(concat!(
            "Disc #1 has 5 positions; at time=0, it is at position 4.\n",
            "Disc #2 has 2 positions; at time=0, it is at position 1.\n",
        ));
        assert_eq!(solve(input), 5);
    }
}

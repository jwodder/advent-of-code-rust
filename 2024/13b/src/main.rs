use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};

const OFFSET: i64 = 10000000000000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Machine {
    a_x: i64,
    a_y: i64,
    b_x: i64,
    b_y: i64,
    prize_x: i64,
    prize_y: i64,
}

impl Machine {
    fn tokens(&self) -> Option<i64> {
        let mut eq1 = Equation {
            a: self.a_x,
            b: self.b_x,
            c: self.prize_x,
        };
        let mut eq2 = Equation {
            a: self.a_y,
            b: self.b_y,
            c: self.prize_y,
        };
        while eq2.a != 0 {
            let scale = eq1.a / eq2.a;
            (eq1, eq2) = (eq2, eq1 - eq2 * scale);
        }
        if eq2.b == 0 {
            if eq2.c != 0 {
                None
            } else {
                let g = gcd(eq1.a, eq2.b);
                if eq1.c % g == 0 { todo!() } else { None }
            }
        } else if eq2.c % eq2.b != 0 {
            None
        } else {
            let b_presses = eq2.c / eq2.b;
            eq2 = Equation {
                a: 0,
                b: 1,
                c: b_presses,
            };
            eq1 = eq1 - eq2 * eq1.b;
            debug_assert!(eq1.b == 0, "eq1.b = {} != 0", eq1.b);
            if eq1.a == 0 {
                panic!("Equation 1 cancelled out")
            } else if eq1.c % eq1.a == 0 {
                let a_presses = eq1.c / eq1.a;
                Some(3 * a_presses + b_presses)
            } else {
                None
            }
        }
    }
}

// A linear Diophantine equation of the form `ax + by = c`
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Equation {
    a: i64,
    b: i64,
    c: i64,
}

impl std::ops::Mul<i64> for Equation {
    type Output = Equation;

    fn mul(self, rhs: i64) -> Equation {
        Equation {
            a: self.a * rhs,
            b: self.b * rhs,
            c: self.c * rhs,
        }
    }
}

impl std::ops::Sub for Equation {
    type Output = Equation;

    fn sub(self, rhs: Equation) -> Equation {
        Equation {
            a: self.a - rhs.a,
            b: self.b - rhs.b,
            c: self.c - rhs.c,
        }
    }
}

impl std::str::FromStr for Machine {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Machine, ParseError> {
        let mut parser = PullParser::new(s.trim());
        parser.skip("Button A: X+")?;
        let a_x = parser.parse_to::<i64, _>(',')?;
        parser.skip(" Y+")?;
        let a_y = parser.parse_to::<i64, _>(Token::Newline)?;
        parser.skip("Button B: X+")?;
        let b_x = parser.parse_to::<i64, _>(',')?;
        parser.skip(" Y+")?;
        let b_y = parser.parse_to::<i64, _>(Token::Newline)?;
        parser.skip("Prize: X=")?;
        let prize_x = parser.parse_to::<i64, _>(',')?;
        parser.skip(" Y=")?;
        let prize_y = parser.parse_to::<i64, _>(Token::Eof)?;
        Ok(Machine {
            a_x,
            a_y,
            b_x,
            b_y,
            prize_x: prize_x + OFFSET,
            prize_y: prize_y + OFFSET,
        })
    }
}

fn solve(input: Input) -> i64 {
    input
        .paragraphs()
        .filter_map(|s| s.parse::<Machine>().unwrap().tokens())
        .sum()
}

/// Compute the greatest common divisor of two signed integers.  If either
/// argument is zero, the absolute value of the other argument is returned.
/// The result will always be nonnegative regardless of the signs of the
/// arguments.
fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a.abs();
    let mut b = b.abs();
    if a == 0 {
        return b;
    } else if b == 0 {
        return a;
    }
    while b != 0 {
        (a, b) = (b, a.rem_euclid(b));
    }
    a
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let m = concat!(
            "Button A: X+94, Y+34\n",
            "Button B: X+22, Y+67\n",
            "Prize: X=8400, Y=5400\n",
        )
        .parse::<Machine>()
        .unwrap();
        assert!(m.tokens().is_none());
    }

    #[test]
    fn test_example2() {
        let m = concat!(
            "Button A: X+26, Y+66\n",
            "Button B: X+67, Y+21\n",
            "Prize: X=12748, Y=12176\n",
        )
        .parse::<Machine>()
        .unwrap();
        assert!(m.tokens().is_some());
    }

    #[test]
    fn test_example3() {
        let m = concat!(
            "Button A: X+17, Y+86\n",
            "Button B: X+84, Y+37\n",
            "Prize: X=7870, Y=6450\n",
        )
        .parse::<Machine>()
        .unwrap();
        assert!(m.tokens().is_none());
    }

    #[test]
    fn test_example4() {
        let m = concat!(
            "Button A: X+69, Y+23\n",
            "Button B: X+27, Y+71\n",
            "Prize: X=18641, Y=10279\n",
        )
        .parse::<Machine>()
        .unwrap();
        assert!(m.tokens().is_some());
    }
}

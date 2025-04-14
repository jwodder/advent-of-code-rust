use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Machine {
    a_x: i32,
    a_y: i32,
    b_x: i32,
    b_y: i32,
    prize_x: i32,
    prize_y: i32,
}

impl Machine {
    fn tokens(&self) -> Option<i32> {
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
                if eq1.c % g == 0 {
                    todo!()
                } else {
                    None
                }
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
    a: i32,
    b: i32,
    c: i32,
}

impl std::ops::Mul<i32> for Equation {
    type Output = Equation;

    fn mul(self, rhs: i32) -> Equation {
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
        let a_x = parser.parse_to::<i32, _>(',')?;
        parser.skip(" Y+")?;
        let a_y = parser.parse_to::<i32, _>(Token::Newline)?;
        parser.skip("Button B: X+")?;
        let b_x = parser.parse_to::<i32, _>(',')?;
        parser.skip(" Y+")?;
        let b_y = parser.parse_to::<i32, _>(Token::Newline)?;
        parser.skip("Prize: X=")?;
        let prize_x = parser.parse_to::<i32, _>(',')?;
        parser.skip(" Y=")?;
        let prize_y = parser.parse_to::<i32, _>(Token::Eof)?;
        Ok(Machine {
            a_x,
            a_y,
            b_x,
            b_y,
            prize_x,
            prize_y,
        })
    }
}

fn solve(input: Input) -> i32 {
    input
        .paragraphs()
        .filter_map(|s| s.parse::<Machine>().unwrap().tokens())
        .sum()
}

/// Compute the greatest common divisor of two signed integers.  If either
/// argument is zero, the absolute value of the other argument is returned.
/// The result will always be nonnegative regardless of the signs of the
/// arguments.
fn gcd(a: i32, b: i32) -> i32 {
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
    fn test_example() {
        let input = Input::from(concat!(
            "Button A: X+94, Y+34\n",
            "Button B: X+22, Y+67\n",
            "Prize: X=8400, Y=5400\n",
            "\n",
            "Button A: X+26, Y+66\n",
            "Button B: X+67, Y+21\n",
            "Prize: X=12748, Y=12176\n",
            "\n",
            "Button A: X+17, Y+86\n",
            "Button B: X+84, Y+37\n",
            "Prize: X=7870, Y=6450\n",
            "\n",
            "Button A: X+69, Y+23\n",
            "Button B: X+27, Y+71\n",
            "Prize: X=18641, Y=10279\n",
        ));
        assert_eq!(solve(input), 480);
    }
}

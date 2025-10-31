use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Nanobot {
    x: i32,
    y: i32,
    z: i32,
    r: u32,
}

impl Nanobot {
    fn radius_contains(self, other: Nanobot) -> bool {
        let dist = self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z);
        dist <= self.r
    }
}

impl std::str::FromStr for Nanobot {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Nanobot, ParseError> {
        let mut parser = PullParser::new(s);
        parser.skip("pos=<")?;
        let x = parser.parse_to::<i32, _>(',')?;
        let y = parser.parse_to::<i32, _>(',')?;
        let z = parser.parse_to::<i32, _>(">, r=")?;
        let r = parser.parse_to::<u32, _>(Token::Eof)?;
        Ok(Nanobot { x, y, z, r })
    }
}

fn solve(input: Input) -> usize {
    let bots = input.parse_lines::<Nanobot>().collect::<Vec<_>>();
    let strongest = bots.iter().copied().max_by_key(|&bot| bot.r).unwrap();
    bots.into_iter()
        .filter(|&bot| strongest.radius_contains(bot))
        .count()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from(concat!(
            "pos=<0,0,0>, r=4\n",
            "pos=<1,0,0>, r=1\n",
            "pos=<4,0,0>, r=3\n",
            "pos=<0,2,0>, r=1\n",
            "pos=<0,5,0>, r=3\n",
            "pos=<0,0,3>, r=1\n",
            "pos=<1,1,1>, r=1\n",
            "pos=<1,1,2>, r=1\n",
            "pos=<1,3,1>, r=1\n",
        ));
        assert_eq!(solve(input), 7);
    }
}

use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vec3 {
    fn taxicab_magnitude(self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl std::str::FromStr for Vec3 {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Vec3, ParseError> {
        let mut parser = PullParser::new(s);
        parser.skip('<')?;
        let x = parser.parse_to::<i32, _>(',')?;
        let y = parser.parse_to::<i32, _>(',')?;
        let z = parser.parse_to::<i32, _>('>')?;
        parser.eof()?;
        Ok(Vec3 { x, y, z })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Particle {
    position: Vec3,
    velocity: Vec3,
    acceleration: Vec3,
}

impl Particle {
    fn escapiness(self) -> (i32, i32, i32) {
        (
            self.acceleration.taxicab_magnitude(),
            self.velocity.taxicab_magnitude(),
            self.position.taxicab_magnitude(),
        )
    }
}

impl std::str::FromStr for Particle {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Particle, ParseError> {
        let mut parser = PullParser::new(s);
        parser.skip("p=")?;
        let position = parser.parse_to::<Vec3, _>(", v=")?;
        let velocity = parser.parse_to::<Vec3, _>(", a=")?;
        let acceleration = parser.parse_to::<Vec3, _>(Token::Eof)?;
        Ok(Particle {
            position,
            velocity,
            acceleration,
        })
    }
}

fn solve(input: Input) -> usize {
    input
        .parse_lines::<Particle>()
        .enumerate()
        .min_by_key(|&(_, p)| p.escapiness())
        .unwrap()
        .0
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
            "p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>\n",
            "p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>\n",
        ));
        assert_eq!(solve(input), 0);
    }
}

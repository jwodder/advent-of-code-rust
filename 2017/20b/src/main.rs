use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};
use std::collections::HashMap;

// A guess at how long it takes for all collisions to occur, decreased
// experimentally after the fact
const TICKS: usize = 1000;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
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
    fn advance(mut self) -> Particle {
        self.velocity += self.acceleration;
        self.position += self.velocity;
        self
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
    let mut particles = HashMap::<Vec3, Vec<Particle>>::new();
    for p in input.parse_lines::<Particle>() {
        particles.entry(p.position).or_default().push(p);
        // Assume no collisions at the start
    }
    for _ in 0..TICKS {
        let mut particles2 = HashMap::<Vec3, Vec<Particle>>::new();
        for p in particles.into_values().filter(|v| v.len() == 1).flatten() {
            let p = p.advance();
            particles2.entry(p.position).or_default().push(p);
        }
        particles = particles2;
    }
    particles.len()
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
            "p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>\n",
            "p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>\n",
            "p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>\n",
            "p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>\n",
        ));
        assert_eq!(solve(input), 1);
    }
}

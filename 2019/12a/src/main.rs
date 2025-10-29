use adventutil::pullparser::{ParseError, PullParser};
use adventutil::{Input, unordered_index_pairs};
use std::str::FromStr;

struct Vector {
    x: i32,
    y: i32,
    z: i32,
}

impl Vector {
    fn energy(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

struct Moon {
    pos: Vector,
    velocity: Vector,
}

impl Moon {
    fn step(&mut self) {
        self.pos.x += self.velocity.x;
        self.pos.y += self.velocity.y;
        self.pos.z += self.velocity.z;
    }

    fn energy(&self) -> i32 {
        self.pos.energy() * self.velocity.energy()
    }
}

impl FromStr for Moon {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Moon, ParseError> {
        let mut parser = PullParser::new(s);
        parser.skip("<x=")?;
        let x = parser.parse_to::<i32, _>(", y=")?;
        let y = parser.parse_to::<i32, _>(", z=")?;
        let z = parser.parse_to::<i32, _>(">")?;
        parser.eof()?;
        Ok(Moon {
            pos: Vector { x, y, z },
            velocity: Vector { x: 0, y: 0, z: 0 },
        })
    }
}

fn solve(input: Input, steps: usize) -> i32 {
    let mut moons = input.parse_lines::<Moon>().collect::<Vec<_>>();
    for _ in 0..steps {
        for (i, j) in unordered_index_pairs(moons.len()) {
            let dx = moons[i].pos.x - moons[j].pos.x;
            moons[i].velocity.x -= dx.signum();
            moons[j].velocity.x += dx.signum();
            let dy = moons[i].pos.y - moons[j].pos.y;
            moons[i].velocity.y -= dy.signum();
            moons[j].velocity.y += dy.signum();
            let dz = moons[i].pos.z - moons[j].pos.z;
            moons[i].velocity.z -= dz.signum();
            moons[j].velocity.z += dz.signum();
        }
        for m in &mut moons {
            m.step();
        }
    }
    moons.into_iter().map(|m| m.energy()).sum()
}

fn main() {
    println!("{}", solve(Input::from_env(), 1000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from(concat!(
            "<x=-1, y=0, z=2>\n",
            "<x=2, y=-10, z=-7>\n",
            "<x=4, y=-8, z=8>\n",
            "<x=3, y=5, z=-1>\n",
        ));
        assert_eq!(solve(input, 10), 179);
    }

    #[test]
    fn test_example2() {
        let input = Input::from(concat!(
            "<x=-8, y=-10, z=0>\n",
            "<x=5, y=5, z=10>\n",
            "<x=2, y=-7, z=3>\n",
            "<x=9, y=-8, z=-3>\n",
        ));
        assert_eq!(solve(input, 100), 1940);
    }
}

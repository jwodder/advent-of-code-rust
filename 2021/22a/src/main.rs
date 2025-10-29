use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};
use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Step {
    on: bool,
    xstart: i32,
    xend: i32,
    ystart: i32,
    yend: i32,
    zstart: i32,
    zend: i32,
}

impl Step {
    fn cubes(self) -> impl Iterator<Item = (i32, i32, i32)> {
        self.xrange().flat_map(move |x| {
            self.yrange()
                .flat_map(move |y| self.zrange().map(move |z| (x, y, z)))
        })
    }

    fn xrange(&self) -> RangeInclusive<i32> {
        mkrange(self.xstart, self.xend)
    }

    fn yrange(&self) -> RangeInclusive<i32> {
        mkrange(self.ystart, self.yend)
    }

    fn zrange(&self) -> RangeInclusive<i32> {
        mkrange(self.zstart, self.zend)
    }
}

impl FromStr for Step {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Step, ParseError> {
        let mut parser = PullParser::new(s);
        let on = if parser.skip("on ").is_ok() {
            true
        } else {
            parser.skip("off ")?;
            false
        };
        parser.skip("x=")?;
        let xstart = parser.parse_to::<i32, _>("..")?;
        let xend = parser.parse_to::<i32, _>(",y=")?;
        let ystart = parser.parse_to::<i32, _>("..")?;
        let yend = parser.parse_to::<i32, _>(",z=")?;
        let zstart = parser.parse_to::<i32, _>("..")?;
        let zend = parser.parse_to::<i32, _>(Token::Eof)?;
        Ok(Step {
            on,
            xstart,
            xend,
            ystart,
            yend,
            zstart,
            zend,
        })
    }
}

fn mkrange(start: i32, end: i32) -> RangeInclusive<i32> {
    #[allow(clippy::reversed_empty_ranges)]
    if start > 50 || end < -50 {
        1..=0
    } else {
        start.clamp(-50, 50)..=end.clamp(-50, 50)
    }
}

fn solve(input: Input) -> usize {
    let mut on_cubes = HashSet::new();
    for step in input.parse_lines::<Step>() {
        let on = step.on;
        for cube in step.cubes() {
            if on {
                on_cubes.insert(cube);
            } else {
                on_cubes.remove(&cube);
            }
        }
    }
    on_cubes.len()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from(concat!(
            "on x=10..12,y=10..12,z=10..12\n",
            "on x=11..13,y=11..13,z=11..13\n",
            "off x=9..11,y=9..11,z=9..11\n",
            "on x=10..10,y=10..10,z=10..10\n",
        ));
        assert_eq!(solve(input), 39);
    }

    #[test]
    fn test_example2() {
        let input = Input::from(concat!(
            "on x=-20..26,y=-36..17,z=-47..7\n",
            "on x=-20..33,y=-21..23,z=-26..28\n",
            "on x=-22..28,y=-29..23,z=-38..16\n",
            "on x=-46..7,y=-6..46,z=-50..-1\n",
            "on x=-49..1,y=-3..46,z=-24..28\n",
            "on x=2..47,y=-22..22,z=-23..27\n",
            "on x=-27..23,y=-28..26,z=-21..29\n",
            "on x=-39..5,y=-6..47,z=-3..44\n",
            "on x=-30..21,y=-8..43,z=-13..34\n",
            "on x=-22..26,y=-27..20,z=-29..19\n",
            "off x=-48..-32,y=26..41,z=-47..-37\n",
            "on x=-12..35,y=6..50,z=-50..-2\n",
            "off x=-48..-32,y=-32..-16,z=-15..-5\n",
            "on x=-18..26,y=-33..15,z=-7..46\n",
            "off x=-40..-22,y=-38..-28,z=23..41\n",
            "on x=-16..35,y=-41..10,z=-47..6\n",
            "off x=-32..-23,y=11..30,z=-14..3\n",
            "on x=-49..-5,y=-3..45,z=-29..18\n",
            "off x=18..30,y=-20..-8,z=-3..13\n",
            "on x=-41..9,y=-7..43,z=-33..15\n",
            "on x=-54112..-39298,y=-85059..-49293,z=-27449..7877\n",
            "on x=967..23432,y=45373..81175,z=27513..53682\n",
        ));
        assert_eq!(solve(input), 590784);
    }
}

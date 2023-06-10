// It turns out that the height & width (but not the area!) of the bounding box
// enclosing the points of light both decrease up until the message is formed,
// after which they start increasing again.  Thus, we just need to look for a
// local minimum of either dimension.
use adventutil::grid::{Grid, GridBounds};
use adventutil::gridgeom::{Point, PointBounds, Vector};
use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Star {
    pos: Point,
    velocity: Vector,
}

impl Star {
    fn step(mut self) -> Star {
        self.pos += self.velocity;
        self
    }
}

impl FromStr for Star {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Star, ParseError> {
        let mut parser = PullParser::new(s);
        parser.skip("position=<")?;
        let _ = parser.skip(Token::Whitespace);
        let posx = parser.parse_to::<i32, _>(',')?;
        parser.skip(Token::Whitespace)?;
        let posy = parser.parse_to::<i32, _>('>')?;
        parser.skip(" velocity=<")?;
        let _ = parser.skip(Token::Whitespace);
        let velx = parser.parse_to::<i32, _>(',')?;
        parser.skip(Token::Whitespace)?;
        let vely = parser.parse_to::<i32, _>('>')?;
        parser.eof()?;
        Ok(Star {
            pos: Point { x: posx, y: posy },
            velocity: Vector { x: velx, y: vely },
        })
    }
}

fn solve(input: Input) -> String {
    let mut points = input.parse_lines::<Star>().collect::<Vec<_>>();
    let mut bounds = PointBounds::for_points(points.iter().map(|&s| s.pos)).unwrap();
    loop {
        let newpoints = points.iter().copied().map(Star::step).collect::<Vec<_>>();
        let newbounds = PointBounds::for_points(newpoints.iter().map(|&s| s.pos)).unwrap();
        if newbounds.height() > bounds.height() {
            let ulx = bounds.min_x;
            let uly = bounds.min_y;
            let points = points.into_iter().map(|s| s.pos).collect::<HashSet<_>>();
            let grbounds = GridBounds::new(
                usize::try_from(bounds.height()).unwrap(),
                usize::try_from(bounds.width()).unwrap(),
            );
            return Grid::from_fn(grbounds, |(y, x)| {
                let y = i32::try_from(y).unwrap();
                let x = i32::try_from(x).unwrap();
                points.contains(&Point {
                    x: ulx + x,
                    y: uly + y,
                })
            })
            .draw()
            .to_string();
        }
        points = newpoints;
        bounds = newbounds;
    }
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

/*
fn main() {
    let mut points = Input::from_env().parse_lines::<Star>().collect::<Vec<_>>();
    let mut prev = None;
    for i in 0..11000 {
        let bounds = PointBounds::for_points(points.iter().map(|&s| s.pos)).unwrap();
        println!("{} x {}", bounds.height(), bounds.width());
        match prev {
            Some(p) if p < bounds.width() => {println!("{i}"); return; }
            _ => (),
        }
        let _ = prev.insert(bounds.width());
        points = points.into_iter().map(Star::step).collect();
    }
}
*/

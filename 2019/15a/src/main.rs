use adventutil::gridgeom::{Point, PointBounds, Vector};
use adventutil::intcode::{Intcode, Outcome};
use adventutil::{Input, unit_dijkstra_length};
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Debug, Eq, PartialEq)]
struct Map {
    known: HashMap<Point, Tile>,
    droid_pos: Point,
    droid: Droid,
}

impl Map {
    fn new(program: Intcode) -> Map {
        Map {
            known: HashMap::from([(Point::ORIGIN, Tile::Empty)]),
            droid_pos: Point::ORIGIN,
            droid: Droid(program),
        }
    }

    fn to_explore(&self) -> Option<ExplorePoint> {
        let mut points = VecDeque::from([ExplorePoint {
            path: Vec::new(),
            final_pos: self.droid_pos,
            penult_pos: None,
        }]);
        while let Some(exp) = points.pop_front() {
            if !self.known.contains_key(&exp.final_pos) {
                return Some(exp);
            }
            for dir in [Vector::NORTH, Vector::SOUTH, Vector::EAST, Vector::WEST] {
                let p2 = exp.final_pos + dir;
                if self.known.get(&p2).is_none_or(|&t| t != Tile::Wall) {
                    let mut path2 = exp.path.clone();
                    path2.push(dir);
                    points.push_back(ExplorePoint {
                        path: path2,
                        final_pos: p2,
                        penult_pos: Some(exp.final_pos),
                    });
                }
            }
        }
        None
    }

    fn explore(&mut self) -> bool {
        let Some(exp) = self.to_explore() else {
            return false;
        };
        match self.droid.explore(exp.path) {
            Tile::Wall => {
                self.known.insert(exp.final_pos, Tile::Wall);
                self.droid_pos = exp.penult_pos.unwrap();
            }
            t => {
                self.known.insert(exp.final_pos, t);
                self.droid_pos = exp.final_pos;
            }
        }
        true
    }

    // DEBUG
    fn draw(&self) {
        let bounds = PointBounds::for_points(self.known.keys().copied()).unwrap();
        for y in bounds.min_y..=bounds.max_y {
            for x in bounds.min_x..=bounds.max_x {
                let ch = if (x, y) == (0, 0) {
                    'S'
                } else if (Point { x, y }) == self.droid_pos {
                    'D'
                } else {
                    match self.known.get(&Point { x, y }) {
                        None => ' ',
                        Some(Tile::Wall) => '#',
                        Some(Tile::Empty) => '.',
                        Some(Tile::Oxygen) => 'O',
                    }
                };
                print!("{ch}");
            }
            println!();
        }
    }

    fn shortest_dist(&self) -> u32 {
        unit_dijkstra_length(
            Point::ORIGIN,
            |p| self.known[p] == Tile::Oxygen,
            |&p| {
                [Vector::NORTH, Vector::SOUTH, Vector::EAST, Vector::WEST]
                    .into_iter()
                    .map(move |d| p + d)
                    .filter(|p2| self.known.get(p2).is_some_and(|&t| t != Tile::Wall))
            },
        )
        .unwrap()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Droid(Intcode);

impl Droid {
    fn explore(&mut self, motions: Vec<Vector>) -> Tile {
        let n = motions.len();
        assert!(n > 0);
        for &d in &motions[..(n - 1)] {
            assert_eq!(self.domove(d), Tile::Empty);
        }
        self.domove(motions[n - 1])
    }

    fn domove(&mut self, dir: Vector) -> Tile {
        assert_eq!(self.0.run_sans_io().unwrap(), Outcome::AwaitingInput);
        let inp = match dir {
            Vector::NORTH => 1,
            Vector::SOUTH => 2,
            Vector::WEST => 3,
            Vector::EAST => 4,
            _ => unreachable!(),
        };
        self.0.provide(inp).unwrap();
        let Outcome::Output(value) = self.0.run_sans_io().unwrap() else {
            panic!("Droid did not output value in response to input");
        };
        match value {
            0 => Tile::Wall,
            1 => Tile::Empty,
            2 => Tile::Oxygen,
            _ => panic!("Unexpected output value {value:?}"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Wall,
    Empty,
    Oxygen,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ExplorePoint {
    path: Vec<Vector>,
    final_pos: Point,
    penult_pos: Option<Point>,
}

fn solve(input: Input) -> u32 {
    let program = input.parse::<Intcode>();
    let mut map = Map::new(program);
    let mut i: usize = 0; // DEBUG
    while map.explore() {
        // BEGIN DEBUG
        i += 1;
        if i == 1000 || (i > 1000 && (i - 1000).is_multiple_of(50)) {
            map.draw();
            println!();
        }
        // END DEBUG
    }
    map.draw(); // DEBUG
    map.shortest_dist()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

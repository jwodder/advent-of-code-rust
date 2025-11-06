use adventutil::Input;
use adventutil::gridgeom::{Point, Vector};
use adventutil::intcode::{Intcode, Outcome};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, Eq, PartialEq)]
struct Map {
    known: HashMap<Point, Tile>,
    droid_pos: Point,
    droid_dir: Vector,
    droid: Droid,
}

impl Map {
    fn new(program: Intcode) -> Map {
        Map {
            known: HashMap::from([(Point::ORIGIN, Tile::Empty)]),
            droid_pos: Point::ORIGIN,
            droid_dir: Vector::NORTH,
            droid: Droid(program),
        }
    }

    fn explore(&mut self) {
        'a: loop {
            for d in [
                self.droid_dir.turn_left(),
                self.droid_dir,
                self.droid_dir.turn_right(),
                -self.droid_dir,
            ] {
                let p2 = self.droid_pos + d;
                if self.known.get(&p2).is_none_or(|&t| t != Tile::Wall) {
                    let t = self.droid.domove(d);
                    let prev = self.known.insert(p2, t);
                    assert!(prev.is_none_or(|p| p == t));
                    if t != Tile::Wall {
                        self.droid_pos = p2;
                        self.droid_dir = d;
                        if p2 == Point::ORIGIN {
                            return;
                        }
                        continue 'a;
                    }
                }
            }
            panic!("I'm trapped!");
        }
    }

    fn fill_time(&self) -> u32 {
        let start = self
            .known
            .iter()
            .find_map(|(&p, &t)| (t == Tile::Oxygen).then_some(p))
            .unwrap();
        let mut oxygenated = HashSet::from([start]);
        let mut nodes = vec![start];
        let mut time = 0;
        while !nodes.is_empty() {
            let mut nodes2 = Vec::new();
            for current in nodes {
                for p in [Vector::NORTH, Vector::SOUTH, Vector::EAST, Vector::WEST]
                    .into_iter()
                    .map(|d| current + d)
                    .filter(|p2| self.known.get(p2).is_some_and(|&t| t != Tile::Wall))
                {
                    if oxygenated.insert(p) {
                        nodes2.push(p);
                    }
                }
            }
            nodes = nodes2;
            time += 1;
        }
        time - 1
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Droid(Intcode);

impl Droid {
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

fn solve(input: Input) -> u32 {
    let program = input.parse::<Intcode>();
    let mut map = Map::new(program);
    map.explore();
    map.fill_time()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

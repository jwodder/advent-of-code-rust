use adventutil::grid::{Coords, Direction, Grid, GridBounds, Padding};
use adventutil::pullparser::{PullParser, Token};
use adventutil::{Input, dijkstra_length};

#[derive(Clone, Debug, Eq, PartialEq)]
struct Map {
    erosions: Grid<usize>,
    depth: usize,
    target: Coords,
}

impl Map {
    fn new(depth: usize, target: Coords) -> Map {
        let mut map = Map {
            erosions: Grid::filled(
                GridBounds {
                    height: target.y + 1,
                    width: target.x + 1,
                },
                0,
            ),
            depth,
            target,
        };
        for c in map.erosions.iter_coords() {
            map.calculate_erosion(c);
        }
        map
    }

    fn calculate_erosion(&mut self, c: Coords) {
        let geologic = if c == self.target {
            0
        } else {
            match (c.x, c.y) {
                (0, 0) => 0,
                (x, 0) => x * 16807,
                (0, y) => y * 48271,
                (x, y) => self.erosions[(y, x - 1)] * self.erosions[(y - 1, x)],
            }
        };
        self.erosions[c] = (geologic + self.depth) % 20183;
    }

    fn get(&mut self, c: Coords) -> Tile {
        if !self.erosions.bounds().contains(c) {
            let old_width = self.erosions.width();
            let old_height = self.erosions.height();
            let padding = Padding {
                left: 0,
                top: 0,
                right: (c.x + 1).saturating_sub(old_width),
                bottom: (c.y + 1).saturating_sub(old_height),
            };
            self.erosions = self.erosions.embiggened(padding, 0);
            for x in old_width..(self.erosions.width()) {
                for y in 0..old_height {
                    self.calculate_erosion(Coords { y, x });
                }
            }
            for y in old_height..(self.erosions.height()) {
                for x in 0..old_width {
                    self.calculate_erosion(Coords { y, x });
                }
            }
            for x in old_width..(self.erosions.width()) {
                for y in old_height..(self.erosions.height()) {
                    self.calculate_erosion(Coords { y, x });
                }
            }
        }
        match self.erosions[c] % 3 {
            0 => Tile::Rocky,
            1 => Tile::Wet,
            2 => Tile::Narrow,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Node {
    pos: Coords,
    tile: Tile,
    tool: Tool,
}

impl Node {
    fn motions(&self, map: &mut Map) -> Vec<(Node, u32)> {
        let mut output = Vec::with_capacity(5);
        let alt_tool = [Tool::Torch, Tool::ClimbingGear, Tool::Neither]
            .into_iter()
            .find(|&t| self.tile.allows(t) && t != self.tool)
            .unwrap();
        if self.pos == map.target && self.tool != Tool::Torch {
            output.push({
                let mut n = *self;
                n.tool = Tool::Torch;
                (n, 7)
            });
        }
        for d in Direction::cardinals() {
            let Some(c2) = self.pos.domove(d) else {
                continue;
            };
            let t2 = map.get(c2);
            if t2.allows(self.tool) {
                output.push({
                    let mut n = *self;
                    n.pos = c2;
                    n.tile = t2;
                    (n, 1)
                });
            } else if t2.allows(alt_tool) {
                output.push({
                    let mut n = *self;
                    n.pos = c2;
                    n.tile = t2;
                    n.tool = alt_tool;
                    (n, 8)
                });
            }
        }
        output
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Tile {
    Rocky,
    Wet,
    Narrow,
}

impl Tile {
    fn allows(self, tool: Tool) -> bool {
        match self {
            Tile::Rocky => matches!(tool, Tool::Torch | Tool::ClimbingGear),
            Tile::Wet => matches!(tool, Tool::ClimbingGear | Tool::Neither),
            Tile::Narrow => matches!(tool, Tool::Torch | Tool::Neither),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Tool {
    Torch,
    ClimbingGear,
    Neither,
}

fn solve(input: Input) -> u32 {
    let s = input.read();
    let mut parser = PullParser::new(s.trim());
    parser.skip("depth: ").unwrap();
    let depth = parser.parse_to::<usize, _>(Token::Newline).unwrap();
    parser.skip("target: ").unwrap();
    let target_x = parser.parse_to::<usize, _>(',').unwrap();
    let target_y = parser.parse_to::<usize, _>(Token::Eof).unwrap();
    let target = Coords {
        y: target_y,
        x: target_x,
    };
    let mut map = Map::new(depth, target);
    dijkstra_length(
        Node {
            pos: Coords { y: 0, x: 0 },
            tile: Tile::Rocky,
            tool: Tool::Torch,
        },
        |n| n.pos == target && n.tool == Tool::Torch,
        |n| n.motions(&mut map),
    )
    .unwrap()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from("depth: 510\ntarget: 10,10\n");
        assert_eq!(solve(input), 45);
    }
}

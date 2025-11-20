use adventutil::counter::Counter;
use adventutil::grid::{Grid, GridFromError};
use adventutil::{Input, unordered_pairs};
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Tile {
    id: u64,
    sides: HashSet<Vec<bool>>,
}

impl Tile {
    fn matches(&self, other: &Tile) -> bool {
        !self.sides.is_disjoint(&other.sides)
    }
}

impl std::str::FromStr for Tile {
    type Err = GridFromError;

    fn from_str(s: &str) -> Result<Tile, GridFromError> {
        let (line1, rest) = s.split_once('\n').unwrap();
        let id = line1
            .trim()
            .strip_prefix("Tile ")
            .unwrap()
            .strip_suffix(':')
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let grid = Grid::from_drawing(rest)?;

        let top = (0..grid.width()).map(|x| grid[(0, x)]).collect::<Vec<_>>();
        let mut top_rev = top.clone();
        top_rev.reverse();

        let bottom = (0..grid.width())
            .map(|x| grid[(grid.height() - 1, x)])
            .collect::<Vec<_>>();
        let mut bottom_rev = bottom.clone();
        bottom_rev.reverse();

        let left = (0..grid.height()).map(|y| grid[(y, 0)]).collect::<Vec<_>>();
        let mut left_rev = left.clone();
        left_rev.reverse();

        let right = (0..grid.height())
            .map(|y| grid[(y, grid.width() - 1)])
            .collect::<Vec<_>>();
        let mut right_rev = right.clone();
        right_rev.reverse();

        Ok(Tile {
            id,
            sides: HashSet::from([
                top, top_rev, bottom, bottom_rev, left, left_rev, right, right_rev,
            ]),
        })
    }
}

fn solve(input: Input) -> u64 {
    let tiles = input
        .paragraphs()
        .map(|p| p.parse::<Tile>().unwrap())
        .collect::<Vec<_>>();
    let mut connections = Counter::new();
    for (t1, t2) in unordered_pairs(&tiles) {
        if t1.matches(t2) {
            connections.add(t1.id);
            connections.add(t2.id);
        }
    }
    connections
        .into_iter()
        .filter_map(|(id, conns)| (conns == 2).then_some(id))
        .product()
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
            "Tile 2311:\n",
            "..##.#..#.\n",
            "##..#.....\n",
            "#...##..#.\n",
            "####.#...#\n",
            "##.##.###.\n",
            "##...#.###\n",
            ".#.#.#..##\n",
            "..#....#..\n",
            "###...#.#.\n",
            "..###..###\n",
            "\n",
            "Tile 1951:\n",
            "#.##...##.\n",
            "#.####...#\n",
            ".....#..##\n",
            "#...######\n",
            ".##.#....#\n",
            ".###.#####\n",
            "###.##.##.\n",
            ".###....#.\n",
            "..#.#..#.#\n",
            "#...##.#..\n",
            "\n",
            "Tile 1171:\n",
            "####...##.\n",
            "#..##.#..#\n",
            "##.#..#.#.\n",
            ".###.####.\n",
            "..###.####\n",
            ".##....##.\n",
            ".#...####.\n",
            "#.##.####.\n",
            "####..#...\n",
            ".....##...\n",
            "\n",
            "Tile 1427:\n",
            "###.##.#..\n",
            ".#..#.##..\n",
            ".#.##.#..#\n",
            "#.#.#.##.#\n",
            "....#...##\n",
            "...##..##.\n",
            "...#.#####\n",
            ".#.####.#.\n",
            "..#..###.#\n",
            "..##.#..#.\n",
            "\n",
            "Tile 1489:\n",
            "##.#.#....\n",
            "..##...#..\n",
            ".##..##...\n",
            "..#...#...\n",
            "#####...#.\n",
            "#..#.#.#.#\n",
            "...#.#.#..\n",
            "##.#...##.\n",
            "..##.##.##\n",
            "###.##.#..\n",
            "\n",
            "Tile 2473:\n",
            "#....####.\n",
            "#..#.##...\n",
            "#.##..#...\n",
            "######.#.#\n",
            ".#...#.#.#\n",
            ".#########\n",
            ".###.#..#.\n",
            "########.#\n",
            "##...##.#.\n",
            "..###.#.#.\n",
            "\n",
            "Tile 2971:\n",
            "..#.#....#\n",
            "#...###...\n",
            "#.#.###...\n",
            "##.##..#..\n",
            ".#####..##\n",
            ".#..####.#\n",
            "#..#.#..#.\n",
            "..####.###\n",
            "..#.#.###.\n",
            "...#.#.#.#\n",
            "\n",
            "Tile 2729:\n",
            "...#.#.#.#\n",
            "####.#....\n",
            "..#.#.....\n",
            "....#..#.#\n",
            ".##..##.#.\n",
            ".#.####...\n",
            "####.#.#..\n",
            "##.####...\n",
            "##..#.##..\n",
            "#.##...##.\n",
            "\n",
            "Tile 3079:\n",
            "#.#.#####.\n",
            ".#..######\n",
            "..#.......\n",
            "######....\n",
            "####.#..#.\n",
            ".#...#.##.\n",
            "#.#####.##\n",
            "..#.###...\n",
            "..#.......\n",
            "..#.###...\n",
        ));
        assert_eq!(solve(input), 20899048083289);
    }
}

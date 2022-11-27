use adventutil::grid::Grid;
use adventutil::{parse_csv, Input};
use std::collections::HashSet;

#[derive(Debug)]
struct Board {
    grid: Grid<u32>,
    marked: HashSet<(usize, usize)>,
}

impl Board {
    fn new(grid: Grid<u32>) -> Board {
        Board {
            grid,
            marked: HashSet::new(),
        }
    }

    fn mark(&mut self, value: u32) {
        self.grid
            .enumerate()
            .filter_map(|(coord, &v)| (v == value).then_some(coord))
            .for_each(|coord| {
                self.marked.insert(coord);
            })
    }

    fn wins(&self) -> bool {
        (0..self.grid.height())
            .any(|y| (0..self.grid.width()).all(|x| self.marked.contains(&(y, x))))
            || (0..self.grid.width())
                .any(|x| (0..self.grid.height()).all(|y| self.marked.contains(&(y, x))))
    }

    fn score(&self) -> u32 {
        (0..self.grid.height())
            .map(|y| {
                (0..self.grid.width())
                    .filter_map(|x| {
                        (!self.marked.contains(&(y, x))).then(|| self.grid.get(y, x).unwrap())
                    })
                    .sum::<u32>()
            })
            .sum()
    }
}

struct Bingo {
    numbers: Vec<u32>,
    boards: Vec<Board>,
}

impl Bingo {
    fn run(mut self) -> u32 {
        for n in self.numbers {
            for board in &mut self.boards {
                board.mark(n);
                if board.wins() {
                    return board.score() * n;
                }
            }
        }
        panic!("No one won");
    }
}

fn main() {
    let mut paras = Input::from_env().paragraphs();
    let numbers = parse_csv::<u32>(&paras.next().unwrap());
    let boards = paras
        .map(|s| Board::new(Grid::<u32>::parse_words(&s).expect("Error parsing input")))
        .collect::<Vec<_>>();
    let game = Bingo { numbers, boards };
    println!("{}", game.run());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let numbers = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        let boards = vec![
            Board::new(
                Grid::<u32>::parse_words(concat!(
                    "22 13 17 11  0\n",
                    " 8  2 23  4 24\n",
                    "21  9 14 16  7\n",
                    " 6 10  3 18  5\n",
                    " 1 12 20 15 19\n",
                ))
                .unwrap(),
            ),
            Board::new(
                Grid::<u32>::parse_words(concat!(
                    " 3 15  0  2 22\n",
                    " 9 18 13 17  5\n",
                    "19  8  7 25 23\n",
                    "20 11 10 24  4\n",
                    "14 21 16 12  6\n",
                ))
                .unwrap(),
            ),
            Board::new(
                Grid::<u32>::parse_words(concat!(
                    "14 21 17 24  4\n",
                    "10 16 15  9 19\n",
                    "18  8 23 26 20\n",
                    "22 11 13  6  5\n",
                    " 2  0 12  3  7\n",
                ))
                .unwrap(),
            ),
        ];
        let game = Bingo { numbers, boards };
        assert_eq!(game.run(), 4512);
    }
}

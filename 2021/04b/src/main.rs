use adventutil::grid::{Coords, Grid};
use adventutil::{parse_csv, Input};
use std::collections::HashSet;

#[derive(Debug)]
struct Board {
    grid: Grid<u32>,
    marked: HashSet<Coords>,
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
            .iter()
            .filter_map(|(coord, &v)| (v == value).then_some(coord))
            .for_each(|coord| {
                self.marked.insert(coord);
            });
    }

    fn wins(&self) -> bool {
        (0..self.grid.height())
            .any(|y| (0..self.grid.width()).all(|x| self.marked.contains(&Coords::new(y, x))))
            || (0..self.grid.width())
                .any(|x| (0..self.grid.height()).all(|y| self.marked.contains(&Coords::new(y, x))))
    }

    fn score(&self) -> u32 {
        self.grid
            .iter()
            .filter_map(|(coords, &v)| (!self.marked.contains(&coords)).then_some(v))
            .sum()
    }
}

struct Bingo {
    numbers: Vec<u32>,
    boards: Vec<Board>,
    winners: HashSet<usize>,
}

impl Bingo {
    fn new(numbers: Vec<u32>, boards: Vec<Board>) -> Bingo {
        Bingo {
            numbers,
            boards,
            winners: HashSet::new(),
        }
    }

    fn run(mut self) -> u32 {
        let qty = self.boards.len();
        for n in self.numbers {
            for (i, board) in self.boards.iter_mut().enumerate() {
                board.mark(n);
                if !self.winners.contains(&i) && board.wins() {
                    self.winners.insert(i);
                    if self.winners.len() == qty {
                        return board.score() * n;
                    }
                }
            }
        }
        panic!("No final winner");
    }
}

fn solve(input: Input) -> u32 {
    let mut paras = input.paragraphs();
    let numbers = parse_csv::<u32>(&paras.next().unwrap());
    let boards = paras
        .map(|s| Board::new(Grid::<u32>::parse_words(&s).expect("Error parsing input")))
        .collect::<Vec<_>>();
    let game = Bingo::new(numbers, boards);
    game.run()
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
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n",
            "\n",
            "22 13 17 11  0\n",
            " 8  2 23  4 24\n",
            "21  9 14 16  7\n",
            " 6 10  3 18  5\n",
            " 1 12 20 15 19\n",
            "\n",
            " 3 15  0  2 22\n",
            " 9 18 13 17  5\n",
            "19  8  7 25 23\n",
            "20 11 10 24  4\n",
            "14 21 16 12  6\n",
            "\n",
            "14 21 17 24  4\n",
            "10 16 15  9 19\n",
            "18  8 23 26 20\n",
            "22 11 13  6  5\n",
            " 2  0 12  3  7\n",
        ));
        assert_eq!(solve(input), 1924);
    }
}

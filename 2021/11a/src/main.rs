use adventutil::grid::{Grid, ParseGridError};
use adventutil::Input;
use std::collections::VecDeque;
use std::iter::repeat_with;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Octopuses(Grid<u32>);

impl Octopuses {
    fn step(&mut self) -> usize {
        let mut flashed = 0;
        let mut queue = VecDeque::new();
        for coord in self.0.iter_coords() {
            self.0[coord] += 1;
            if self.0[coord] == 10 {
                flashed += 1;
                queue.push_back(coord);
            }
        }
        while let Some(coord) = queue.pop_front() {
            for c in self.0.adjacent_coords(coord) {
                self.0[c] += 1;
                if self.0[c] == 10 {
                    flashed += 1;
                    queue.push_back(c);
                }
            }
        }
        for coord in self.0.iter_coords() {
            if self.0[coord] > 9 {
                self.0[coord] = 0;
            }
        }
        flashed
    }

    fn flashes_over_steps(&mut self, steps: usize) -> usize {
        repeat_with(|| self.step()).take(steps).sum()
    }
}

impl FromStr for Octopuses {
    type Err = ParseGridError<ParseIntError>;

    fn from_str(s: &str) -> Result<Octopuses, Self::Err> {
        s.parse::<Grid<u32>>().map(Octopuses)
    }
}

fn solve(input: Input, steps: usize) -> usize {
    input.parse::<Octopuses>().flashes_over_steps(steps)
}

fn main() {
    println!("{}", solve(Input::from_env(), 100));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let mut octos = "11111\n19991\n19191\n19991\n11111\n"
            .parse::<Octopuses>()
            .unwrap();
        assert_eq!(octos.step(), 9);
        assert_eq!(
            octos,
            "34543\n40004\n50005\n40004\n34543\n"
                .parse::<Octopuses>()
                .unwrap()
        );
        assert_eq!(octos.step(), 0);
        assert_eq!(
            octos,
            "45654\n51115\n61116\n51115\n45654\n"
                .parse::<Octopuses>()
                .unwrap()
        );
    }

    #[test]
    fn test_example2_steps() {
        let mut octos = concat!(
            "5483143223\n",
            "2745854711\n",
            "5264556173\n",
            "6141336146\n",
            "6357385478\n",
            "4167524645\n",
            "2176841721\n",
            "6882881134\n",
            "4846848554\n",
            "5283751526\n",
        )
        .parse::<Octopuses>()
        .unwrap();
        octos.step();
        assert_eq!(
            octos,
            concat!(
                "6594254334\n",
                "3856965822\n",
                "6375667284\n",
                "7252447257\n",
                "7468496589\n",
                "5278635756\n",
                "3287952832\n",
                "7993992245\n",
                "5957959665\n",
                "6394862637\n",
            )
            .parse::<Octopuses>()
            .unwrap()
        );
        octos.step();
        assert_eq!(
            octos,
            concat!(
                "8807476555\n",
                "5089087054\n",
                "8597889608\n",
                "8485769600\n",
                "8700908800\n",
                "6600088989\n",
                "6800005943\n",
                "0000007456\n",
                "9000000876\n",
                "8700006848\n",
            )
            .parse::<Octopuses>()
            .unwrap()
        );
        octos.step();
        assert_eq!(
            octos,
            concat!(
                "0050900866\n",
                "8500800575\n",
                "9900000039\n",
                "9700000041\n",
                "9935080063\n",
                "7712300000\n",
                "7911250009\n",
                "2211130000\n",
                "0421125000\n",
                "0021119000\n",
            )
            .parse::<Octopuses>()
            .unwrap()
        );
        octos.step();
        assert_eq!(
            octos,
            concat!(
                "2263031977\n",
                "0923031697\n",
                "0032221150\n",
                "0041111163\n",
                "0076191174\n",
                "0053411122\n",
                "0042361120\n",
                "5532241122\n",
                "1532247211\n",
                "1132230211\n",
            )
            .parse::<Octopuses>()
            .unwrap()
        );
        octos.step();
        assert_eq!(
            octos,
            concat!(
                "4484144000\n",
                "2044144000\n",
                "2253333493\n",
                "1152333274\n",
                "1187303285\n",
                "1164633233\n",
                "1153472231\n",
                "6643352233\n",
                "2643358322\n",
                "2243341322\n",
            )
            .parse::<Octopuses>()
            .unwrap()
        );
        octos.step();
        assert_eq!(
            octos,
            concat!(
                "5595255111\n",
                "3155255222\n",
                "3364444605\n",
                "2263444496\n",
                "2298414396\n",
                "2275744344\n",
                "2264583342\n",
                "7754463344\n",
                "3754469433\n",
                "3354452433\n",
            )
            .parse::<Octopuses>()
            .unwrap()
        );
        octos.step();
        assert_eq!(
            octos,
            concat!(
                "6707366222\n",
                "4377366333\n",
                "4475555827\n",
                "3496655709\n",
                "3500625609\n",
                "3509955566\n",
                "3486694453\n",
                "8865585555\n",
                "4865580644\n",
                "4465574644\n",
            )
            .parse::<Octopuses>()
            .unwrap()
        );
        octos.step();
        assert_eq!(
            octos,
            concat!(
                "7818477333\n",
                "5488477444\n",
                "5697666949\n",
                "4608766830\n",
                "4734946730\n",
                "4740097688\n",
                "6900007564\n",
                "0000009666\n",
                "8000004755\n",
                "6800007755\n",
            )
            .parse::<Octopuses>()
            .unwrap()
        );
        octos.step();
        assert_eq!(
            octos,
            concat!(
                "9060000644\n",
                "7800000976\n",
                "6900000080\n",
                "5840000082\n",
                "5858000093\n",
                "6962400000\n",
                "8021250009\n",
                "2221130009\n",
                "9111128097\n",
                "7911119976\n",
            )
            .parse::<Octopuses>()
            .unwrap()
        );
        octos.step();
        assert_eq!(
            octos,
            concat!(
                "0481112976\n",
                "0031112009\n",
                "0041112504\n",
                "0081111406\n",
                "0099111306\n",
                "0093511233\n",
                "0442361130\n",
                "5532252350\n",
                "0532250600\n",
                "0032240000\n",
            )
            .parse::<Octopuses>()
            .unwrap()
        );
        octos.step();
    }

    #[test]
    fn test_example2_flashes_over_10_steps() {
        let input = Input::from(concat!(
            "5483143223\n",
            "2745854711\n",
            "5264556173\n",
            "6141336146\n",
            "6357385478\n",
            "4167524645\n",
            "2176841721\n",
            "6882881134\n",
            "4846848554\n",
            "5283751526\n",
        ));
        assert_eq!(solve(input, 10), 204);
    }

    #[test]
    fn test_example2_flashes_over_100_steps() {
        let input = Input::from(concat!(
            "5483143223\n",
            "2745854711\n",
            "5264556173\n",
            "6141336146\n",
            "6357385478\n",
            "4167524645\n",
            "2176841721\n",
            "6882881134\n",
            "4846848554\n",
            "5283751526\n",
        ));
        assert_eq!(solve(input, 100), 1656);
    }
}

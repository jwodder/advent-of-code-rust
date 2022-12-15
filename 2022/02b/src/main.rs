use adventutil::Input;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

impl Rps {
    fn value(&self) -> u32 {
        match self {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Scissors => 3,
        }
    }

    fn from_col1(s: &str) -> Rps {
        match s {
            "A" => Rps::Rock,
            "B" => Rps::Paper,
            "C" => Rps::Scissors,
            s => panic!("Invalid column 1 value: {s:?}"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }

    fn from_col2(s: &str) -> Outcome {
        match s {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            s => panic!("Invalid column 2 value: {s:?}"),
        }
    }

    fn needed_move(&self, other: Rps) -> Rps {
        use Outcome::*;
        use Rps::*;
        match (self, other) {
            (Win, Rock) => Paper,
            (Win, Paper) => Scissors,
            (Win, Scissors) => Rock,
            (Lose, Rock) => Scissors,
            (Lose, Paper) => Rock,
            (Lose, Scissors) => Paper,
            (Draw, rps) => rps,
        }
    }
}

fn solve(input: Input) -> u32 {
    input
        .lines()
        .map(|s| {
            let (them, outcome) = parse_round(&s);
            outcome.needed_move(them).value() + outcome.score()
        })
        .sum()
}

fn parse_round(s: &str) -> (Rps, Outcome) {
    let (col1, col2) = s.split_once(' ').unwrap();
    (Rps::from_col1(col1), Outcome::from_col2(col2))
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from("A Y\nB X\nC Z\n");
        assert_eq!(solve(input), 12);
    }
}

use adventutil::Input;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

impl Rps {
    fn score(&self, other: Rps) -> u32 {
        use Rps::*;
        match (self, other) {
            (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => 6,
            (Scissors, Rock) | (Paper, Scissors) | (Rock, Paper) => 0,
            _ => 3,
        }
    }

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

    fn from_col2(s: &str) -> Rps {
        match s {
            "X" => Rps::Rock,
            "Y" => Rps::Paper,
            "Z" => Rps::Scissors,
            s => panic!("Invalid column 2 value: {s:?}"),
        }
    }
}

fn solve(input: Input) -> u32 {
    input
        .lines()
        .map(|s| {
            let (them, me) = parse_round(&s);
            me.score(them) + me.value()
        })
        .sum()
}

fn parse_round(s: &str) -> (Rps, Rps) {
    let (col1, col2) = s.split_once(' ').unwrap();
    (Rps::from_col1(col1), Rps::from_col2(col2))
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from("A Y\nB X\nC Z\n");
        assert_eq!(solve(input), 15);
    }
}

use adventutil::Input;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn score(&self, other: RPS) -> u32 {
        use RPS::*;
        match (self, other) {
            (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => 6,
            (Scissors, Rock) | (Paper, Scissors) | (Rock, Paper) => 0,
            _ => 3,
        }
    }

    fn value(&self) -> u32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }

    fn from_col1(s: &str) -> RPS {
        match s {
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            "C" => RPS::Scissors,
            s => panic!("Invalid column 1 value: {s:?}"),
        }
    }

    fn from_col2(s: &str) -> RPS {
        match s {
            "X" => RPS::Rock,
            "Y" => RPS::Paper,
            "Z" => RPS::Scissors,
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

fn parse_round(s: &str) -> (RPS, RPS) {
    let (col1, col2) = s.split_once(' ').unwrap();
    (RPS::from_col1(col1), RPS::from_col2(col2))
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
        assert_eq!(solve(input), 15);
    }
}

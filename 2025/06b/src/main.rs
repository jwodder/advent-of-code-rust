use adventutil::Input;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Operation {
    Add,
    Mul,
}

impl Operation {
    fn eval(self, left: u64, right: u64) -> u64 {
        match self {
            Operation::Add => left + right,
            Operation::Mul => left * right,
        }
    }
}

fn solve(input: Input) -> u64 {
    let grid = input
        .lines()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let width = grid.iter().map(Vec::len).max().unwrap();
    let mut total = 0;
    let mut op = Operation::Add;
    let mut accum = None;
    for i in 0..width {
        let mut chrs = (0..grid.len())
            .filter_map(|j| grid[j].get(i))
            .collect::<Vec<_>>();
        if chrs.iter().all(|&c| c.is_ascii_whitespace()) {
            if let Some(ac) = accum {
                total += ac;
            }
            accum = None;
        } else {
            let prev = accum.unwrap_or_else(|| {
                let (op2, e) = match chrs.pop().unwrap() {
                    '+' => (Operation::Add, 0),
                    '*' => (Operation::Mul, 1),
                    c => panic!("Unexpected operation {c:?}"),
                };
                op = op2;
                e
            });
            let n = String::from_iter(chrs).trim().parse::<u64>().unwrap();
            accum = Some(op.eval(prev, n));
        }
    }
    if let Some(ac) = accum {
        total += ac;
    }
    total
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
            "123 328  51 64\n",
            " 45 64  387 23\n",
            "  6 98  215 314\n",
            "*   +   *   +\n",
        ));
        assert_eq!(solve(input), 3263827);
    }
}

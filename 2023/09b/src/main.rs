use adventutil::Input;

fn extrapolate(values: Vec<i32>) -> i32 {
    let mut stack = vec![values];
    loop {
        let latest = stack.last().unwrap();
        let diffs = (0..(latest.len() - 1))
            .map(|i| latest[i + 1] - latest[i])
            .collect::<Vec<_>>();
        if diffs.iter().all(|&n| n == 0) {
            break;
        }
        stack.push(diffs);
    }
    let mut prev_val = 0;
    while let Some(diffs) = stack.pop() {
        prev_val = diffs[0] - prev_val;
    }
    prev_val
}

fn solve(input: Input) -> i32 {
    input
        .lines()
        .map(|ln| {
            extrapolate(
                ln.split_whitespace()
                    .map(|w| w.parse::<i32>().unwrap())
                    .collect(),
            )
        })
        .sum()
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
            "0 3 6 9 12 15\n",
            "1 3 6 10 15 21\n",
            "10 13 16 21 30 45\n",
        ));
        assert_eq!(solve(input), 2);
    }
}

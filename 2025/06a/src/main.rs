use adventutil::Input;

fn solve(input: Input) -> u64 {
    let grid = input
        .lines()
        .map(|s| {
            s.split_ascii_whitespace()
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut accum = 0;
    for i in 0..grid[0].len() {
        let nums = (0..(grid.len() - 1)).map(|j| grid[j][i].parse::<u64>().unwrap());
        match grid.last().unwrap()[i].as_str() {
            "+" => accum += nums.sum::<u64>(),
            "*" => accum += nums.product::<u64>(),
            s => panic!("Unexpected operation {s:?}"),
        }
    }
    accum
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
        assert_eq!(solve(input), 4277556);
    }
}

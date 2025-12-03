use adventutil::Input;

fn max_joltage(joltages: Vec<u64>) -> u64 {
    // `tbl[i][j]` will contain the largest joltage made from `j` digits from
    // index `i` onwards
    let mut tbl = vec![vec![0; 13]; joltages.len() + 1];
    for i in (0..joltages.len()).rev() {
        for length in 1..=12 {
            if i + length > joltages.len() {
                break;
            }
            tbl[i][length] = std::cmp::max(
                joltages[i] * std::iter::repeat_n(10, length - 1).product::<u64>()
                    + tbl[i + 1][length - 1],
                tbl[i + 1][length],
            );
        }
    }
    tbl[0][12]
}

fn solve(input: Input) -> u64 {
    input
        .lines()
        .map(|ln| {
            let joltages = ln
                .chars()
                .map(|c| u64::from(c.to_digit(10).unwrap()))
                .collect::<Vec<_>>();
            max_joltage(joltages)
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
            "987654321111111\n",
            "811111111111119\n",
            "234234234234278\n",
            "818181911112111\n",
        ));
        assert_eq!(solve(input), 3121910778619);
    }
}

use adventutil::{Input, unordered_pairs};

fn solve(input: Input) -> u32 {
    input
        .lines()
        .map(|ln| {
            let joltages = ln
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>();
            unordered_pairs(&joltages)
                .map(|(&a, &b)| a * 10 + b)
                .max()
                .unwrap()
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
        assert_eq!(solve(input), 357);
    }
}

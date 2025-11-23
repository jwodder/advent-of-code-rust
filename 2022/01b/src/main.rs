use adventutil::Input;
use adventutil::maxn::maxn;

fn solve(input: Input) -> usize {
    maxn(
        3,
        input.paragraphs().map(|s| {
            s.lines()
                .map(|t| t.parse::<usize>().unwrap())
                .sum::<usize>()
        }),
    )
    .into_iter()
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
            "1000\n", "2000\n", "3000\n", "\n", "4000\n", "\n", "5000\n", "6000\n", "\n", "7000\n",
            "8000\n", "9000\n", "\n", "10000\n",
        ));
        assert_eq!(solve(input), 45000);
    }
}

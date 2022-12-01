use adventutil::Input;

fn solve<I: IntoIterator<Item = String>>(iter: I) -> usize {
    let mut calories = iter
        .into_iter()
        .map(|s| {
            s.lines()
                .map(|t| t.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect::<Vec<_>>();
    calories.sort();
    calories[(calories.len() - 3)..].iter().sum()
}

fn main() {
    println!("{}", solve(Input::from_env().paragraphs()));
}

use adventutil::Input;

fn solve<I: IntoIterator<Item = String>>(iter: I) -> usize {
    iter.into_iter()
        .map(|s| {
            s.lines()
                .map(|t| t.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .max()
        .unwrap()
}

fn main() {
    println!("{}", solve(Input::from_env().paragraphs()));
}

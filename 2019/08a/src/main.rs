use adventutil::Input;
use adventutil::counter::Counter;

fn solve(input: Input) -> u64 {
    let digits = input
        .read()
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    digits
        .chunks(25 * 6)
        .map(|layer| {
            let counts = layer.iter().copied().collect::<Counter<_>>();
            (counts[&0], counts[&1] * counts[&2])
        })
        .min_by_key(|&(qty0, _)| qty0)
        .unwrap()
        .1
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

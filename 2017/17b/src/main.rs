use adventutil::Input;

fn solve(input: Input) -> usize {
    let offset = input.parse::<usize>();
    let mut i = 0;
    let mut after_zero = 0;
    for j in 1..=50_000_000 {
        i = (i + offset) % j + 1;
        if i == 1 {
            after_zero = j;
        }
    }
    after_zero
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

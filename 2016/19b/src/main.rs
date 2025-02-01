use adventutil::Input;

fn solve(input: Input) -> u32 {
    let n = input.parse::<u32>();
    let mut elves = (1..=n).collect::<Vec<_>>();
    let mut i = 0;
    while elves.len() != 1 {
        elves.remove((i + elves.len() / 2) % elves.len());
        if i < elves.len() {
            i += 1;
        } else {
            i = 0;
        }
    }
    elves.pop().unwrap()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from("5");
        assert_eq!(solve(input), 2);
    }
}

use adventutil::Input;

fn solve(input: Input) -> u32 {
    // <https://en.wikipedia.org/wiki/Josephus_problem#k_=_2>
    let n = input.parse::<u32>();
    let m = n.ilog2();
    (n - (1 << m)) * 2 + 1
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
        assert_eq!(solve(input), 3);
    }
}

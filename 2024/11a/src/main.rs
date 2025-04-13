use adventutil::Input;

fn solve(input: Input) -> usize {
    let s = input.read();
    let mut numbers = s
        .split_whitespace()
        .map(|w| w.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    for _ in 0..25 {
        let mut new_numbers = Vec::new();
        for n in numbers {
            if n == 0 {
                new_numbers.push(1);
            } else {
                let ns = n.to_string();
                if ns.len() % 2 == 0 {
                    let len = ns.len() / 2;
                    new_numbers.push(ns[..len].parse().unwrap());
                    new_numbers.push(ns[len..].parse().unwrap());
                } else {
                    new_numbers.push(n * 2024);
                }
            }
        }
        numbers = new_numbers;
    }
    numbers.len()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = Input::from("125 17");
        assert_eq!(solve(input), 55312);
    }
}

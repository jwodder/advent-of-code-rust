use adventutil::Input;
use itertools::Itertools;

fn solve(input: Input, target: usize) -> usize {
    let container_qtys = input
        .parse_lines::<usize>()
        .powerset()
        .filter(|comb| comb.iter().sum::<usize>() == target)
        .map(|comb| comb.len())
        .collect::<Vec<_>>();
    let &min_qty = container_qtys.iter().min().unwrap();
    container_qtys.into_iter().filter(|&i| i == min_qty).count()
}

fn main() {
    println!("{}", solve(Input::from_env(), 150));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from("20\n15\n10\n5\n5\n");
        assert_eq!(solve(input, 25), 3);
    }
}

use adventutil::Input;

fn solve(input: Input) -> usize {
    let offset = input.parse::<usize>();
    let mut buffer = vec![0];
    let mut i = 0;
    for j in 1..=2017 {
        i = (i + offset) % buffer.len() + 1;
        buffer.insert(i, j);
    }
    buffer[i + 1]
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from("3");
        assert_eq!(solve(input), 638);
    }
}

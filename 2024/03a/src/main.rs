use adventutil::Input;

fn solve(input: Input) -> u32 {
    let s = input.read();
    s.match_indices("mul(")
        .filter_map(|(i, _)| {
            let j = s[(i + 4)..].find(',')? + i + 4;
            let k = s[j..].find(')')? + j;
            let a = s[(i + 4)..j].parse::<u32>().ok()?;
            let b = s[(j + 1)..k].parse::<u32>().ok()?;
            Some(a * b)
        })
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
        let input =
            Input::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(solve(input), 161);
    }
}

use adventutil::Input;
use itertools::Itertools;

fn solve(input: Input) -> u32 {
    let s = input.read();
    let mut active = true;
    s.match_indices("mul(")
        .merge(s.match_indices("do()"))
        .merge(s.match_indices("don't()"))
        .filter_map(|(i, t)| match t {
            "do()" => {
                active = true;
                None
            }
            "don't()" => {
                active = false;
                None
            }
            "mul(" if active => {
                let j = s[(i + 4)..].find(',')? + i + 4;
                let k = s[j..].find(')')? + j;
                let a = s[(i + 4)..j].parse::<u32>().ok()?;
                let b = s[(j + 1)..k].parse::<u32>().ok()?;
                Some(a * b)
            }
            _ => None,
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
    fn test_example1() {
        let input = Input::from(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        );
        assert_eq!(solve(input), 48);
    }
}

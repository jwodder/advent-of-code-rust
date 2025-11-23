use adventutil::Input;

fn solve(input: Input) -> String {
    let prep_time = input.parse::<usize>();
    let mut recipes = vec![3, 7];
    let mut elf1 = 0;
    let mut elf2 = 1;
    while recipes.len() < prep_time + 10 {
        let r1 = recipes[elf1];
        let r2 = recipes[elf2];
        let mixture = r1 + r2;
        recipes.extend(digits(mixture));
        elf1 = (elf1 + r1 + 1) % recipes.len();
        elf2 = (elf2 + r2 + 1) % recipes.len();
    }
    recipes
        .into_iter()
        .skip(prep_time)
        .take(10)
        .map(|d| char::from_digit(u32::try_from(d).unwrap(), 10).unwrap())
        .collect()
}

fn digits(mut x: usize) -> Vec<usize> {
    if x == 0 {
        vec![0]
    } else {
        let mut ds = Vec::new();
        while x != 0 {
            ds.push(x % 10);
            x /= 10;
        }
        ds.reverse();
        ds
    }
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("9", "5158916779")]
    #[case("5", "0124515891")]
    #[case("18", "9251071085")]
    #[case("2018", "5941429882")]
    fn examples(#[case] num: &'static str, #[case] answer: &str) {
        let input = Input::from(num);
        assert_eq!(solve(input), answer);
    }
}

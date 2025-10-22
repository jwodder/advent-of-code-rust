use adventutil::Input;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Searcher {
    needle: Vec<usize>,
    offset: usize,
}

impl Searcher {
    fn find(&mut self, haystack: &[usize]) -> Option<usize> {
        'outer: loop {
            for i in 0..self.needle.len() {
                match haystack.get(self.offset + i).copied() {
                    Some(n) if n == self.needle[i] => (),
                    Some(_) => {
                        self.offset += 1;
                        continue 'outer;
                    }
                    None => return None,
                }
            }
            return Some(self.offset);
        }
    }
}

fn solve(input: Input) -> usize {
    let needle = input
        .read()
        .trim()
        .chars()
        .map(|c| usize::try_from(c.to_digit(32).unwrap()).unwrap())
        .collect::<Vec<_>>();
    let mut searcher = Searcher { needle, offset: 0 };
    let mut recipes = vec![3, 7];
    let mut elf1 = 0;
    let mut elf2 = 1;
    loop {
        if let Some(i) = searcher.find(&recipes) {
            return i;
        }
        let r1 = recipes[elf1];
        let r2 = recipes[elf2];
        let mixture = r1 + r2;
        recipes.extend(digits(mixture));
        elf1 = (elf1 + r1 + 1) % recipes.len();
        elf2 = (elf2 + r2 + 1) % recipes.len();
    }
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
    #[case("51589", 9)]
    #[case("01245", 5)]
    #[case("92510", 18)]
    #[case("59414", 2018)]
    fn example(#[case] num: &'static str, #[case] answer: usize) {
        let input = Input::from(num);
        assert_eq!(solve(input), answer);
    }
}

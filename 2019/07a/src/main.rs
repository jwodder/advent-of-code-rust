use adventutil::intcode::{Intcode, VecIO};
use adventutil::Input;
use itertools::Itertools;

fn solve(input: Input) -> i64 {
    let program = input.parse::<Intcode>();
    (0..5)
        .permutations(5)
        .map(|perm| {
            perm.into_iter().fold(0, |input, phase| {
                let mut io = VecIO::from([phase, input]);
                program.clone().run(&mut io).unwrap();
                io.output[0]
            })
        })
        .max()
        .unwrap()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0", 43210)]
    #[case(
        "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
        54321
    )]
    #[case("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0", 65210)]
    fn test_solve(#[case] program: &'static str, #[case] thrust: i64) {
        let input = Input::from(program);
        assert_eq!(solve(input), thrust);
    }
}

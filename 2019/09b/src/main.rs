use adventutil::intcode::{Intcode, VecIO};
use adventutil::Input;

fn solve(input: Input) -> i64 {
    let mut program = input.parse::<Intcode>();
    let mut io = VecIO::from([2]);
    program.run(&mut io).unwrap();
    match io.output.len() {
        1 => io.output[0],
        n => panic!("Got {n} outputs, expected 1"),
    }
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let mut program = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"
            .parse::<Intcode>()
            .unwrap();
        let mut io = VecIO::default();
        program.run(&mut io).unwrap();
        assert_eq!(
            io.output,
            [109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        );
    }

    #[test]
    fn test_example2() {
        let mut program = "1102,34915192,34915192,7,4,7,99,0"
            .parse::<Intcode>()
            .unwrap();
        let mut io = VecIO::default();
        program.run(&mut io).unwrap();
        assert_eq!(io.output.len(), 1);
        assert_eq!(io.output[0].to_string().len(), 16);
    }

    #[test]
    fn test_example3() {
        let mut program = "104,1125899906842624,99".parse::<Intcode>().unwrap();
        let mut io = VecIO::default();
        program.run(&mut io).unwrap();
        assert_eq!(io.output, [1125899906842624]);
    }
}

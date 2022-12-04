use adventutil::Input;

fn run(array: &mut [usize]) {
    let mut i = 0;
    while i < array.len() {
        match array[i] {
            1 => array[array[i + 3]] = array[array[i + 1]] + array[array[i + 2]],
            2 => array[array[i + 3]] = array[array[i + 1]] * array[array[i + 2]],
            99 => return,
            n => panic!("Invalid opcode {n}"),
        }
        i += 4;
    }
}

fn solve(input: Input) -> usize {
    let mut array = input.parse_csv_line::<usize>();
    array[1] = 12;
    array[2] = 2;
    run(&mut array);
    array[0]
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let mut array = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        run(&mut array);
        assert_eq!(array, [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    }

    #[test]
    fn test_example2() {
        let mut array = [1, 0, 0, 0, 99];
        run(&mut array);
        assert_eq!(array, [2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_example3() {
        let mut array = [2, 3, 0, 3, 99];
        run(&mut array);
        assert_eq!(array, [2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_example4() {
        let mut array = [2, 4, 4, 5, 99, 0];
        run(&mut array);
        assert_eq!(array, [2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_example5() {
        let mut array = [1, 1, 1, 4, 99, 5, 6, 0, 99];
        run(&mut array);
        assert_eq!(array, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}

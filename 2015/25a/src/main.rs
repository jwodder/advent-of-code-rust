use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser};

type UInt = u32;

const INITIAL: u64 = 20_151_125;
const FACTOR: u64 = 252_533;
const MODULUS: u64 = 33_554_393;
/// The order of `FACTOR` in the multiplicative group of integers *modulo*
/// `MODULUS`, i.e., the smallest positive integer `n` such that `FACTOR.pow(n)
/// % MODULUS == 1`.
const FACTOR_MOD_ORDER: UInt = 16_777_196;

fn parse_spec(s: &str) -> Result<(UInt, UInt), ParseError> {
    let mut parser = PullParser::new(s);
    parser
        .skip("To continue, please consult the code grid in the manual.  Enter the code at row ")?;
    let row = parser.parse_to::<UInt, _>(',')?;
    parser.skip(" column ")?;
    let column = parser.parse_to::<UInt, _>('.')?;
    parser.eof()?;
    Ok((row, column))
}

fn coord2n(row: UInt, column: UInt) -> UInt {
    // (column * (column - 1) + row * (row - 1)) / 2 + row * column - row + 1
    (column * (column - 1)).midpoint(row * (row - 1)) + row * column - row + 1
}

fn nth_code(n: UInt) -> UInt {
    let mut code = INITIAL;
    for _ in 0..((n - 1) % FACTOR_MOD_ORDER) {
        code = (code * FACTOR) % MODULUS;
    }
    UInt::try_from(code).expect("Overflow")
}

fn coord2code(row: UInt, column: UInt) -> UInt {
    nth_code(coord2n(row, column))
}

fn solve(input: Input) -> UInt {
    let (row, column) = parse_spec(input.read().trim()).expect("Parse error");
    coord2code(row, column)
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1, 1, 1)]
    #[case(2, 1, 2)]
    #[case(1, 2, 3)]
    #[case(4, 2, 12)]
    #[case(1, 5, 15)]
    fn test_coord2n(#[case] row: UInt, #[case] column: UInt, #[case] n: UInt) {
        assert_eq!(coord2n(row, column), n);
    }

    #[rstest]
    #[case(1, 1, 20151125)]
    #[case(2, 1, 31916031)]
    #[case(3, 1, 16080970)]
    #[case(6, 6, 27995004)]
    fn test_coord2code(#[case] row: UInt, #[case] column: UInt, #[case] code: UInt) {
        assert_eq!(coord2code(row, column), code);
    }
}

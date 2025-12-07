use adventutil::Input;
use adventutil::pullparser::ParseError;
use adventutil::ranges::parse_range;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Range {
    start: u32,
    end: u32, // inclusive
}

impl std::str::FromStr for Range {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Range, ParseError> {
        let (start, end) = parse_range::<u32>(s)?.into_inner();
        Ok(Range { start, end })
    }
}

fn solve(input: Input) -> u32 {
    let mut ranges = input.parse_lines::<Range>().collect::<Vec<_>>();
    ranges.sort_unstable();
    let mut iter = ranges.into_iter();
    let mut leading = iter.next().unwrap();
    let mut qty = leading.start;
    for r in iter {
        if leading.end.saturating_add(1) >= r.start {
            leading.end = leading.end.max(r.end);
        } else {
            qty += r.start - leading.end - 1;
            leading = r;
        }
    }
    qty += u32::MAX - leading.end;
    qty
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

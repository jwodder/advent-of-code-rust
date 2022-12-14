use adventutil::counter::Counter;
use adventutil::pullparser::{ParseError, PullParser};
use adventutil::Input;
use std::str::FromStr;

struct Room {
    name: String,
    sector: u32,
    checksum: String,
}

impl Room {
    fn valid(&self) -> bool {
        let counts = self
            .name
            .chars()
            .filter(|&c| c != '-')
            .collect::<Counter<_>>();
        let mut counts = counts.into_iter().collect::<Vec<_>>();
        counts.sort_by(|&(c1, q1), &(c2, q2)| (q2, c1).cmp(&(q1, c2)));
        counts
            .into_iter()
            .take(5)
            .map(|(c, _)| c)
            .collect::<String>()
            == self.checksum
    }
}

impl FromStr for Room {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Room, ParseError> {
        let mut parser = PullParser::new(s);
        let name_sector = parser.scan_to('[')?;
        let checksum = parser.parse_to::<String, _>(']')?;
        parser.eof()?;
        let (name, sector) = name_sector
            .rsplit_once('-')
            .ok_or_else(|| ParseError::InvalidToken(name_sector.into()))?;
        let sector = sector.parse::<u32>()?;
        Ok(Room {
            name: name.into(),
            sector,
            checksum,
        })
    }
}

fn solve(input: Input) -> u32 {
    input
        .parse_lines::<Room>()
        .filter_map(|r| r.valid().then_some(r.sector))
        .sum()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("aaaaa-bbb-z-y-x-123[abxyz]", true)]
    #[case("a-b-c-d-e-f-g-h-987[abcde]", true)]
    #[case("not-a-real-room-404[oarel]", true)]
    #[case("totally-real-room-200[decoy]", false)]
    fn test_valid(#[case] r: Room, #[case] valid: bool) {
        assert_eq!(r.valid(), valid);
    }
}

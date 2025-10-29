use adventutil::Input;
use adventutil::counter::Counter;
use adventutil::pullparser::{ParseError, PullParser};
use std::str::FromStr;

const ORD_A: u32 = 'a' as u32;

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

    fn decrypt(&self) -> String {
        self.name
            .chars()
            .map(|c| {
                if c == '-' {
                    ' '
                } else {
                    char::from_u32(((c as u32) - ORD_A + self.sector) % 26 + ORD_A).unwrap()
                }
            })
            .collect()
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
        .filter(Room::valid)
        .find(|r| r.decrypt() == "northpole object storage")
        .expect("Room not found")
        .sector
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
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

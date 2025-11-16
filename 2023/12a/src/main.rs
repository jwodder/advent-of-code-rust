use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};

#[derive(Clone, Debug, Eq, PartialEq)]
struct SpringRow {
    record: Vec<Spring>,
    contiguous: Vec<usize>,
}

impl SpringRow {
    #[allow(clippy::needless_range_loop)]
    fn arrangements(&self) -> u32 {
        let record_len = self.record.len();
        let contig_len = self.contiguous.len();
        // `tbl[i][j]` will contain the number of arrangements of
        // `self.contiguous[j..]` in the record `self.record[i..]`
        let mut tbl = vec![vec![0u32; contig_len + 1]; record_len + 1];
        tbl[record_len][contig_len] = 1;
        for j in 0..contig_len {
            tbl[record_len][j] = 0;
        }
        for i in (0..record_len).rev() {
            tbl[i][contig_len] =
                u32::from(tbl[i + 1][contig_len] == 1 && self.record[i] != Spring::Damaged);
        }
        for i in (0..record_len).rev() {
            for j in (0..contig_len).rev() {
                if matches!(self.record[i], Spring::Operational | Spring::Unknown) {
                    tbl[i][j] += tbl[i + 1][j];
                }
                if matches!(self.record[i], Spring::Damaged | Spring::Unknown) {
                    let start = i;
                    let end = i + self.contiguous[j];
                    if end <= record_len
                        && self.record[start..end]
                            .iter()
                            .all(|&s| s != Spring::Operational)
                    {
                        if end == record_len {
                            tbl[i][j] += tbl[end][j + 1];
                        } else if self.record[end] != Spring::Damaged {
                            tbl[i][j] += tbl[end + 1][j + 1];
                        }
                    }
                }
            }
        }
        tbl[0][0]
    }
}

impl std::str::FromStr for SpringRow {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<SpringRow, ParseError> {
        let mut parser = PullParser::new(s);
        let record = parser.scan_to(Token::Whitespace)?;
        let record = record
            .chars()
            .map(|c| match c {
                '.' => Ok(Spring::Operational),
                '#' => Ok(Spring::Damaged),
                '?' => Ok(Spring::Unknown),
                _ => Err(ParseError::InvalidToken(c.to_string())),
            })
            .collect::<Result<Vec<_>, _>>()?;
        let contiguous = parser.delimited(',', |s| s.parse::<usize>().map_err(Into::into))?;
        Ok(SpringRow { record, contiguous })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

fn solve(input: Input) -> u32 {
    input
        .parse_lines::<SpringRow>()
        .map(|sr| sr.arrangements())
        .sum()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from(concat!(
            "???.### 1,1,3\n",
            ".??..??...?##. 1,1,3\n",
            "?#?#?#?#?#?#?#? 1,3,1,6\n",
            "????.#...#... 4,1,1\n",
            "????.######..#####. 1,6,5\n",
            "?###???????? 3,2,1\n",
        ));
        assert_eq!(solve(input), 21);
    }
}

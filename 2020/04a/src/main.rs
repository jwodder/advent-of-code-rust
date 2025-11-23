use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser};
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
enum Field {
    Byr,
    Iyr,
    Eyr,
    Hgt,
    Hcl,
    Ecl,
    Pid,
    Cid,
}

impl std::str::FromStr for Field {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Field, ParseError> {
        use Field::*;
        match s {
            "byr" => Ok(Byr),
            "iyr" => Ok(Iyr),
            "eyr" => Ok(Eyr),
            "hgt" => Ok(Hgt),
            "hcl" => Ok(Hcl),
            "ecl" => Ok(Ecl),
            "pid" => Ok(Pid),
            "cid" => Ok(Cid),
            s => Err(ParseError::InvalidToken(s.into())),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Passport {
    fields: HashSet<Field>,
}

impl Passport {
    fn valid(&self) -> bool {
        self.fields.len() == 8 || (self.fields.len() == 7 && !self.fields.contains(&Field::Cid))
    }
}

impl std::str::FromStr for Passport {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Passport, ParseError> {
        let mut fields = HashSet::new();
        for kv in s.split_ascii_whitespace() {
            let mut parser = PullParser::new(kv);
            let key = parser.parse_to::<Field, _>(':')?;
            fields.insert(key);
        }
        Ok(Passport { fields })
    }
}

fn solve(input: Input) -> usize {
    input
        .paragraphs()
        .map(|s| s.parse::<Passport>().unwrap())
        .filter(Passport::valid)
        .count()
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
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n",
            "byr:1937 iyr:2017 cid:147 hgt:183cm\n",
            "\n",
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n",
            "hcl:#cfa07d byr:1929\n",
            "\n",
            "hcl:#ae17e1 iyr:2013\n",
            "eyr:2024\n",
            "ecl:brn pid:760753108 byr:1931\n",
            "hgt:179cm\n",
            "\n",
            "hcl:#cfa07d eyr:2025 pid:166559648\n",
            "iyr:2011 ecl:brn hgt:59in\n",
        ));
        assert_eq!(solve(input), 2);
    }
}

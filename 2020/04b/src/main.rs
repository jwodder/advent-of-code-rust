use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use std::collections::HashMap;
use std::str::FromStr;

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

impl Field {
    fn valid(&self, value: &str) -> bool {
        use Field::*;
        match self {
            Byr => {
                value.len() == 4
                    && value.chars().all(|c| c.is_ascii_digit())
                    && (1920..=2002).contains(&value.parse::<u32>().unwrap())
            }
            Iyr => {
                value.len() == 4
                    && value.chars().all(|c| c.is_ascii_digit())
                    && (2010..=2020).contains(&value.parse::<u32>().unwrap())
            }
            Eyr => {
                value.len() == 4
                    && value.chars().all(|c| c.is_ascii_digit())
                    && (2020..=2030).contains(&value.parse::<u32>().unwrap())
            }
            Hgt => {
                if let Some(s) = value.strip_suffix("cm") {
                    if let Ok(h) = s.parse::<usize>() {
                        (150..=193).contains(&h)
                    } else {
                        false
                    }
                } else if let Some(s) = value.strip_suffix("in") {
                    if let Ok(h) = s.parse::<usize>() {
                        (59..=76).contains(&h)
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Hcl => {
                let chars = value.chars().collect::<Vec<_>>();
                chars.len() == 7
                    && chars[0] == '#'
                    && chars[1..]
                        .iter()
                        .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
            }
            Ecl => ["amb", "blu", "brn", "grn", "gry", "hzl", "oth"]
                .binary_search(&value)
                .is_ok(),
            Pid => value.len() == 9 && value.chars().all(|c| c.is_ascii_digit()),
            Cid => true,
        }
    }
}

impl FromStr for Field {
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

struct Passport {
    fields: HashMap<Field, String>,
}

impl Passport {
    fn valid(&self) -> bool {
        (self.fields.len() == 8
            || (self.fields.len() == 7 && !self.fields.contains_key(&Field::Cid)))
            && self.fields.iter().all(|(k, v)| k.valid(v))
    }
}

impl FromStr for Passport {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Passport, ParseError> {
        let mut fields = HashMap::new();
        for kv in s.split_ascii_whitespace() {
            let mut parser = PullParser::new(kv);
            let key = parser.parse_to::<Field, _>(':')?;
            let value = parser.parse_to::<String, _>(Token::Eof)?;
            fields.insert(key, value);
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
    use rstest::rstest;
    use Field::*;

    #[rstest]
    #[case(Byr, "2002", true)]
    #[case(Byr, "2003", false)]
    #[case(Hgt, "60in", true)]
    #[case(Hgt, "190cm", true)]
    #[case(Hgt, "190in", false)]
    #[case(Hgt, "190", false)]
    #[case(Hcl, "#123abc", true)]
    #[case(Hcl, "#123abz", false)]
    #[case(Hcl, "123abc", false)]
    #[case(Ecl, "brn", true)]
    #[case(Ecl, "wat", false)]
    #[case(Pid, "000000001", true)]
    #[case(Pid, "0123456789", false)]
    fn test_field_valid(#[case] field: Field, #[case] value: &str, #[case] valid: bool) {
        assert_eq!(field.valid(value), valid);
    }

    #[rstest]
    #[case(
        "eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
        false
    )]
    #[case(
        "iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946",
        false
    )]
    #[case(
        "hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
        false
    )]
    #[case(
        "hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007",
        false
    )]
    #[case(
        "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
        true
    )]
    #[case(
        "eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
        true
    )]
    #[case(
        "hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022",
        true
    )]
    #[case(
        "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
        true
    )]
    fn test_passport_valid(#[case] passport: Passport, #[case] valid: bool) {
        assert_eq!(passport.valid(), valid);
    }
}

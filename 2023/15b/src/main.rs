use adventutil::Input;
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Step {
    Remove { label: String },
    Add { label: String, focal_len: u32 },
}

impl std::str::FromStr for Step {
    type Err = ParseStepError;

    fn from_str(s: &str) -> Result<Step, ParseStepError> {
        let i = s.find(['-', '=']).ok_or(ParseStepError::NoOp)?;
        let label = s[..i].to_owned();
        match &s[i..=i] {
            "-" => Ok(Step::Remove { label }),
            "=" => {
                let focal_len = s[(i + 1)..].parse::<u32>()?;
                Ok(Step::Add { label, focal_len })
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
enum ParseStepError {
    #[error("No '-' or '=' operator in step")]
    NoOp,
    #[error(transparent)]
    Int(#[from] std::num::ParseIntError),
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct LensBox(Vec<(String, u32)>);

impl LensBox {
    fn remove(&mut self, label: &str) {
        self.0.retain(|p| p.0 != label);
    }

    fn add(&mut self, label: String, focal_len: u32) {
        if let Some(p) = self.0.iter_mut().find(|p| p.0 == label) {
            p.1 = focal_len;
        } else {
            self.0.push((label, focal_len));
        }
    }

    fn powers(&self, boxno1: u32) -> u32 {
        self.0.iter().zip(1..).map(|(p, i)| boxno1 * i * p.1).sum()
    }
}

fn hash(s: &str) -> usize {
    let mut r = 0;
    for c in s.chars() {
        r = (r + u32::from(c)) * 17 % 256;
    }
    usize::try_from(r).unwrap()
}

fn solve(input: Input) -> u32 {
    let mut boxes = vec![LensBox::default(); 256];
    for step in input.parse_csv_line::<Step>() {
        match step {
            Step::Remove { label } => {
                let i = hash(&label);
                boxes[i].remove(&label);
            }
            Step::Add { label, focal_len } => {
                let i = hash(&label);
                boxes[i].add(label, focal_len);
            }
        }
    }
    boxes.into_iter().zip(1..).map(|(b, i)| b.powers(i)).sum()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        assert_eq!(solve(input), 145);
    }
}

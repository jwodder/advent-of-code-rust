use serde::Deserialize;
use std::fmt;
use std::path::PathBuf;
use std::process::{Command, ExitStatus, Stdio};
use thiserror::Error;

pub fn project_root() -> Result<PathBuf, LocateError> {
    match Command::new("cargo")
        .arg("locate-project")
        .arg("--workspace")
        .stderr(Stdio::inherit())
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
    {
        Ok(output) if output.status.success() => {
            let location = serde_json::from_slice::<LocateProject>(&output.stdout)?;
            if !location.root.is_absolute() {
                return Err(LocateError::InvalidPath(location.root));
            }
            if let Some(root) = location.root.parent() {
                Ok(root.to_owned())
            } else {
                Err(LocateError::InvalidPath(location.root))
            }
        }
        Ok(output) => Err(LocateError::Exit(output.status)),
        Err(e) => Err(LocateError::Startup(e)),
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
struct LocateProject {
    root: PathBuf,
}

#[derive(Debug, Error)]
pub enum LocateError {
    #[error("failed to run `cargo locate-project`")]
    Startup(#[source] std::io::Error),
    #[error("command `cargo locate-project` failed: {0}")]
    Exit(ExitStatus),
    #[error("could not deserialize `cargo locate-project` output")]
    Deserialize(#[from] serde_json::Error),
    #[error("manifest path is absolute or parentless: {}", .0.display())]
    InvalidPath(PathBuf),
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Problem {
    pub year: u32,
    pub day: u32,
    pub ab: char,
}

impl Problem {
    pub fn from_year_and_id(year: u32, id: &str) -> Option<Problem> {
        let (day, ab) = parse_problem_id(id)?;
        Some(Problem { year, day, ab })
    }

    pub fn id(self) -> String {
        format!("{:02}{}", self.day, self.ab)
    }

    pub fn package(self) -> String {
        format!("advent-of-code-{}-{:02}{}", self.year, self.day, self.ab)
    }

    pub fn input_file(self) -> String {
        format!("{}/inputs/{:02}.txt", self.year, self.day)
    }
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{:02}{}", self.year, self.day, self.ab)
    }
}

impl std::str::FromStr for Problem {
    type Err = ParseProblemError;

    fn from_str(s: &str) -> Result<Problem, ParseProblemError> {
        let (year, pr) = s.split_once('-').ok_or(ParseProblemError)?;
        let year = year.parse::<u32>().map_err(|_| ParseProblemError)?;
        if !(2015..2100).contains(&year) {
            return Err(ParseProblemError);
        }
        let (day, ab) = parse_problem_id(pr).ok_or(ParseProblemError)?;
        Ok(Problem { year, day, ab })
    }
}

fn parse_problem_id(s: &str) -> Option<(u32, char)> {
    if !(s.len() == 3 && s.chars().take(2).all(|c| c.is_ascii_digit())) {
        return None;
    }
    let day = s.get(0..2).and_then(|t| t.parse::<u32>().ok())?;
    let ab = match s.get(2..) {
        Some("a") => 'a',
        Some("b") => 'b',
        _ => return None,
    };
    Some((day, ab))
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("problems must in the form 20XX-XX{{a|b}}")]
pub struct ParseProblemError;

use anyhow::Context;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use thiserror::Error;

const MEAN_RATIO_THRESHOLD: f64 = 0.1;

#[derive(Clone, Debug, Eq, Parser, PartialEq)]
struct Arguments {
    #[arg(long)]
    all: bool,

    #[arg(long, default_value = "cmptimes.csv")]
    csv_file: PathBuf,

    committishes: String,

    problems: Vec<Problem>,
}

fn main() -> anyhow::Result<()> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!("[{:<5}] {}", record.level(), message));
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stderr())
        .apply()
        .expect("no other logger should have been previously initialized");
    let mut args = Arguments::parse();
    let root_dir = toollib::project_root()?;
    if args.all {
        args.problems = all_problems(&root_dir)?;
    }
    let mut reporter = Reporter::new(&args.committishes);
    let report_dir = root_dir.join("target").join("cmptimes");
    fs_err::create_dir_all(&report_dir)?;
    for pr in args.problems {
        log::info!("Comparing times for {pr} ...");
        let package = pr.package();
        let report_path = report_dir.join(format!("{pr}.json"));
        let rc = Command::new("hyperfine")
            .arg("-L")
            .arg("committish")
            .arg(&args.committishes)
            .arg("-s")
            .arg(format!(
                "git checkout {{committish}} && cargo build -r -p {package}"
            ))
            .arg("-n")
            .arg("{committish}")
            .arg("--export-json")
            .arg(&report_path)
            .arg(format!(
                "target/release/{package} {}/inputs/{}.txt",
                pr.year, pr.day
            ))
            .current_dir(&root_dir)
            .status()
            .context("failed to run hyperfine")?;
        if !rc.success() {
            anyhow::bail!("hyperfine for {pr} failed: {rc}");
        }
        let hfreport = serde_json::from_slice::<HyperfineReport>(&fs_err::read(report_path)?)?;
        reporter.add(pr, hfreport);
    }
    reporter
        .export_all(&args.csv_file)
        .context("failed to export CSV file")?;
    print!("{}", reporter.report());
    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Problem {
    year: u32,
    day: u32,
    ab: char,
}

impl Problem {
    fn from_year_and_id(year: u32, id: &str) -> Option<Problem> {
        let (day, ab) = parse_problem_id(id)?;
        Some(Problem { year, day, ab })
    }

    fn package(self) -> String {
        format!("advent-of-code-{}-{:02}{}", self.year, self.day, self.ab)
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
struct ParseProblemError;

#[derive(Clone, Debug, PartialEq)]
struct Reporter {
    committishes: Vec<String>,
    reports: BTreeMap<Problem, BTreeMap<String, HyperfineResult>>,
}

impl Reporter {
    fn new(committishes: &str) -> Reporter {
        Reporter {
            committishes: committishes.split(',').map(ToOwned::to_owned).collect(),
            reports: BTreeMap::new(),
        }
    }

    fn add(&mut self, pr: Problem, hfr: HyperfineReport) {
        let rs = hfr
            .results
            .into_iter()
            .map(|r| (r.command.clone(), r))
            .collect::<BTreeMap<_, _>>();
        self.reports.insert(pr, rs);
    }

    fn export_all(&self, path: &Path) -> csv::Result<()> {
        let mut out = csv::Writer::from_path(path)?;
        for (&pr, rs) in &self.reports {
            for committish in &self.committishes {
                if let Some(r) = rs.get(committish) {
                    let line = ReportLine {
                        problem: pr.to_string(),
                        committish,
                        mean: r.mean,
                        stddev: r.stddev,
                    };
                    out.serialize(line)?;
                }
            }
        }
        Ok(())
    }

    fn report(&self) -> String {
        let mut notable = Vec::new();
        for (&pr, rs) in &self.reports {
            for (i, c1) in self.committishes.iter().enumerate() {
                let mean1 = rs[c1].mean;
                for c2 in self.committishes.iter().skip(i + 1) {
                    let mean2 = rs[c2].mean;
                    let mut ratio = mean1 / mean2;
                    if ratio < 1.0 {
                        ratio = 1.0 / ratio;
                    }
                    if ratio >= (1.0 + MEAN_RATIO_THRESHOLD) {
                        notable.push(pr);
                    }
                }
            }
        }
        if notable.is_empty() {
            return String::from("No notable changes in runtime");
        }
        let mut s = String::from("| Problem |");
        for cm in &self.committishes {
            let _ = write!(&mut s, " {cm} |");
        }
        let _ = write!(&mut s, "\n| --- |");
        for _ in 0..self.committishes.len() {
            let _ = write!(&mut s, " --- |");
        }
        let _ = writeln!(&mut s);
        for pr in notable {
            let _ = write!(&mut s, "| {pr} |");
            for cm in &self.committishes {
                let r = &self.reports[&pr][cm];
                let _ = write!(&mut s, " {} s ± {} s |", r.mean, r.stddev);
            }
            let _ = writeln!(&mut s);
        }
        s
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct ReportLine<'a> {
    problem: String,
    committish: &'a str,
    mean: f64,
    stddev: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
struct HyperfineReport {
    results: Vec<HyperfineResult>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
struct HyperfineResult {
    command: String,
    mean: f64,
    stddev: f64,
    // median: f64,
    // user: f64,
    // system: f64,
    // min: f64,
    // max: f64,
    // times: Vec<f64>
    // exit_codes: Vec<u32>,
    // parameters: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
struct Answer {
    problem: String,
    //input: String,
    //answer: String,
}

fn all_problems(root: &Path) -> anyhow::Result<Vec<Problem>> {
    let mut problems = Vec::new();
    for entry in fs_err::read_dir(root)? {
        let entry = entry?;
        let answerpath = entry.path().join("answers.csv");
        if entry.file_type()?.is_dir() && answerpath.exists() {
            let year = match entry.file_name().into_string() {
                Ok(s) => match s.parse::<u32>() {
                    Ok(year) => year,
                    Err(_) => anyhow::bail!("Found answers.csv in non-year directory {s:?}"),
                },
                Err(oss) => anyhow::bail!(
                    "Found answers.csv in directory with undecodable name {:?}",
                    oss.to_string_lossy()
                ),
            };
            log::debug!("Reading answers from {}", answerpath.display());
            let mut reader = csv::Reader::from_path(&answerpath)
                .with_context(|| format!("failed to read {}", answerpath.display()))?;
            for answer in reader.deserialize::<Answer>() {
                let answer = answer.with_context(|| {
                    format!("failed to read entry from {}", answerpath.display())
                })?;
                if let Some(pr) = Problem::from_year_and_id(year, &answer.problem) {
                    problems.push(pr);
                } else {
                    anyhow::bail!(
                        "Invalid problem id {} in {year}/answers.csv",
                        answer.problem
                    );
                }
            }
        }
    }
    problems.sort_unstable();
    Ok(problems)
}

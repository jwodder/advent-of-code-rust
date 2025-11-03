use anyhow::{Context, bail};
use fs_err::read_dir;
use futures_util::stream::{StreamExt, iter};
use serde::Deserialize;
use std::fmt::{self, Write as _};
use std::path::Path;
use std::process::ExitCode;
use std::time::{Duration, Instant};
use tokio::process::Command;
use tokio::time::timeout;

const WORKERS: usize = 8;
const TIMEOUT: Duration = Duration::from_secs(30);
const SLOWEST_QTY: usize = 10;

#[derive(Debug, Deserialize, Eq, PartialEq)]
struct Answer {
    problem: String,
    input: String,
    #[allow(clippy::struct_field_names)]
    answer: String,
}

struct TestCase<'a> {
    workspace_dir: &'a Path,
    year: i32,
    answer: Answer,
}

impl TestCase<'_> {
    async fn run(self) -> (String, TestResult) {
        log::info!("RUNNING: {self}");
        let mut cmd = Command::new("cargo");
        cmd.arg("run")
            .arg("-q")
            .arg("-r")
            .arg("-p")
            .arg(format!(
                "advent-of-code-{}-{}",
                self.year, self.answer.problem
            ))
            .arg("--")
            .arg(format!("{}/inputs/{}", self.year, self.answer.input))
            .current_dir(self.workspace_dir)
            .kill_on_drop(true);
        let start = Instant::now();
        let r = timeout(TIMEOUT, cmd.output()).await;
        let elapsed = start.elapsed();
        let name = self.to_string();
        match r {
            Ok(Ok(out)) => {
                if out.status.success() {
                    if let Ok(s) = String::from_utf8(out.stdout) {
                        if s.trim() == self.answer.answer {
                            log::info!("PASS: {self}");
                            (name, TestResult::Success { elapsed })
                        } else {
                            log::error!("FAIL: {self}");
                            (name, TestResult::Fail)
                        }
                    } else {
                        log::error!("Problem {self} binary emitted non-UTF-8");
                        (name, TestResult::Fail)
                    }
                } else {
                    log::error!("Problem {} binary failed: {}", self, out.status);
                    // TODO: Display stderr?
                    (name, TestResult::Fail)
                }
            }
            Ok(Err(e)) => {
                log::error!("Problem {self} binary failed to execute: {e}");
                (name, TestResult::Fail)
            }
            Err(_) => {
                log::error!("TIMEOUT: {self}");
                (name, TestResult::Timeout)
            }
        }
    }
}

impl fmt::Display for TestCase<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.year, self.answer.problem)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum TestResult {
    Success { elapsed: Duration },
    Timeout,
    Fail,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Reporter {
    timeouts: Vec<String>,
    slowest: Vec<(String, Duration)>,
    success: bool,
}

impl Reporter {
    fn from_results(res: Vec<(String, TestResult)>) -> Reporter {
        let mut success = true;
        let mut timeouts = Vec::new();
        let mut slowest = Vec::new();
        for (name, tr) in res {
            match tr {
                TestResult::Success { elapsed } => slowest.push((name, elapsed)),
                TestResult::Timeout => {
                    timeouts.push(name);
                    success = false;
                }
                TestResult::Fail => success = false,
            }
        }
        timeouts.sort_unstable();
        slowest.sort_unstable_by_key(|&(_, dur)| std::cmp::Reverse(dur));
        slowest.truncate(SLOWEST_QTY);
        Reporter {
            timeouts,
            slowest,
            success,
        }
    }

    fn write_slowest(&self) -> anyhow::Result<()> {
        let mut s = String::from("## Slowest Solutions\n\n| Problem | Runtime |\n| --- | --- |\n");
        for name in &self.timeouts {
            let _ = writeln!(&mut s, "| {name} | TIMEOUT |");
        }
        for (name, dur) in &self.slowest {
            let _ = writeln!(&mut s, "| {name} | {dur:?} |");
        }
        if let Some(path) = std::env::var_os("GITHUB_STEP_SUMMARY") {
            fs_err::write(path, s)?;
        } else {
            print!("\n{s}");
        }
        Ok(())
    }

    fn status(&self) -> ExitCode {
        if self.success {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<ExitCode> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!("[{:<5}] {}", record.level(), message));
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stderr())
        .apply()
        .expect("no other logger should have been previously initialized");
    let mut cases = Vec::new();
    let workspace_dir = toollib::project_root()?;
    for entry in read_dir(&workspace_dir)? {
        let entry = entry?;
        let answerpath = entry.path().join("answers.csv");
        if entry.file_type()?.is_dir() && answerpath.exists() {
            let year = match entry.file_name().into_string() {
                Ok(s) => match s.parse::<i32>() {
                    Ok(year) => year,
                    Err(_) => bail!("Found answers.csv in non-year directory {s:?}"),
                },
                Err(oss) => bail!(
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
                cases.push(TestCase {
                    workspace_dir: &workspace_dir,
                    year,
                    answer,
                });
            }
        }
    }
    let res = iter(cases)
        .map(TestCase::run)
        .buffer_unordered(WORKERS)
        .collect::<Vec<_>>()
        .await;
    let reporter = Reporter::from_results(res);
    reporter.write_slowest()?;
    Ok(reporter.status())
}

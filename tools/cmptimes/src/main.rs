use anyhow::Context;
use clap::Parser;
use patharg::OutputArg;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use toollib::Problem;

const MEAN_RATIO_THRESHOLD: f64 = 0.1;

/// Run the solutions to the given problems across multiple Git committishes,
/// time the executions, and output notable runtime differences
#[derive(Clone, Debug, Eq, Parser, PartialEq)]
struct Arguments {
    /// Time all solutions
    ///
    /// The {year}/answer.csv files must be decrypted before this option can be
    /// used.
    #[arg(short = 'a', long)]
    all: bool,

    /// Write a CSV file of all times to the given path
    #[arg(short = 'C', long, default_value = "cmptimes.csv", value_name = "PATH")]
    csv_file: PathBuf,

    /// Do not emit a report of notable runtime differences
    #[arg(short, long)]
    no_report: bool,

    /// Write a report of notable runtime differences to the given path
    #[arg(short, long, default_value_t, value_name = "PATH")]
    outfile: OutputArg,

    /// Include all solutions in the report regardless of runtime ratio
    #[arg(short = 'R', long)]
    report_all: bool,

    /// A comma-separated list of Git committishes from which to time solutions
    committishes: String,

    /// Problem IDs in the form "YYYY-DD{a|b}"
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
        args.problems = toollib::get_all_solutions(&root_dir)?
            .into_iter()
            .map(|(pr, _)| pr)
            .collect();
        args.problems.sort_unstable();
    }
    let start_head = get_git_head(&root_dir).context("failed to determine current Git HEAD")?;
    let mut reporter = Reporter::new(&args.committishes, args.report_all);
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
            .arg("-n")
            .arg("{committish}")
            .arg("--shell=none")
            .arg("-s")
            .arg(format!(
                "bash -c 'git checkout {{committish}} && cargo build -r -p {package}'"
            ))
            .arg("--export-json")
            .arg(&report_path)
            .arg(format!("target/release/{package} {}", pr.input_file()))
            .current_dir(&root_dir)
            .status()
            .context("failed to run hyperfine")?;
        if !rc.success() {
            anyhow::bail!("hyperfine for {pr} failed: {rc}");
        }
        let hfreport = serde_json::from_slice::<HyperfineReport>(&fs_err::read(report_path)?)?;
        reporter.add(pr, hfreport);
    }
    let rc = Command::new("git")
        .arg("checkout")
        .arg(start_head)
        .current_dir(&root_dir)
        .status()
        .context("failed to run `git checkout ...`")?;
    if !rc.success() {
        log::warn!("Failed to check initial Git HEAD back out");
    }
    reporter
        .export_all(&args.csv_file)
        .context("failed to export CSV file")?;
    if !args.no_report {
        args.outfile
            .write(reporter.report())
            .context("failed to write report")?;
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq)]
struct Reporter {
    committishes: Vec<String>,
    reports: BTreeMap<Problem, BTreeMap<String, HyperfineResult>>,
    report_all: bool,
}

impl Reporter {
    fn new(committishes: &str, report_all: bool) -> Reporter {
        Reporter {
            committishes: committishes.split(',').map(ToOwned::to_owned).collect(),
            reports: BTreeMap::new(),
            report_all,
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

    fn is_notable(&self, rs: &BTreeMap<String, HyperfineResult>) -> bool {
        if self.report_all {
            return true;
        }
        for (i, c1) in self.committishes.iter().enumerate() {
            let mean1 = rs[c1].mean;
            for c2 in self.committishes.iter().skip(i + 1) {
                let mean2 = rs[c2].mean;
                let mut ratio = mean1 / mean2;
                if ratio < 1.0 {
                    ratio = 1.0 / ratio;
                }
                if ratio >= (1.0 + MEAN_RATIO_THRESHOLD) || self.report_all {
                    return true;
                }
            }
        }
        false
    }

    fn report(&self) -> String {
        let mut table = MarkdownTable::new(
            std::iter::once("Problem").chain(self.committishes.iter().map(String::as_str)),
        );
        for (&pr, rs) in &self.reports {
            if self.is_notable(rs) {
                let mut cells = vec![pr.to_string()];
                for cm in &self.committishes {
                    let r = &self.reports[&pr][cm];
                    cells.push(format!(
                        "{} ± {}",
                        show_seconds(r.mean),
                        show_seconds(r.stddev)
                    ));
                }
                table.add_row(cells);
            }
        }
        if table.is_empty() {
            String::from("No notable changes in runtime\n")
        } else {
            table.display()
        }
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

fn get_git_head(dirpath: &Path) -> anyhow::Result<String> {
    let git_dir = read_git(dirpath, vec!["rev-parse", "--git-dir"])?;
    let head = fs_err::read(Path::new(&git_dir).join("HEAD"))?;
    let head = std::str::from_utf8(&head).context("failed to decode {git_dir}/HEAD contents")?;
    if let Some(s) = head.trim().strip_prefix("ref: ") {
        Ok(s.strip_prefix("refs/heads/").unwrap_or(s).to_owned())
    } else {
        match read_git(dirpath, vec!["describe", "--tags", "--exact-match", "HEAD"]) {
            Ok(tag) => Ok(tag),
            Err(_) => read_git(dirpath, vec!["rev-parse", "--short", "HEAD"]),
        }
    }
}

fn read_git(dirpath: &Path, args: Vec<&'static str>) -> anyhow::Result<String> {
    let mut cmdline = String::from("git");
    for s in &args {
        cmdline.push(' ');
        cmdline.push_str(s);
    }
    let output = Command::new("git")
        .args(args)
        .stderr(Stdio::null())
        .current_dir(dirpath)
        .output()
        .with_context(|| format!("failed to run `{cmdline}`"))?;
    if !output.status.success() {
        anyhow::bail!("`{cmdline}` command was not successful: {}", output.status);
    }
    Ok(std::str::from_utf8(&output.stdout)
        .with_context(|| format!("`{cmdline}` output was not UTF-8"))?
        .trim()
        .to_owned())
}

fn show_seconds(s: f64) -> String {
    if s >= 1.0 {
        format!("{s:.1} s")
    } else {
        format!("{:.1} ms", s * 1000.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct MarkdownTable {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    colwidths: Vec<usize>,
}

impl MarkdownTable {
    fn new<I: IntoIterator<Item: Into<String>>>(headers: I) -> MarkdownTable {
        let headers = headers.into_iter().map(Into::into).collect::<Vec<_>>();
        let colwidths = headers.iter().map(|s| charwidth(s)).collect();
        MarkdownTable {
            headers,
            rows: Vec::new(),
            colwidths,
        }
    }

    fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    fn add_row<I: IntoIterator<Item: Into<String>>>(&mut self, cells: I) {
        let cells = cells.into_iter().map(Into::into).collect::<Vec<_>>();
        assert_eq!(cells.len(), self.headers.len(), "Ragged Markdown table");
        for (width, cell) in std::iter::zip(&mut self.colwidths, &cells) {
            let cellwidth = charwidth(cell);
            if *width < cellwidth {
                *width = cellwidth;
            }
        }
        self.rows.push(cells);
    }

    fn display(&self) -> String {
        let mut s = String::from("|");
        for (h, &width) in std::iter::zip(&self.headers, &self.colwidths) {
            let _ = write!(&mut s, " {h:width$} |");
        }
        let _ = write!(&mut s, "\n|");
        for &width in &self.colwidths {
            let _ = write!(&mut s, " {:-<width$} |", "");
        }
        let _ = writeln!(&mut s);
        for row in &self.rows {
            let _ = write!(&mut s, "|");
            for (cell, &width) in std::iter::zip(row, &self.colwidths) {
                let _ = write!(&mut s, " {cell:width$} |");
            }
            let _ = writeln!(&mut s);
        }
        s
    }
}

fn charwidth(s: &str) -> usize {
    s.chars().count()
}

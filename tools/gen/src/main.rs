use serde::Serialize;
use tinytemplate::TinyTemplate;

static CARGO_TOML_TEMPLATE: &str = include_str!("templates/Cargo.toml.tt");
static SRC_MAIN_RS_TEMPLATE: &str = include_str!("templates/src-main.rs.tt");

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct Context {
    year: String,
    problem: String,
}

fn main() -> anyhow::Result<()> {
    let Some(ctx) = parse_args() else {
        anyhow::bail!("Usage: cargo gen YYYY DD{{a|b}}");
    };
    let mut tt = TinyTemplate::new();
    tt.add_template("Cargo.toml", CARGO_TOML_TEMPLATE)?;
    tt.add_template("src-main.rs", SRC_MAIN_RS_TEMPLATE)?;
    let problem_dir = toollib::project_root()?.join(&ctx.year).join(&ctx.problem);
    fs_err::create_dir_all(problem_dir.join("src"))?;
    fs_err::write(
        problem_dir.join("Cargo.toml"),
        tt.render("Cargo.toml", &ctx)?,
    )?;
    fs_err::write(
        problem_dir.join("src").join("main.rs"),
        tt.render("src-main.rs", &ctx)?,
    )?;
    println!("New solution templated at {}", problem_dir.display());
    Ok(())
}

fn parse_args() -> Option<Context> {
    let mut argv = std::env::args().skip(1);
    let year = argv.next()?;
    if !(year.len() == 4 && year.chars().all(|c| c.is_ascii_digit())) {
        return None;
    }
    let problem = argv.next()?;
    if !(problem.len() == 3
        && problem.chars().take(2).all(|c| c.is_ascii_digit())
        && "ab".contains(problem.get(2..3)?))
    {
        return None;
    }
    Some(Context { year, problem })
}

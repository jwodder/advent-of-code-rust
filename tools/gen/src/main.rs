use serde::Serialize;
use tinytemplate::TinyTemplate;
use toollib::Problem;

static CARGO_TOML_TEMPLATE: &str = include_str!("templates/Cargo.toml.tt");
static SRC_MAIN_RS_TEMPLATE: &str = include_str!("templates/src-main.rs.tt");

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct Context {
    year: String,
    problem: String,
}

fn main() -> anyhow::Result<()> {
    let Some(pr) = std::env::args()
        .nth(1)
        .and_then(|s| s.parse::<Problem>().ok())
    else {
        anyhow::bail!("Usage: cargo gen YYYY-DD{{a|b}}");
    };
    let ctx = Context {
        year: pr.year.to_string(),
        problem: pr.id(),
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

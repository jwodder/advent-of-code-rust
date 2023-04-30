use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use std::collections::HashMap;

fn solve(input: Input) -> u32 {
    let mut components = Vec::new();
    let mut dirsizes = HashMap::new();
    for ln in input.lines() {
        if let Some(dir) = ln.strip_prefix("$ cd ") {
            if dir == "/" {
                components.clear();
            } else if dir == ".." {
                components.pop();
            } else {
                components.push(dir.to_string());
            }
        } else if let Ok((size, _)) = parse_file_size(&ln) {
            for i in 0..(components.len() + 1) {
                *dirsizes.entry(components[0..i].to_vec()).or_insert(0) += size;
            }
        } else if ln != "$ ls" && !ln.starts_with("dir ") {
            panic!("Invalid line {ln:?}");
        }
    }
    let unused = 70_000_000 - dirsizes[&Vec::new()];
    let needed = 30_000_000 - unused;
    dirsizes
        .into_values()
        .filter(|&sz| sz >= needed)
        .min()
        .unwrap()
}

fn parse_file_size(s: &str) -> Result<(u32, String), ParseError> {
    let mut parser = PullParser::new(s);
    let size = parser.parse_to::<u32, _>(Token::Whitespace)?;
    let filename = parser.parse_to::<String, _>(Token::Eof)?;
    Ok((size, filename))
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from(concat!(
            "$ cd /\n",
            "$ ls\n",
            "dir a\n",
            "14848514 b.txt\n",
            "8504156 c.dat\n",
            "dir d\n",
            "$ cd a\n",
            "$ ls\n",
            "dir e\n",
            "29116 f\n",
            "2557 g\n",
            "62596 h.lst\n",
            "$ cd e\n",
            "$ ls\n",
            "584 i\n",
            "$ cd ..\n",
            "$ cd ..\n",
            "$ cd d\n",
            "$ ls\n",
            "4060174 j\n",
            "8033020 d.log\n",
            "5626152 d.ext\n",
            "7214296 k\n",
        ));
        assert_eq!(solve(input), 24933642);
    }
}

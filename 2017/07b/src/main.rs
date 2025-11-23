use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, Eq, PartialEq)]
struct Program {
    name: String,
    weight: usize,
    children: Vec<String>,
}

impl std::str::FromStr for Program {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Program, ParseError> {
        let mut parser = PullParser::new(s);
        let name = parser.scan_to(Token::Whitespace)?.to_owned();
        parser.skip('(')?;
        let weight = parser.parse_to::<usize, _>(')')?;
        let children = if parser.eof().is_err() {
            parser.skip(" -> ")?;
            parser.delimited(", ", |p| Ok(p.to_owned()))?
        } else {
            Vec::new()
        };
        Ok(Program {
            name,
            weight,
            children,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Tower {
    programs: HashMap<String, Program>,
    parents2children: HashMap<String, Vec<String>>,
}

impl Tower {
    fn new() -> Tower {
        Tower {
            programs: HashMap::new(),
            parents2children: HashMap::new(),
        }
    }

    fn add(&mut self, p: Program) {
        self.parents2children
            .insert(p.name.clone(), p.children.clone());
        self.programs.insert(p.name.clone(), p);
    }

    fn root(&self) -> &str {
        let mut parentless = self.programs.keys().collect::<HashSet<_>>();
        for children in self.parents2children.values() {
            for c in children {
                parentless.remove(c);
            }
        }
        assert_eq!(parentless.len(), 1);
        parentless.into_iter().next().unwrap()
    }

    // If the children of `name` are balanced, returns Ok with `name`'s weight
    // and the sum of its children's subtower weights; otherwise, returns Err
    // with the weight that a descendant would need to be to balance the tower.
    fn child_weights(&self, name: &str) -> Result<(usize, usize), usize> {
        let mut weights = HashMap::<usize, Vec<(usize, usize)>>::new();

        for c in &self.parents2children[name] {
            let (w, subweight) = self.child_weights(c)?;
            weights
                .entry(w + subweight)
                .or_default()
                .push((w, subweight));
        }
        let weight_sum = if weights.is_empty() {
            0
        } else {
            let mut single_subtower_weight = None;
            let mut single_node_weight = None;
            let mut common_subtower_weight = None;
            let mut common_qty = None;
            for (w, tower_weights) in weights {
                if tower_weights.len() == 1 {
                    assert!(
                        single_subtower_weight.replace(w).is_none(),
                        "Multiple weights with count == 1"
                    );
                    single_node_weight = Some(tower_weights.into_iter().next().unwrap().0);
                } else {
                    assert!(
                        common_subtower_weight.replace(w).is_none(),
                        "Multiple weights with count != 1"
                    );
                    common_qty = Some(tower_weights.len());
                }
            }
            let common = common_subtower_weight.unwrap();
            if let Some(w) = single_subtower_weight {
                let node_weight = single_node_weight.unwrap();
                return Err(if common > w {
                    common - w + node_weight
                } else {
                    node_weight - (w - common)
                });
            } else {
                common * common_qty.unwrap()
            }
        };
        Ok((self.programs[name].weight, weight_sum))
    }
}

fn solve(input: Input) -> usize {
    let mut tower = Tower::new();
    for program in input.parse_lines::<Program>() {
        tower.add(program);
    }
    let Err(n) = tower.child_weights(tower.root()) else {
        panic!("Tower is already balanced");
    };
    n
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
            "pbga (66)\n",
            "xhth (57)\n",
            "ebii (61)\n",
            "havc (66)\n",
            "ktlj (57)\n",
            "fwft (72) -> ktlj, cntj, xhth\n",
            "qoyq (66)\n",
            "padx (45) -> pbga, havc, qoyq\n",
            "tknk (41) -> ugml, padx, fwft\n",
            "jptl (61)\n",
            "ugml (68) -> gyxo, ebii, jptl\n",
            "gyxo (61)\n",
            "cntj (57)\n",
        ));
        assert_eq!(solve(input), 60);
    }
}

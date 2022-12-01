use adventutil::Input;
use json::JsonValue;

fn sum_nums(obj: &JsonValue) -> i32 {
    use JsonValue::*;
    match obj {
        Null | Short(_) | String(_) | Boolean(_) => 0,
        Number(n) => {
            // The Err type used by json for this conversion does not implement
            // Debug, so we can't use unwrap() or expect():
            if let Ok(i) = i32::try_from(*n) {
                i
            } else {
                panic!("Number does not fit in i32");
            }
        }
        Array(values) => values.iter().map(sum_nums).sum(),
        Object(o) => {
            let mut total = 0;
            for (_, v) in o.iter() {
                if v == "red" {
                    return 0;
                }
                total += sum_nums(v);
            }
            total
        }
    }
}

fn solve(input: Input) -> i32 {
    let obj = json::parse(&input.read()).expect("Error parsing JSON");
    sum_nums(&obj)
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("[1,2,3]", 6)]
    #[case(r#"[1,{"c":"red","b":2},3]"#, 4)]
    #[case(r#"{"d":"red","e":[1,2,3,4],"f":5}"#, 0)]
    #[case(r#"[1,"red",5]"#, 6)]
    fn test_solve(#[case] s: &'static str, #[case] total: i32) {
        assert_eq!(solve(Input::from(s)), total);
    }
}

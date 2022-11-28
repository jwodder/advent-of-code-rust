use adventutil::Input;

fn main() {
    println!("{}", Input::from_env().parse_lines::<i32>().sum::<i32>());
}

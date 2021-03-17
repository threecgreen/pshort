use std::env;

const TRUNC_LEN: usize = 1;
const DEFAULT_TARGET_LEN: usize = 18;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = pshort::replace_home(args.get(1).unwrap_or(&"~".to_owned()));
    let target_len: usize = args
        .get(2)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(DEFAULT_TARGET_LEN);
    println!("{}", pshort::truncate_path(&path, target_len, TRUNC_LEN));
}

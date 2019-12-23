use std::env;

use pshort;

fn main() {
    let args: Vec<String> = env::args().collect();
    let target_len: usize = args
        .get(2)
        .unwrap_or(&"1".to_owned())
        .parse::<usize>()
        .unwrap();
    let path = pshort::replace_home(args.get(1).unwrap_or(&"~".to_owned()));
    println!("{}", pshort::truncate_path(&path, target_len, 1));
}

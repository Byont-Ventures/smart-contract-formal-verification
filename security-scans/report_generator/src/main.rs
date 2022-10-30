use slither_runner as slither;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contract_name = &args[1];

    let path = env::current_dir().expect("ERROR: Failed to get current path!");
    let path_string = path.to_str().expect("ERROR: can not convert path to string!");
    let _result = slither::run_slither(path_string, contract_name);
    // println!("Result is {}", result);
}
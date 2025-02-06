use std::env;
use egen::extract;

fn main() {
    let args: Vec<String> = env::args().collect();
    extract(&args);
    return;
}

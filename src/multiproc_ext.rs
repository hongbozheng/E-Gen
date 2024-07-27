use std::env;
use egg::extract;

fn main() {
    let args: Vec<String> = env::args().collect();
    extract(&args);
    return;
}

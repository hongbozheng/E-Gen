use egen::generate;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    generate(&args);
    return;
}

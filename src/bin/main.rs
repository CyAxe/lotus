use lottas::start;
use std::io::{self, BufRead};


fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    start(lines.map(|x| x.unwrap()).collect());
}

use std::io::{self, BufRead, Write};
use ulj_core;

fn main() {
    let stdin = io::stdin();
    let mut lines = String::new();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        lines.push_str(&line);
    }
    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
    let output = ulj_core::parse_ul(lines).to_string();
    writeln!(handle, "{}", output).expect("Could not write stdout"); // add `?` if you care about errors here
}

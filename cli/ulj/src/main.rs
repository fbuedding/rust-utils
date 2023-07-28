use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        let stdout = io::stdout(); // get the global stdout entity
        let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
        writeln!(handle, "{}", line).expect("Could not write stdout"); // add `?` if you care about errors here
    }
}

use clap::Parser;
use std::io::{self, BufRead, Write};
use ulj_core;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    attr_def: bool,
}
fn main() {
    let args = Args::parse();
    let stdin = io::stdin();
    let mut lines = String::new();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        lines.push_str(&line);
    }
    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
    let output : String; 
    if args.attr_def {
        output = ulj_core::parse_ul_to_attribute(lines.as_str()).to_string();
    } else {
        output = ulj_core::parse_ul(lines.as_str()).to_string();
    }
    writeln!(handle, "{}", output).expect("Could not write stdout"); // add `?` if you care about errors here
}

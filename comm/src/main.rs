#![forbid(unsafe_code)]

use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::{self, BufReader};

fn read_lines(path: &str) -> io::Result<HashSet<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut result = HashSet::new();
    for line in reader.lines() {
        result.insert(line?);
    }
    Ok(result)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let strings1 = read_lines(&args[1]).unwrap();
    let strings2 = read_lines(&args[2]).unwrap();
    for line in strings1.intersection(&strings2) {
        println!("{}", line);
    }
}

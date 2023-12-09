use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn sample_lines(id: &str) -> impl Iterator<Item=String> {
  BufReader::new(File::open(format!("data/{}.txt", id)).unwrap())
    .lines()
    .map(Result::unwrap)
}

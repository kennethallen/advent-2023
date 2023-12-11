use std::io::{BufRead, BufReader};
use std::fs::{File, read_to_string};

pub fn sample_lines(id: &str) -> impl Iterator<Item=String> {
  BufReader::new(File::open(sample_filename(id)).unwrap())
    .lines()
    .map(Result::unwrap)
}

pub fn sample_file(id: &str) -> String {
  read_to_string(sample_filename(id)).unwrap()
}

fn sample_filename(id: &str) -> String {
  format!("data/{}.txt", id)
}

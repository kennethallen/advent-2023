use std::iter::zip;

use crate::util::usize;

use nom::{IResult, character::complete::{char, line_ending}, bytes::complete::tag, multi::many1, sequence::{delimited, preceded, terminated, pair}, combinator::eof, Parser};

pub fn part1(file: String) -> usize {
  let (_, races) = parse(file.as_str()).unwrap();
  races.into_iter()
    .map(|(time, record)| {
      let mut ways = 0;
      for hold in 0..=time {
        if record < hold * (time - hold) {
          ways += 1
        }
      }
      ways
    })
    .product()
}

fn parse(input: &str) -> IResult<&str, Vec<(usize, usize)>> {
  terminated(
    pair(
      delimited(
        tag("Time:"),
        many1(preceded(many1(char(' ')), usize)),
        line_ending,
      ),
      delimited(
        tag("Distance:"),
        many1(preceded(many1(char(' ')), usize)),
        line_ending,
      ),
    ),
    eof,
  ).map(|(times, dists)| {
    assert!(times.len() == dists.len());
    zip(times, dists).collect()
  }).parse(input)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_util::sample_file;

  #[test]
  fn test1_sample() {
    assert_eq!(part1(sample_file("06a")), 288);
  }

  #[test]
  fn test1() {
    assert_eq!(part1(sample_file("06")), 2374848);
  }
}

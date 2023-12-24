use std::num::ParseIntError;

use crate::util::usize;

use itertools::Itertools;
use nom::{IResult, character::complete::{char, line_ending, digit1}, bytes::complete::tag, multi::many1, sequence::{delimited, preceded, terminated, pair}, combinator::{eof, verify, map_res, map}};

pub fn part1(file: String) -> usize { process(file, parse_num_list) }
pub fn part2(file: String) -> usize { process(file, parse_big_num) }

fn process(file: String, parse_nums: impl Fn(&str) -> IResult<&str, Vec<usize>>) -> usize {
  let (_, races) = parse(file.as_str(), parse_nums).unwrap();
  races.into_iter()
    .map(|(time, record)| {
      let mut lo = 0;
      let mut hi = time/2;
      while hi > lo {
        let mid = (hi + lo) / 2;
        let score = mid * (time - mid);
        if score > record {
          hi = mid;
        } else {
          lo = mid + 1;
        }
      }
      time + 1 - 2 * lo
    })
    .product()
}

fn parse(input: &str, parse_nums: impl Fn(&str) -> IResult<&str, Vec<usize>>) -> IResult<&str, Vec<(usize, usize)>> {
  map(
    verify(
      terminated(
        pair(
          delimited(
            tag("Time:"),
            &parse_nums,
            line_ending,
          ),
          delimited(
            tag("Distance:"),
            &parse_nums,
            line_ending,
          ),
        ),
        eof,
      ),
      |(times, dists)| times.len() == dists.len(),
    ),
    |(times, dists)| times.into_iter().zip_eq(dists).collect(),
  )(input)
}

fn parse_num_list(input: &str) -> IResult<&str, Vec<usize>> {
  many1(preceded(many1(char(' ')), usize))(input)
}

fn parse_big_num(input: &str) -> IResult<&str, Vec<usize>> {
  map_res(
    many1(preceded(many1(char(' ')), digit1)),
    |chunks| -> Result<_, ParseIntError> {
      let chunks: String = chunks.into_iter().collect();
      Ok(vec![chunks.parse()?])
    },
  )(input)
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

  #[test]
  fn test2_sample() {
    assert_eq!(part2(sample_file("06a")), 71503);
  }

  #[test]
  fn test2() {
    assert_eq!(part2(sample_file("06")), 39132886);
  }
}

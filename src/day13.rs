use std::cmp::min;

use itertools::{Itertools, iproduct};
use nom::{IResult, character::complete::{one_of, line_ending}, multi::{many1, separated_list1}, sequence::terminated, combinator::{eof, map, verify}};

pub fn part1(file: String) -> usize { process(file, 0) }
pub fn part2(file: String) -> usize { process(file, 1) }

fn process(file: String, target_mistakes: usize) -> usize {
  parse(file.as_str()).unwrap().1.into_iter()
    .map(|pat| {
      (1..pat.len())
        .find(|&y|
          iproduct!(0..min(y, pat.len()-y), 0..pat[0].len())
            .filter(|&(off, x)| pat[y+off][x] != pat[y-1-off][x])
            .take(target_mistakes + 1)
            .count() == target_mistakes
        )
        .map(|y| y*100)
        .or_else(||
          (1..pat[0].len())
            .find(|&x|
              iproduct!(0..min(x, pat[0].len()-x), &pat)
                .filter(|&(off, row)| row[x+off] != row[x-1-off])
                .take(target_mistakes + 1)
                .count() == target_mistakes
            )
        )
        .unwrap()
    })
    .sum()
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<Vec<bool>>>> {
  terminated(
    separated_list1(
      line_ending,
      verify(
        many1(terminated(
          many1(map(
            one_of("#."),
            |c| match c { '#' => true, '.' => false, _=> unreachable!() },
            )),
          line_ending,
        )),
        |pat: &Vec<Vec<bool>>| pat.iter().map(Vec::len).all_equal(),
      ),
    ),
    eof,
  )(input)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_util::sample_file;

  #[test]
  fn test1_sample() {
    assert_eq!(part1(sample_file("13a")), 405);
  }

  #[test]
  fn test1() {
    assert_eq!(part1(sample_file("13")), 30802);
  }

  #[test]
  fn test2_sample() {
    assert_eq!(part2(sample_file("13a")), 400);
  }

  #[test]
  fn test2() {
    assert_eq!(part2(sample_file("13")), 37876);
  }
}

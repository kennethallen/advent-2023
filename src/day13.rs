use std::cmp::min;

use itertools::Itertools;
use nom::{IResult, character::complete::{one_of, line_ending}, multi::{many1, separated_list1}, sequence::terminated, combinator::{eof, map, verify}};

pub fn part1(file: String) -> usize {
  parse(file.as_str()).unwrap().1.into_iter()
    .map(|pat| {
      (1..pat.len())
        .find(|&y|
          (0..min(y, pat.len()-y))
            .all(|off| pat[y+off] == pat[y-1-off])
        )
        .map(|y| y*100)
        .or_else(||
          (1..pat[0].len())
            .find(|&x|
              (0..min(x, pat[0].len()-x))
                .all(|off|
                  pat.iter()
                    .all(|row| row[x+off] == row[x-1-off])
                )
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
}

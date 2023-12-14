use crate::util::isize;

use itertools::Itertools;
use nom::{IResult, character::complete::char, multi::separated_list1, sequence::terminated, combinator::eof};

pub fn part1(lines: impl Iterator<Item=String>) -> isize {
  lines
    .map(|line| parse(line.as_str()).unwrap().1)
    .map(|mut ns| {
      let mut sum_lasts = 0;
      loop {
        sum_lasts += ns.last().copied().unwrap();
        if ns.iter().all_equal() { break sum_lasts; }
        ns = ns
          .windows(2)
          .map(|pair| pair[1] - pair[0])
          .collect();
      }
    })
    .sum()
}

pub fn part2(lines: impl Iterator<Item=String>) -> isize {
  lines
    .map(|line| parse(line.as_str()).unwrap().1)
    .map(|mut ns| {
      let mut sum_firsts = 0;
      loop {
        sum_firsts += ns.first().copied().unwrap();
        if ns.iter().all_equal() { break sum_firsts; }
        ns = ns
          .windows(2)
          .map(|pair| pair[1] - pair[0])
          .collect();

        // Negate every other
        sum_firsts -= ns.first().copied().unwrap();
        if ns.iter().all_equal() { break sum_firsts; }
        ns = ns
          .windows(2)
          .map(|pair| pair[1] - pair[0])
          .collect();
      }
    })
    .sum()
}

fn parse(input: &str) -> IResult<&str, Vec<isize>> {
  terminated(
    separated_list1(
      char(' '),
      isize,
    ),
    eof,
  )(input)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_util::sample_lines;

  #[test]
  fn test1_sample() {
    assert_eq!(part1(sample_lines("09a")), 114);
  }

  #[test]
  fn test1() {
    assert_eq!(part1(sample_lines("09")), 1782868781);
  }

  #[test]
  fn test2_sample() {
    assert_eq!(part2(sample_lines("09a")), 2);
  }

  #[test]
  fn test2() {
    assert_eq!(part2(sample_lines("09")), 1057);
  }
}

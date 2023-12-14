use crate::util::isize;

use nom::{IResult, character::complete::char, multi::separated_list1, sequence::terminated, combinator::eof};

pub fn part1(lines: impl Iterator<Item=String>) -> isize {
  lines
    .map(|line| parse(line.as_str()).unwrap().1)
    .map(|ns| {
      let mut levels = vec![ns];
      while !levels.last().unwrap().iter().all(|&n| n == 0) {
        levels.push(levels.last().unwrap()
          .windows(2)
          .map(|pair| pair[1] - pair[0])
          .collect())
      }
      levels.into_iter()
        .filter_map(|ns| ns.last().copied())
        .sum::<isize>()//?
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

  /*#[test]
  fn test2_sample() {
    assert_eq!(part2(sample_lines("09c")), 6);
  }

  #[test]
  fn test2() {
    assert_eq!(part2(sample_lines("09")), 1);
  }*/
}

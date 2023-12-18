use nom::{IResult, character::complete::{char, line_ending}, multi::separated_list0, sequence::{terminated, pair}, combinator::eof, bytes::complete::take_while};

pub fn part1(file: String) -> usize {
  parse(file.as_str()).unwrap().1.into_iter()
    .map(hash)
    .map(Into::<usize>::into)
    .sum()
}

fn hash(s: &str) -> u8 {
  let mut h = 0;
  for c in s.chars() {
    let c: u8 = c.try_into().unwrap();
    h += c;
    h *= 17;
  }
  h
}

fn parse(input: &str) -> IResult<&str, Vec<&str>> {
  terminated(
    separated_list0(
      char(','),
      take_while(|c: char| c.is_alphabetic() || c.is_digit(10) || c == '-' || c == '='),
    ),
    pair(line_ending, eof),
  )(input)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_util::sample_file;

  #[test]
  fn test1_sample() {
    assert_eq!(part1(sample_file("15a")), 1320);
  }

  #[test]
  fn test1() {
    assert_eq!(part1(sample_file("15")), 502139);
  }
}

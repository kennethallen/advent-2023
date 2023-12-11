use crate::util::usize;

use std::collections::{HashSet, VecDeque};

use nom::{IResult, character::complete::char, bytes::complete::tag, multi::{many1, many0}, sequence::{pair, delimited, terminated, preceded}, combinator::eof};

pub fn part1(lines: impl Iterator<Item=String>) -> usize {
  lines
    .map(|line| Card::parse(&line).unwrap().1)
    .map(|c| c.score())
    .sum()
}

pub fn part2(lines: impl Iterator<Item=String>) -> usize {
  let cards = lines
    .map(|line| Card::parse(&line).unwrap().1);

  let mut counts = VecDeque::new();
  let mut sum = 0;
  for (i, card) in cards.enumerate() {
    assert!(card.id == i + 1);
    let count = counts.pop_front().unwrap_or(1);
    sum += count;
    let matches = card.matches();
    while counts.len() < matches {
      counts.push_back(1);
    }
    for j in 0..matches {
      counts[j] += count;
    }
  }
  sum
}

#[derive(Debug)]
struct Card {
  id: usize,
  winners: HashSet<usize>,
  nums: Vec<usize>,
}

impl Card {
  fn parse(input: &str) -> IResult<&str, Self> {
    let spaces = || many1(char(' '));

    let (input, id) = delimited(
      pair(tag("Card"), spaces()),
      usize,
      char(':'),
    )(input)?;
    let (input, winners) = terminated(
      many0(preceded(spaces(), usize)),
      tag(" |"),
    )(input)?;
    let winners = winners.into_iter().collect();
    let (input, nums) = terminated(
      many0(preceded(spaces(), usize)),
      eof,
    )(input)?;
    Ok((input, Self { id, winners, nums }))
  }

  fn matches(&self) -> usize {
    self.nums.iter()
      .filter(|n| self.winners.contains(n))
      .count()
  }

  fn score(&self) -> usize {
    match self.matches() {
      0 => 0,
      n => 1 << n-1,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_util::sample_lines;

  #[test]
  fn test1_sample() {
    assert_eq!(part1(sample_lines("04a")), 13);
  }

  #[test]
  fn test1() {
    assert_eq!(part1(sample_lines("04")), 22897);
  }

  #[test]
  fn test2_sample() {
    assert_eq!(part2(sample_lines("04a")), 30);
  }

  #[test]
  fn test2() {
    assert_eq!(part2(sample_lines("04")), 5095824);
  }
}
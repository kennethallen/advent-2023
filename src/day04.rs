use crate::util::usize;

use std::collections::HashSet;

use nom::{IResult, character::complete::char, bytes::complete::tag, multi::{many1, separated_list0}, sequence::{separated_pair, pair}};

pub fn part1(lines: impl Iterator<Item=String>) -> usize {
  /*let mut cards = lines
    .map(|line| parse(&line).unwrap().1);
  let c0 = cards.next().unwrap();
  assert!(cards.all(|c| c.nums.len() == c0.nums.len() && c.winners.len() == c0.winners.len()));

  1*/
  lines
    .map(|line| parse(&line).unwrap().1)
    .map(|c| score(&c))
    .sum()
}

#[derive(Debug)]
struct Card {
  id: usize,
  winners: HashSet<usize>,
  nums: Vec<usize>,
}

fn parse(input: &str) -> IResult<&str, Card> {
  let spaces = || many1(char(' '));

  let (input, _) = tag("Card")(input)?;
  let (input, _) = spaces()(input)?;
  let (input, id) = usize(input)?;
  let (input, _) = char(':')(input)?;
  let (input, _) = spaces()(input)?;
  let (input, (winners, nums)) = separated_pair(
    separated_list0(spaces(), usize),
    pair(tag(" |"), spaces()),
    separated_list0(spaces(), usize),
  )(input)?;
  assert!(input.is_empty());
  let winners = winners.into_iter().collect();

  Ok((input, Card { id, winners, nums }))
}

fn score(card: &Card) -> usize {
  dbg!(&card, card.nums.iter()
    .filter(|n| card.winners.contains(n))
    .count(), match card.nums.iter()
    .filter(|n| card.winners.contains(n))
    .count() {
    0 => 0,
    n => 1 << n-1,
  });
  match card.nums.iter()
    .filter(|n| card.winners.contains(n))
    .count() {
    0 => 0,
    n => 1 << n-1,
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
}
use std::collections::HashMap;

use nom::{IResult, character::complete::{u32, multispace1, alpha1}, bytes::complete::tag, sequence::separated_pair, multi::separated_list1};

pub fn part1(lines: impl Iterator<Item=String>) -> usize {
  lines
    .map(|line| parse(&line).unwrap().1)
    .filter(|g| g.rounds.iter().all(possible))
    .map(|g| g.id)
    .sum()
}
pub fn part2(lines: impl Iterator<Item=String>) -> usize {
  lines
    .map(|line| parse(&line).unwrap().1)
    .map(|g| power(&g))
    .sum()
}

type Round = HashMap<String, usize>;

#[derive(Debug)]
struct Game {
  id: usize,
  rounds: Vec<Round>,
}

fn parse(input: &str) -> IResult<&str, Game> {
  let (input, _) = tag("Game ")(input)?;
  let (input, id) = u32(input)?;
  let id = id.try_into().unwrap();
  let (input, _) = tag(": ")(input)?;
  let (input, rounds) = separated_list1(
    tag("; "),
    separated_list1(tag(", "), parse_entry),
  )(input)?;
  let rounds = rounds.into_iter()
    .map(|round| round.into_iter().collect())
    .collect();
  Ok((input, Game { id, rounds }))
}
fn parse_entry(input: &str) -> IResult<&str, (String, usize)> {
  let (input, (v, k)) = separated_pair(
    u32,
    multispace1,
    alpha1,
  )(input)?;
  Ok((input, (k.to_owned(), v.try_into().unwrap())))
}

fn possible(round: &Round) -> bool {
  round.iter().all(|(k, &v)|
    match k.as_str() {
      "red"   => v <= 12,
      "green" => v <= 13,
      "blue"  => v <= 14,
      _       => false,
    }
  )
}

fn power(game: &Game) -> usize {
  let mut mins: Round = Default::default();
  for round in &game.rounds {
    for (k, &v) in round {
      mins.entry(k.clone())
        .and_modify(|v0| { *v0 = Ord::max(v, *v0) })
        .or_insert(v);
    }
  }
  ["red", "green", "blue"].iter()
    .map(|&k| mins.get(k).copied().unwrap_or_default())
    .product()
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_util::sample_lines;

  #[test]
  fn test1_sample() {
    assert_eq!(part1(sample_lines("02a")), 8);
  }

  #[test]
  fn test1() {
    assert_eq!(part1(sample_lines("02")), 3099);
  }

  #[test]
  fn test2_sample() {
    assert_eq!(part2(sample_lines("02a")), 2286);
  }

  #[test]
  fn test2() {
    assert_eq!(part2(sample_lines("02")), 72970);
  }
}
use enum_map::{EnumMap, Enum};
use nom::{IResult, character::complete::u32, bytes::complete::tag, sequence::separated_pair, multi::separated_list1, branch::alt};

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

#[derive(Debug, Enum, Clone, Copy)]
enum Color { Red, Green, Blue }

type Round = EnumMap<Color, usize>;

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
fn parse_entry(input: &str) -> IResult<&str, (Color, usize)> {
  let parse_color = alt((
    parse_one_color("red"  , Color::Red  ),
    parse_one_color("green", Color::Green),
    parse_one_color("blue" , Color::Blue ),
  ));
  let (input, (v, k)) = separated_pair(
    u32,
    tag(" "),
    parse_color,
  )(input)?;
  Ok((input, (k, v.try_into().unwrap())))
}
fn parse_one_color(text: &'static str, color: Color) -> impl Fn(&str) -> IResult<&str, Color> {
  move |input| {
    let (input, _) = tag(text)(input)?;
    Ok((input, color))
  }
}

fn possible(round: &Round) -> bool {
     round[Color::Red]   <= 12
  && round[Color::Green] <= 13
  && round[Color::Blue]  <= 14
}

fn power(game: &Game) -> usize {
  let mut mins: Round = Default::default();
  for round in &game.rounds {
    for (k, &v) in round {
      mins[k] = Ord::max(v, mins[k]);
    }
  }
  mins.into_values().product()
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
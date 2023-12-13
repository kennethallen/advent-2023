use std::collections::HashMap;

use enum_map::{enum_map, Enum, EnumMap};
use nom::{IResult, character::complete::{char, one_of, line_ending}, multi::{count, many1}, sequence::{terminated, separated_pair, delimited}, combinator::{eof, map}, bytes::complete::tag};

pub fn part1(file: String) -> usize {
  let (route, map) = parse(file.as_str()).unwrap().1;
  let start = parse_location("AAA").unwrap().1;
  let goal = parse_location("ZZZ").unwrap().1;

  let mut jumps: HashMap<Location, Result<usize, Location>> = map.keys()
    .map(|&jump_start| {
      let mut loc = jump_start;
      for steps in 0..route.len() {
        if loc == goal { return (jump_start, Ok(steps)); }
        loc = map[&loc][route[steps]];
      }
      (jump_start, if loc == goal { Ok(route.len()) } else { Err(loc) })
    })
    .collect();

  let mut jump_size = route.len();
  loop {
    if let Ok(steps) = jumps[&start] {
      break steps;
    }
    jumps = jumps.iter()
      .map(|(&jump_start, &outcome)| (jump_start, match outcome {
        Ok(steps) => Ok(steps),
        Err(jump_end) => match jumps[&jump_end] {
          Ok(steps) => Ok(jump_size + steps),
          Err(new_jump_end) => Err(new_jump_end),
        },
      }))
      .collect();
    jump_size <<= 1;
  }
}

#[derive(Debug, Enum, Clone, Copy)]
enum Dir {
  Left, Right,
}

impl Dir {
  fn parse(input: &str) -> IResult<&str, Self> {
    map(
      one_of("LR"),
      |c| match c {
        'L' => Self::Left,
        'R' => Self::Right,
        _ => unreachable!(),
      },
    )(input)
  }
}

type Location = u32;
type Route = Vec<Dir>;
type Map = HashMap<Location, EnumMap<Dir, Location>>;

fn parse(input: &str) -> IResult<&str, (Route, Map)> {
  terminated(
    separated_pair(
      many1(Dir::parse),
      count(line_ending, 2),
      map(
        many1(parse_map_line),
        |lines| lines.into_iter().collect(),
      )
    ),
    eof,
  )(input)
}

fn parse_location(input: &str) -> IResult<&str, Location> {
  map(
    count(
      map(
        one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
        |c| c as u32 - 'A' as u32,
      ),
      3,
    ),
    |cs| cs[0]*(26*26) + cs[1]*26 + cs[2],
  )(input)
}

fn parse_map_line(input: &str) -> IResult<&str, (Location, EnumMap<Dir, Location>)> {
  map(
    terminated(
      separated_pair(
        parse_location,
        tag(" = "),
        delimited(
          char('('),
          separated_pair(
            parse_location,
            tag(", "),
            parse_location,
          ),
          char(')'),
        ),
      ),
      line_ending,
    ),
    |(start, (left, right))| (start, enum_map! { Dir::Left => left, Dir::Right => right })
  )(input)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_util::sample_file;

  #[test]
  fn test1_sample() {
    assert_eq!(part1(sample_file("08a")), 2);
    assert_eq!(part1(sample_file("08b")), 6);
  }

  #[test]
  fn test1() {
    assert_eq!(part1(sample_file("08")), 19199);
  }

  /*#[test]
  fn test2_sample() {
    assert_eq!(part2(sample_file("08a")), 5905);
  }

  #[test]
  fn test2() {
    assert_eq!(part2(sample_file("08")), 250506580);
  }
  */
}

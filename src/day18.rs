
use std::collections::HashSet;

use nom::{IResult, sequence::{terminated, delimited}, character::complete::one_of, combinator::{eof, map_res, map}, bytes::complete::{tag, take}, character::complete::char};

use crate::util::usize;

pub fn part1(lines: impl Iterator<Item=String>) -> usize { process(lines) }

fn process(lines: impl Iterator<Item=String>) -> usize {
  let mut curs: (isize, isize) = (0, 0);
  let mut trench = HashSet::from([curs]);
  for line in lines {
    let (dir, run, _) = parse(line.as_str()).unwrap().1;
    for _ in 0..run {
      match dir {
        Dir::R => curs.1 += 1,
        Dir::U => curs.0 -= 1,
        Dir::L => curs.1 -= 1,
        Dir::D => curs.0 += 1,
      }
      trench.insert(curs);
    }
  }

  let mins = (
    trench.iter().copied().map(|(y, _)| y).min().unwrap(),
    trench.iter().copied().map(|(_, x)| x).min().unwrap(),
  );
  let maxs = (
    trench.iter().copied().map(|(y, _)| y).max().unwrap(),
    trench.iter().copied().map(|(_, x)| x).max().unwrap(),
  );
  let mut holes = HashSet::new();
  let mut to_check = vec![];
  for x in mins.1..=maxs.1 {
    to_check.push((mins.0, x));
    to_check.push((maxs.0, x));
  }
  for y in mins.0+1..maxs.0 {
    to_check.push((y, mins.1));
    to_check.push((y, maxs.1));
  }
  while let Some(pos) = to_check.pop() {
    if !trench.contains(&pos) && holes.insert(pos) {
      for next in [(pos.0+1, pos.1), (pos.0-1, pos.1), (pos.0, pos.1+1), (pos.0, pos.1-1)] {
        if next.0 >= mins.0 && next.0 <= maxs.0 && next.1 >= mins.1 && next.1 <= maxs.1 {
          to_check.push(next);
        }
      }
    }
  }
  usize::try_from((maxs.1 - mins.1 + 1)*(maxs.0 - mins.0 + 1)).unwrap() - holes.len()
}

fn parse(input: &str) -> IResult<&str, (Dir, usize, u32)> {
  let (input, dir) = terminated(map(one_of("UDLR"), |c| match c { 'U' => Dir::U, 'D' => Dir::D, 'L' => Dir::L, 'R' => Dir::R, _ => unreachable!() }), char(' '))(input)?;
  let (input, run) = terminated(usize, char(' '))(input)?;
  let (input, color) = terminated(parse_color, eof)(input)?;
  Ok((input, (dir, run, color)))
}
fn parse_color(input: &str) -> IResult<&str, u32> {
  delimited(
    tag("(#"),
    map_res(
      take(6usize),
      |s| u32::from_str_radix(s, 16),
    ),
    char(')'),
  )(input)
}

enum Dir { R, U, L, D }

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_util::sample_lines;

  #[test]
  fn test1_sample() {
    assert_eq!(part1(sample_lines("18a")), 62);
  }

  #[test]
  fn test1() {
    assert_eq!(part1(sample_lines("18")), 58550);
  }
}

use crate::util::usize;

use nom::{IResult, character::complete::{char, line_ending, alpha1}, multi::separated_list0, sequence::{terminated, pair, preceded}, combinator::{eof, map}, bytes::complete::take_while, branch::alt};

pub fn part1(file: String) -> usize {
  parse1(&file).unwrap().1.into_iter()
    .map(hash)
    .map(Into::<usize>::into)
    .sum()
}

pub fn part2(file: String) -> usize {
  let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];
  for (label, op) in parse2(&file).unwrap().1 {
    let box_num: usize = hash(label).into();
    let b = &mut boxes[box_num];
    match (op, b.iter_mut().enumerate().find(|(_, slot)| slot.0 == label)) {
      (None, None) => (),
      (None, Some((i, _))) => { b.remove(i); },
      (Some(focal_length), None) => b.push((label, focal_length)),
      (Some(focal_length), Some((_, slot))) => slot.1 = focal_length,
    }
  }
  boxes.into_iter()
    .enumerate()
    .flat_map(|(box_num, b)| b.into_iter()
      .enumerate()
      .map(move |(lens_num, (_, focal_length))| (box_num + 1)*(lens_num + 1)*focal_length))
    .sum()
}

fn hash(s: &str) -> u8 {
  let mut h: u8 = 0;
  for c in s.chars() {
    let c: u8 = c.try_into().unwrap();
    h = h.overflowing_add(c).0.overflowing_mul(17).0;
  }
  h
}

fn parse1(input: &str) -> IResult<&str, Vec<&str>> {
  terminated(
    separated_list0(
      char(','),
      take_while(|c: char| c.is_alphabetic() || c.is_digit(10) || c == '-' || c == '='),
    ),
    pair(line_ending, eof),
  )(input)
}

fn parse2(input: &str) -> IResult<&str, Vec<(&str, Option<usize>)>> {
  terminated(
    separated_list0(
      char(','),
      pair(
        alpha1,
        alt((
          map(char('-'), |_| None),
          map(preceded(char('='), usize), |n| Some(n)),
        )),
      )
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

  #[test]
  fn test2_sample() {
    assert_eq!(part2(sample_file("15a")), 145);
  }

  #[test]
  fn test2() {
    assert_eq!(part2(sample_file("15")), 284132);
  }
}

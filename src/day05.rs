use std::{collections::BTreeMap, ops::Bound};

use crate::util::usize;

use nom::{IResult, character::complete::{char, line_ending}, bytes::complete::tag, multi::{separated_list1, many0}, sequence::{preceded, terminated, tuple}, combinator::eof};

pub fn part1(file: String) -> usize {
  let (_, (seeds, maps)) = parse(file.as_str()).unwrap();

  seeds.into_iter()
    .map(|x| maps.iter().fold(x, |x, m| m.translate(x)))
    .min().unwrap()
}

fn parse(input: &str) -> IResult<&str, (Vec<usize>, Vec<Map>)> {
  let (mut input, seeds) = terminated(parse_seeds, line_ending)(input)?;
  let mut maps = vec![];
  for elems in ["seed", "soil", "fertilizer", "water", "light", "temperature", "humidity", "location"].windows(2) {
    let (input1, map) = preceded(
      tuple((
        line_ending,
        tag(format!("{}-to-{} map:", elems[0], elems[1]).as_str()),
        line_ending,
      )),
      Map::parse,
    )(input)?;
    input = input1;
    maps.push(map);
  }
  let (input, _) = eof(input)?;
  Ok((input, (seeds, maps)))
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<usize>> {
  preceded(
    tag("seeds: "),
    separated_list1(char(' '), usize),
  )(input)
}

#[derive(Default, Debug)]
struct Map {
  inner: BTreeMap<usize, (usize, usize)>,
}

impl Map {
  fn parse(input: &str) -> IResult<&str, Self> {
    let (input, entries) = many0(tuple((
      terminated(usize, char(' ')),
      terminated(usize, char(' ')),
      terminated(usize, line_ending),
    )))(input)?;
    Ok((input, Self { inner: entries.into_iter().map(|(off, start, len)| (start, (off, len))).collect() }))
  }

  fn translate(&self, x: usize) -> usize {
    if let Some((start, (off, len))) = self.inner.upper_bound(Bound::Included(&x)).key_value() {
      if x < start + len {
        return x - start + off;
      }
    }
    x
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_util::sample_file;

  #[test]
  fn test1_sample() {
    assert_eq!(part1(sample_file("05a")), 35);
  }

  #[test]
  fn test1() {
    assert_eq!(part1(sample_file("05")), 84470622);
  }

  /*
  #[test]
  fn test2_sample() {
    assert_eq!(part2(sample_lines("04a")), 30);
  }

  #[test]
  fn test2() {
    assert_eq!(part2(sample_lines("04")), 5095824);
  }
  */
}
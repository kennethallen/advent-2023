use std::{collections::BTreeMap, ops::Bound};

use crate::util::usize;

use nom::{IResult, character::complete::{char, line_ending}, bytes::complete::tag, multi::{many0, many1}, sequence::{preceded, terminated, tuple}, combinator::eof};

pub fn part1(file: String) -> usize {
  let (_, (seed_ranges, maps)) = parse(file.as_str()).unwrap();

  let mut ranges: Box<dyn Iterator<Item=Range>> = Box::new(seed_ranges.into_iter());
  for map in &maps {
    ranges = Box::new(ranges.flat_map(|r| map.translate(r)));
  }
  ranges.map(|t| t.0).min().unwrap()
}

fn parse(input: &str) -> IResult<&str, (Vec<Range>, Vec<Map>)> {
  let (mut input, seeds) = terminated(parse_seeds, line_ending)(input)?;
  let seeds = seeds.into_iter().map(|i| (i, 1)).collect();
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
    tag("seeds:"),
    many1(preceded(char(' '), usize)),
  )(input)
}

type Range = (usize, usize);

#[derive(Default, Debug)]
struct Map {
  inner: BTreeMap<usize, Range>,
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

  fn translate(&self, (mut in_start, mut in_len): Range) -> Vec<Range> {
    // TODO implement custom iterator
    let mut outs = vec![];
    let mut curs = self.inner.upper_bound(Bound::Included(&in_start));
    while let Some((&map_start, &(map_off, map_len))) = curs.key_value() {
      if in_start < map_start { // Gap before next map entry
        let out_len = Ord::min(in_len, map_start - in_start);
        outs.push((in_start, out_len));
        in_start += out_len;
        in_len -= out_len;
        if in_len == 0 { break; }
      } else {
        if in_start < map_start + map_len {
          let out_len = Ord::min(in_len, map_len - (in_start - map_start));
          outs.push((in_start - map_start + map_off, out_len));
          in_start += out_len;
          in_len -= out_len;
          if in_len == 0 { break; }
        }
        curs.move_next();
      }
    }
    if in_len > 0 { outs.push((in_start, in_len)); }
    outs
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
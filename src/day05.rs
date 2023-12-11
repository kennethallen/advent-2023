use std::{collections::BTreeMap, ops::Bound};

use crate::util::usize;

use nom::{IResult, character::complete::{char, line_ending}, bytes::complete::tag, multi::{many0, many1}, sequence::{delimited, preceded, terminated, tuple, separated_pair}, combinator::eof, Parser};

pub fn part1(file: String) -> usize { process(file, parse_single_seeds) }
pub fn part2(file: String) -> usize { process(file, parse_seed_ranges) }

fn process(file: String, parse_seeds: impl FnMut(&str) -> IResult<&str, Vec<Range>>) -> usize {
  let (_, (seed_ranges, maps)) = parse(file.as_str(), parse_seeds).unwrap();

  let mut ranges: Box<dyn Iterator<Item=Range>> = Box::new(seed_ranges.into_iter());
  for map in &maps {
    ranges = Box::new(ranges.flat_map(|r| map.translate(r)));
  }
  ranges.map(|t| t.0).min().unwrap()
}

fn parse(input: &str, parse_seeds: impl FnMut(&str) -> IResult<&str, Vec<Range>>) -> IResult<&str, (Vec<Range>, Vec<Map>)> {
  let (mut input, seeds) = delimited(
    tag("seeds:"),
    parse_seeds,
    line_ending,
  )(input)?;
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

fn parse_single_seeds(input: &str) -> IResult<&str, Vec<Range>> {
  many1(preceded(char(' '), usize))
    .map(|seeds| seeds.into_iter().map(|i| (i, 1)).collect())
    .parse(input)
}
fn parse_seed_ranges(input: &str) -> IResult<&str, Vec<Range>> {
  many1(preceded(char(' '), separated_pair(usize, char(' '), usize)))(input)
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

  fn translate(&self, (mut in_start, mut in_len): Range) -> impl IntoIterator<Item=Range> {
    // TODO implement custom iterator
    let mut outs = vec![];
    let mut curs = self.inner.upper_bound(Bound::Included(&in_start));
    // Until we go off the high end of the mappings...
    'scan: {
      while let Some((&map_start, &(map_off, map_len))) = curs.key_value() {
        if in_start < map_start {
          // There is a gap before the next mapping. This will never happen on the first iteration. Identity-map until that one starts
          let out_len = map_start - in_start;
          if in_len < out_len {
            // There is room before the next mapping for the remaining range
            outs.push((in_start, in_len));
            break 'scan;
          }
          outs.push((in_start, out_len));
          in_start += out_len;
          in_len -= out_len;
        }

        if in_start < map_start + map_len {
          // This mapping intersects with our remaining range
          let out_len = map_len - (in_start - map_start);
          let out_start = in_start - map_start + map_off;
          if in_len < out_len {
            // There is room in this mapping for the remaining range
            outs.push((out_start, in_len));
            break 'scan;
          }
          outs.push((out_start, out_len));
          in_start += out_len;
          in_len -= out_len;
        }
        // We're done with this mapping. Move to the next higher one
        curs.move_next();
      }
      // We are off the high end of the mappings. Identity-map the remaining range
      outs.push((in_start, in_len));
    }
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

  #[test]
  fn test2_sample() {
    assert_eq!(part2(sample_file("05a")), 46);
  }

  #[test]
  fn test2() {
    assert_eq!(part2(sample_file("05")), 26714516);
  }
}

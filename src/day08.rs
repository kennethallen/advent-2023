use std::{collections::{HashMap, HashSet}, mem::{swap, take}};

use enum_map::{enum_map, Enum, EnumMap};
use itertools::iproduct;
use nom::{IResult, character::complete::{char, one_of, line_ending}, multi::{count, many1}, sequence::{terminated, separated_pair, delimited}, combinator::{eof, map}, bytes::complete::tag};
use num::Integer;

pub fn part1(file: String) -> usize {
  let (route, map) = parse(&file).unwrap().1;
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

#[derive(Default, Clone)]
struct SparseRange {
  vals: HashSet<usize>,
  len: usize,
}
impl SparseRange {
  fn append(&mut self, other: &Self) {
    self.vals.extend(other.vals.iter().map(|n| n + self.len));
    self.len += other.len;
  }

  fn rotate_left(&mut self, shift: usize) {
    let old_ends = take(&mut self.vals);
    self.vals.extend(old_ends.into_iter()
      .map(|end| (end + self.len - shift) % self.len));
  }

  fn echo(&mut self, n: usize) {
    let old_ends = take(&mut self.vals);
    self.vals.extend(iproduct!(old_ends, 0..n)
      .map(|(end, rep)| rep*self.len + end));
    self.len *= n;
  }
}

#[derive(Clone)]
struct CyclicSet {
  prefix: SparseRange,
  cycle: SparseRange,
}

impl CyclicSet {
  fn contains(&self, end: usize) -> bool {
    if end < self.prefix.len {
      self.prefix.vals.contains(&end)
    } else {
      self.cycle.vals.contains(&((end - self.prefix.len) % self.cycle.len))
    }
  }

  fn intersection(mut self, mut other: Self) -> Self {
    // Ensure self has the longer (or equal) prefix
    if self.prefix.len < other.prefix.len {
      swap(&mut self, &mut other);
    }
    // Intersect prefix with other whole set
    self.prefix.vals.retain(|&end| other.contains(end));
    // Rotate the other's cycle as if we had matched prefix lengths. From now on, other's prefix is invalid
    other.cycle.rotate_left((self.prefix.len - other.prefix.len) % other.cycle.len);

    // Expand the cycle length to the least common multiple of the two cycle lengths and fill duplicates to keep equivalence
    self.cycle.echo(self.cycle.len.lcm(&other.cycle.len) / self.cycle.len);
    // Intersect cycle with other cycle
    self.cycle.vals.retain(|end| other.cycle.vals.contains(&(end % other.cycle.len)));

    self
  }

  fn min(&self) -> Option<usize> {
    self.prefix.vals.iter().next().copied()
      .or(self.cycle.vals.iter().next().copied().map(|end| end + self.prefix.len))
  }
}

pub fn part2(file: String) -> usize {
  let (route, map) = parse(&file).unwrap().1;

  let jumps: HashMap<_, _> = map.keys()
    .map(|&jump_start| {
      let mut loc = jump_start;
      let mut ends = HashSet::new();
      for steps in 0..route.len() {
        if is_ghost_end(&loc) {
          ends.insert(steps);
        }
        loc = map[&loc][route[steps]];
      }
      (jump_start, (loc, SparseRange { vals: ends, len: route.len() }))
    })
    .collect();

  let sparse_range = |s: &[Location]|
    s.iter().fold(SparseRange::default(), |mut ei, l| { ei.append(&jumps[l].1); ei });
  let end_profiles: HashMap<Location, CyclicSet> = map.keys()
    .copied()
    .filter(is_ghost_start)
    .map(|start| {
      let mut loc = start;
      let mut path = vec![];
      let mut visited = HashMap::new();
      loop {
        if let Err(occupied) = visited.try_insert(loc, path.len()) {
          let (prefix, cycle) = path.split_at(*occupied.entry.get());
          break (start, CyclicSet { prefix: sparse_range(prefix), cycle: sparse_range(cycle) });
        }
        path.push(loc);
        loc = jumps[&loc].0;
      }
    })
    .collect();

  end_profiles.into_values()
    .reduce(CyclicSet::intersection)
    .and_then(|end_prof| end_prof.min())
    .unwrap()
}

#[derive(Enum, Clone, Copy)]
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

fn is_ghost_start(loc: &Location) -> bool {
  *loc % 26 == 0
}
fn is_ghost_end(loc: &Location) -> bool {
  *loc % 26 == 25
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

  #[test]
  fn test2_sample() {
    assert_eq!(part2(sample_file("08c")), 6);
  }

  #[test]
  fn test2() {
    assert_eq!(part2(sample_file("08")), 13663968099527);
  }
}

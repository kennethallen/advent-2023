use std::{collections::{HashMap, BinaryHeap}, cmp::Reverse};

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

#[derive(Default, Debug)]
struct EndsInterval {
  ends: Vec<usize>,
  len: usize,
}
impl EndsInterval {
  fn append(&mut self, other: &Self) {
    self.ends.extend(other.ends.iter().map(|n| n + self.len));
    self.len += other.len;
  }
}

#[derive(Debug)]
struct GhostEnds {
  prefix: EndsInterval,
  cycle: EndsInterval,
}

impl IntoIterator for GhostEnds {
  type Item = usize;
  type IntoIter = GhostEndsIter;
  fn into_iter(self) -> Self::IntoIter {
    GhostEndsIter {
      data: self,
      offset: Default::default(),
      next_idx: Default::default(),
      in_cycle: Default::default(),
    }
  }
}

struct GhostEndsIter {
  data: GhostEnds,
  offset: usize,
  next_idx: usize,
  in_cycle: bool,
}

impl Iterator for GhostEndsIter {
  type Item = usize;

  fn next(&mut self) -> Option<Self::Item> {
    let mut cur_ends = &(if self.in_cycle { &self.data.cycle } else { &self.data.prefix }).ends;
    if self.next_idx >= cur_ends.len() {
      self.next_idx = 0;
      self.offset += (if self.in_cycle { &self.data.cycle } else { &self.data.prefix }).len;
      self.in_cycle = true;
      cur_ends = &self.data.cycle.ends;
    }
    let elem = cur_ends[self.next_idx] + self.offset;
    self.next_idx += 1;
    Some(elem)
  }
}

pub fn part2(file: String) -> usize {
  let (route, map) = parse(file.as_str()).unwrap().1;

  let jumps: HashMap<_, _> = map.keys()
    .map(|&jump_start| {
      let mut loc = jump_start;
      let mut ends = vec![];
      for steps in 0..route.len() {
        if is_ghost_end(&loc) {
          ends.push(steps);
        }
        loc = map[&loc][route[steps]];
      }
      (jump_start, (loc, EndsInterval { ends, len: route.len() }))
    })
    .collect();

  let subpath_interval = |s: &[Location]|
    s.iter().fold(EndsInterval::default(), |mut ei, l| { ei.append(&jumps[l].1); ei });
  let end_profiles: HashMap<Location, GhostEnds> = map.keys()
    .cloned()
    .filter(is_ghost_start)
    .map(|start| {
      let mut loc = start;
      let mut path = vec![];
      let mut visited = HashMap::new();
      loop {
        if let Err(occupied) = visited.try_insert(loc, path.len()) {
          break (start, GhostEnds {
            prefix: subpath_interval(&path[0..*occupied.entry.get()]),
            cycle: subpath_interval(&path[*occupied.entry.get()..]),
          });
        }
        path.push(loc);
        loc = jumps[&loc].0;
      }
    })
    .collect();

  //TODO Chinese remainder theorem solution?
  let mut end_iters: Vec<_> = end_profiles.into_values().map(IntoIterator::into_iter).collect();
  let mut queue: BinaryHeap<_> = end_iters.iter_mut().enumerate().map(|(idx, iter)| (Reverse(iter.next().unwrap()), idx)).collect();
  loop {
    let (Reverse(lowest_end), lowest_idx) = queue.pop().unwrap();
    if queue.iter().all(|&(Reverse(end), _)| end == lowest_end) {
      break lowest_end;
    }
    queue.push((Reverse(end_iters[lowest_idx].next().unwrap()), lowest_idx));
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

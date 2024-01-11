use std::{iter::once, collections::{HashMap, HashSet}, mem::swap, cmp::Ordering};

use crate::util::usize;

use itertools::Itertools;
use nom::{IResult, character::complete::char, multi::separated_list1, combinator::{map_res, map}, sequence::separated_pair};

pub fn part1(lines: impl Iterator<Item=String>) -> usize {
  let mut state = State::parse(lines);
  state.settle();
  state.bricks.iter()
    .filter(|&brick|
      brick.supports.iter().all(|&other| state.bricks[other].supported_by.len() > 1))
    .count()
}

pub fn part2(lines: impl Iterator<Item=String>) -> usize {
  let mut state = State::parse(lines);
  state.settle();
  (0..state.bricks.len())
    .map(|i| {
      let mut scen = state.clone();
      scen.vaporize(i);
      scen.settle().len()
    })
    .sum()
}

type Coord = [usize; 3];

#[derive(Clone)]
struct State {
  bricks: Vec<Brick>,
  cols: HashMap<[usize; 2], Vec<usize>>,
  unsupported: HashSet<usize>,
}

impl State {
  fn parse(lines: impl Iterator<Item=String>) -> Self {
    let mut bricks: Vec<_> = lines.map(|line| Brick::parse(&line).unwrap().1).collect();

    let mut cols = HashMap::new();
    for (i, brick) in bricks.iter().enumerate() {
      for xz in brick.iter_xys() {
        (match cols.try_insert(xz, vec![]) {
          Ok(v) => v,
          Err(e) => e.entry.into_mut(),
        }).push(i);
      }
    }
    for col in cols.values_mut() {
      col.sort_by_key(|&i| bricks[i].pos[2]);
    }
    let cols = cols;

    for pair in cols.values().flat_map(|col| col.windows(2)) {
      let lo = pair[0];
      let hi = pair[1];
      if bricks[lo].just_beneath(&bricks[hi]) {
        bricks[lo].supports.insert(hi);
        bricks[hi].supported_by.insert(lo);
      }
    }
    let unsupported: HashSet<_> = bricks.iter().positions(Brick::can_fall).collect();
    
    Self { bricks, cols, unsupported }
  }

  fn settle(&mut self) -> HashSet<usize> {
    let mut drops = self.unsupported.clone();
    while let Some(&unsup) = self.unsupported.iter().next() {
      self.unsupported.remove(&unsup);
      debug_assert!(self.bricks[unsup].can_fall());

      // Find y to drop to
      let mut floor = 0;
      let mut sups = HashSet::new();
      for xy in self.bricks[unsup].iter_xys() {
        let col = &self.cols[&xy];
        let col_i = col.iter().position(|&n| n == unsup).unwrap();
        if col_i > 0 {
          let below = col[col_i - 1];
          let new_floor = self.bricks[below].max_z();
          match new_floor.cmp(&floor) {
            Ordering::Greater => {
              floor = new_floor;
              sups.clear();
              sups.insert(below);
            },
            Ordering::Equal => { sups.insert(below); },
            Ordering::Less => (),
          }
        }
      }
      debug_assert!(floor < self.bricks[unsup].pos[2]);
      debug_assert_eq!(floor == 0, sups.is_empty());

      // Remove old supports
      while let Some(&supportee) = self.bricks[unsup].supports.iter().next() {
        self.bricks[unsup].supports.remove(&supportee);
        self.bricks[supportee].supported_by.remove(&unsup);
        if self.bricks[supportee].can_fall() {
          self.unsupported.insert(supportee);
          drops.insert(supportee);
        }
      }

      // Move and add new supports
      self.bricks[unsup].pos[2] = floor + 1;
      for &sup in &sups {
        self.bricks[sup].supports.insert(unsup);
      }
      self.bricks[unsup].supported_by = sups;
    }
    drops
  }

  fn vaporize(&mut self, vap_i: usize) {
    // Remove vaporized brick
    let vap = self.bricks.swap_remove(vap_i);
    let new_i = vap_i;
    let old_i = self.bricks.len();

    for xy in vap.iter_xys() {
      let col = self.cols.get_mut(&xy).unwrap();
      col.remove(col.iter().position(|&i| i == vap_i).unwrap());
    }
    for &supporter in &vap.supported_by {
      let supporter = if supporter == old_i { new_i } else { supporter };
      self.bricks[supporter].supports.remove(&vap_i);
    }
    for &supported in &vap.supports {
      let supported = if supported == old_i { new_i } else { supported };
      self.bricks[supported].supported_by.remove(&vap_i);
      if self.bricks[supported].can_fall() { self.unsupported.insert(supported); }
    }

    // Reassign reused index
    if old_i != new_i {
      for xy in self.bricks[new_i].iter_xys() {
        let col = self.cols.get_mut(&xy).unwrap();
        let col_i = col.iter().position(|&i| i == old_i).unwrap();
        col[col_i] = new_i;
      }
      for supporter in self.bricks[new_i].supported_by.clone() {
        self.bricks[supporter].supports.remove(&old_i);
        self.bricks[supporter].supports.insert(new_i);
      }
      for supported in self.bricks[new_i].supports.clone() {
        self.bricks[supported].supported_by.remove(&old_i);
        self.bricks[supported].supported_by.insert(new_i);
      }
    }
  }
}

#[derive(Clone, Debug)]
struct Brick {
  pos: Coord,
  len: usize,
  dim: usize,
  supports: HashSet<usize>,
  supported_by: HashSet<usize>,
}

impl Brick {
  fn iter_xys<'a>(&'a self) -> Box<dyn Iterator<Item=[usize; 2]> + 'a> {
    match self.dim {
      0 => Box::new((0..self.len).map(|x| [x + self.pos[0], self.pos[1]])),
      1 => Box::new((0..self.len).map(|y| [self.pos[0], y + self.pos[1]])),
      2 => Box::new(once([self.pos[0], self.pos[1]])),
      _ => unreachable!(),
    }
  }

  fn just_beneath(&self, other: &Self) -> bool {
    self.pos[2] + 1 == other.pos[2]
  }

  fn max_z(&self) -> usize {
    if self.dim == 2 { self.pos[2] + self.len - 1 } else { self.pos[2] }
  }

  fn can_fall(&self) -> bool {
    self.pos[2] > 1 && self.supported_by.is_empty()
  }

  fn parse(input: &str) -> IResult<&str, Self> {
    map(
      separated_pair(parse_coord, char('~'), parse_coord),
      |(mut c0, mut c1)| {
        let dim = (0..3).find(|&i| c0[i] != c1[i]).unwrap_or_default();
        if c0[dim] > c1[dim] { swap(&mut c0, &mut c1); }
        Self { pos: c0, len: c1[dim] - c0[dim] + 1, dim, supports: HashSet::new(), supported_by: HashSet::new() }
      },
    )(input)
  }
}

fn parse_coord(input: &str) -> IResult<&str, Coord> {
  map_res(
    separated_list1(char(','), usize),
    |ns| ns.try_into(),
  )(input)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_util::sample_lines;

  #[test]
  fn test1_sample() {
    assert_eq!(part1(sample_lines("22a")), 5);
  }

  #[test]
  fn test1() {
    assert_eq!(part1(sample_lines("22")), 522);
  }

  #[test]
  fn test2_sample() {
    assert_eq!(part2(sample_lines("22a")), 7);
  }

  #[test]
  fn test2() {
    assert_eq!(part2(sample_lines("22")), 83519);
  }
}

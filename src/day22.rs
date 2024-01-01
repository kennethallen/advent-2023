use std::{iter::once, collections::{HashMap, HashSet}, mem::swap, cmp::Ordering};

use crate::util::usize;

use itertools::Itertools;
use nom::{IResult, character::complete::char, multi::separated_list1, combinator::{map_res, map}, sequence::separated_pair};

/*fn assert_invariants(bricks: &Vec<Brick>, cols: &HashMap<[usize; 2], Vec<usize>>, unsupported: &HashSet<usize>) {
  for (i, brick) in bricks.iter().enumerate() {
    assert!(brick.pos[2] > 0);

    for (j, b1) in bricks.iter().enumerate() {
      assert_eq!(i == j, brick.intersects(&b1), "{} {:?}\n{} {:?}", i, brick, j, b1);
    }

    for &supporter in &bricks[i].supported_by {
      assert!(bricks[supporter].supports.contains(&i), "{} {:?}\n{} {:?}", i, brick, supporter, &bricks[supporter]);
      assert_eq!(bricks[supporter].max_z() + 1, brick.pos[2]);
    }
    for &supportee in &bricks[i].supports {
      assert!(bricks[supportee].supported_by.contains(&i));
      assert_eq!(bricks[supportee].pos[2], brick.max_z() + 1);
    }
  }

  for col in cols.values() {
    assert!(col.iter().map(|&i| bricks[i].pos[2]).is_sorted());
  }
}*/

pub fn part1(lines: impl Iterator<Item=String>) -> usize {
  let mut bricks: Vec<_> = lines.map(|line| Brick::parse(line.as_str()).unwrap().1).collect();

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

  let mut unsupported: HashSet<_> = bricks.iter().positions(Brick::can_fall).collect();
  for pair in cols.values().flat_map(|col| col.windows(2)) {
    let lo = pair[0];
    let hi = pair[1];
    if bricks[lo].just_beneath(&bricks[hi]) {
      bricks[lo].supports.insert(hi);
      bricks[hi].supported_by.insert(lo);
      unsupported.remove(&hi);
    }
  }

  //assert_invariants(&bricks, &cols, &unsupported);
  while let Some(&unsup) = unsupported.iter().next() {
    unsupported.remove(&unsup);
    debug_assert!(bricks[unsup].can_fall());

    // Find y to drop to
    let mut floor = 0;
    let mut sups = HashSet::new();
    for xy in bricks[unsup].iter_xys() {
      let col = &cols[&xy];
      let col_i = col.iter().position(|&n| n == unsup).unwrap();
      if col_i > 0 {
        let below = col[col_i - 1];
        let new_floor = bricks[below].max_z();
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
    debug_assert!(floor < bricks[unsup].pos[2]);
    debug_assert_eq!(floor == 0, sups.is_empty());

    // Remove old supports
    while let Some(&supportee) = bricks[unsup].supports.iter().next() {
      bricks[unsup].supports.remove(&supportee);
      bricks[supportee].supported_by.remove(&unsup);
      if bricks[supportee].can_fall() {
        unsupported.insert(supportee);
      }
    }

    // Move and add new supports
    bricks[unsup].pos[2] = floor + 1;
    for &sup in &sups {
      bricks[sup].supports.insert(unsup);
    }
    bricks[unsup].supported_by = sups;

    //assert_invariants(&bricks, &cols, &unsupported);
  }

  /*for z in (0..=6).rev() {
    for x in 0..=2 {
      let bs: Vec<_> = bricks.iter().positions(|b| b.intersects_subspace([Some(x), None, Some(z)])).collect();
      print!("{}", match bs.len() {
        0 => '.',
        1 => ['A', 'B', 'C', 'D', 'E', 'F', 'G'][bs[0]],
        _ => '?',
      });
    }
    println!();
  }
  println!();

  for z in (0..=6).rev() {
    for y in 0..=2 {
      let bs: Vec<_> = bricks.iter().positions(|b| b.intersects_subspace([None, Some(y), Some(z)])).collect();
      print!("{}", match bs.len() {
        0 => '.',
        1 => ['A', 'B', 'C', 'D', 'E', 'F', 'G'][bs[0]],
        _ => '?',
      });
    }
    println!();
  }
  println!();*/

  bricks.iter()
    .filter(|&brick|
      brick.supports.iter().all(|&other| bricks[other].supported_by.len() > 1))
    .count()
}

type Coord = [usize; 3];

#[derive(Debug)]
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

  fn intersects_subspace(&self, pat: [Option<usize>; 3]) -> bool {
    (0..self.pos.len()).all(|dim|
      match pat[dim] {
        Some(spec) => 
          if dim == self.dim {
            spec >= self.pos[dim] && spec < self.pos[dim] + self.len
          } else {
            spec == self.pos[dim]
          },
        None => true,
      }
    )
  }

  fn intersects(&self, other: &Self) -> bool {
    (0..self.pos.len()).all(|dim|
      match (dim == self.dim, dim == other.dim) {
        (true, true) => self.pos[dim] < other.pos[dim] + other.len && self.pos[dim] + self.len > other.pos[dim],
        (true, false) => self.pos[dim] <= other.pos[dim] && self.pos[dim] + self.len > other.pos[dim],
        (false, true) => other.pos[dim] <= self.pos[dim] && other.pos[dim] + other.len > self.pos[dim],
        (false, false) => self.pos[dim] == other.pos[dim],
      }
    )
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
}

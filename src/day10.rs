use std::mem::swap;

use bitvec::prelude::*;
use itertools::{Itertools, iproduct};
use strum::{EnumIter, IntoEnumIterator};

use crate::util::Coord;

pub fn part1(lines: impl Iterator<Item=String>) -> usize {
  let (map, map_size, start) = prep(lines);

  let mut dir = map[start.0][start.1].connects()[0];
  let mut path_len = 0;
  let mut pos = start;
  loop {
    pos = dir.try_step(pos, map_size).unwrap();
    path_len += 1;
    if pos == start { break; }
    dir = dir.try_turn(map[pos.0][pos.1]).unwrap();
  }

  path_len / 2
}

pub fn part2(lines: impl Iterator<Item=String>) -> usize {
  let (map, map_size, _) = prep(lines);

  let mut interpoints = vec![bitvec![0; map[0].len() + 1]; map.len() + 1];
  let interpoints_size = (map_size.0 + 1, map_size.1 + 1);
  interpoints[0].set(0, true);
  let mut to_visit = vec![(0, 0)];

  while let Some(pos) = to_visit.pop() {
    for dir in Dir::iter() {
      if let Some(nbr) = dir.interpoints_try_step(pos, interpoints_size, &map) {
        if !interpoints[nbr.0][nbr.1] {
          interpoints[nbr.0].set(nbr.1, true);
          to_visit.push(nbr);
        }
      }
    }
  }

  iproduct!(0..map_size.0, 0..map_size.1)
    .filter(|&(y, x)| iproduct!(y..=y+1, x..=x+1)
      .all(|(iy, ix)| !interpoints[iy][ix]))
    .count()
}

fn prep(lines: impl Iterator<Item=String>) -> (Vec<Vec<Tile>>, Coord, Coord) {
  let mut map: Vec<Vec<_>> = lines
    .map(|s| s.chars().map(Tile::try_parse).map(Option::unwrap).collect())
    .collect();
  let map_size = (map.len(), map[0].len());

  let start = map.iter()
    .enumerate()
    .flat_map(|(y, row)| row.iter()
      .enumerate()
      .filter(|(_, &c)| c == Tile::Start)
      .map(move |(x, _)| (y, x)))
    .exactly_one().unwrap();
  let dirs: Vec<_> = Dir::iter()
    .filter(|d|
      d.try_step(start, map_size)
        .and_then(|(y, x)| d.try_turn(map[y][x]))
        .is_some()
    )
    .collect();
  assert_eq!(dirs.len(), 2);
  map[start.0][start.1] = Tile::connecting(dirs[0], dirs[1]);

  (map, map_size, start)
}

#[derive(EnumIter, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Dir { E, N, W, S }

impl Dir {
  fn try_step(&self, (y, x): Coord, (max_y, max_x): Coord) -> Option<Coord> {
    match self {
      Self::E => x.checked_add(1).map(|x| (y, x)).filter(|&(_, x)| x < max_x),
      Self::N => y.checked_sub(1).map(|y| (y, x)),
      Self::W => x.checked_sub(1).map(|x| (y, x)),
      Self::S => y.checked_add(1).map(|y| (y, x)).filter(|&(y, _)| y < max_y),
    }
  }

  fn interpoints_try_step(&self, (y, x): Coord, interpoints_size: Coord, map: &Vec<Vec<Tile>>) -> Option<Coord> {
    let dest = self.try_step((y, x), interpoints_size)?;
    let map_size = (interpoints_size.0 - 1, interpoints_size.1 - 1);
    let valid_tile = |&(y, x): &_| y < map_size.0 && x < map_size.1;
    let between_tiles = || {
      let y_m1 = y.checked_sub(1);
      let x_m1 = x.checked_sub(1);
      let se = Some((y, x)).filter(valid_tile);
      let ne = y_m1.map(|y| (y, x)).filter(valid_tile);
      let sw = x_m1.map(|x| (y, x)).filter(valid_tile);
      let nw = y_m1.and_then(|y| x_m1.map(|x| (y, x))).filter(valid_tile);
      Some(match self {
        Self::E => (ne?, Self::S, se?, Self::N),
        Self::N => (nw?, Self::E, ne?, Self::W),
        Self::W => (sw?, Self::N, nw?, Self::S),
        Self::S => (se?, Self::W, sw?, Self::E),
      })
    };
    if let Some((t0, d0, t1, d1)) = between_tiles() {
      if map[t0.0][t0.1].connects().contains(&d0) && map[t1.0][t1.1].connects().contains(&d1) {
        None
      } else {
        Some(dest)
      }
    } else {
      Some(dest)
    }
  }

  fn try_turn(&self, t: Tile) -> Option<Self> {
    match (self, t) {
      (Self::E, Tile::PipeEW) => Some(Self::E),
      (Self::E, Tile::PipeNW) => Some(Self::N),
      (Self::E, Tile::PipeWS) => Some(Self::S),
      (Self::N, Tile::PipeNS) => Some(Self::N),
      (Self::N, Tile::PipeSE) => Some(Self::E),
      (Self::N, Tile::PipeWS) => Some(Self::W),
      (Self::W, Tile::PipeEW) => Some(Self::W),
      (Self::W, Tile::PipeEN) => Some(Self::N),
      (Self::W, Tile::PipeSE) => Some(Self::S),
      (Self::S, Tile::PipeNS) => Some(Self::S),
      (Self::S, Tile::PipeEN) => Some(Self::E),
      (Self::S, Tile::PipeNW) => Some(Self::W),
      _ => None,
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tile { Empty, Start, PipeEW, PipeNS, PipeEN, PipeNW, PipeWS, PipeSE }

impl Tile {
  fn try_parse(c: char) -> Option<Self> {
    match c {
      '.' => Some(Self::Empty),
      'S' => Some(Self::Start),
      '-' => Some(Self::PipeEW),
      '|' => Some(Self::PipeNS),
      'L' => Some(Self::PipeEN),
      'J' => Some(Self::PipeNW),
      '7' => Some(Self::PipeWS),
      'F' => Some(Self::PipeSE),
      _ => None,
    }
  }

  fn connecting(mut d0: Dir, mut d1: Dir) -> Self {
    if d0 > d1 {
      swap(&mut d0, &mut d1);
    }
    match (d0, d1) {
      (Dir::N, Dir::S) => Self::PipeNS,
      (Dir::E, Dir::W) => Self::PipeEW,
      (Dir::E, Dir::N) => Self::PipeEN,
      (Dir::N, Dir::W) => Self::PipeNW,
      (Dir::W, Dir::S) => Self::PipeWS,
      (Dir::E, Dir::S) => Self::PipeSE,
      _ => panic!(),
    }
  }

  fn connects(&self) -> &'static [Dir] {
    match self {
      Self::Empty => &[],
      Self::Start => panic!(),
      Self::PipeEW => &[Dir::E, Dir::W],
      Self::PipeNS => &[Dir::N, Dir::S],
      Self::PipeEN => &[Dir::E, Dir::N],
      Self::PipeNW => &[Dir::N, Dir::W],
      Self::PipeWS => &[Dir::W, Dir::S],
      Self::PipeSE => &[Dir::E, Dir::S],
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_util::sample_lines;

  #[test]
  fn test1_sample() {
    assert_eq!(part1(sample_lines("10a")), 4);
    assert_eq!(part1(sample_lines("10b")), 8);
  }

  #[test]
  fn test1() {
    assert_eq!(part1(sample_lines("10")), 6778);
  }

  #[test]
  fn test2_sample() {
    assert_eq!(part2(sample_lines("10c")), 4);
    assert_eq!(part2(sample_lines("10d")), 4);
    assert_eq!(part2(sample_lines("10e")), 8);
    assert_eq!(part2(sample_lines("10f")), 10);
  }

  #[test]
  fn test2() {
    assert_eq!(part2(sample_lines("10")), 433);
  }
}

use itertools::Itertools;
use strum::{EnumIter, IntoEnumIterator};

pub fn part1(lines: impl Iterator<Item=String>) -> usize {
  let map: Vec<Vec<_>> = lines
    .map(|s| s.chars().collect())
    .collect();

  let start = map.iter()
    .enumerate()
    .flat_map(|(y, row)| row.iter()
      .enumerate()
      .filter(|(_, &c)| c == 'S')
      .map(move |(x, _)| (y, x)))
    .exactly_one().unwrap();
  let mut dir = Dir::iter()
    .filter(|d|
      d.try_step(start)
        .and_then(|(y, x)| d.try_turn(map[y][x]))
        .is_some()
    )
    .next().unwrap();

  let mut path_len = 0;
  let mut pos = start;
  loop {
    pos = dir.try_step(pos).unwrap();
    path_len += 1;
    if pos == start { break; }
    dir = dir.try_turn(map[pos.0][pos.1]).unwrap();
  }

  path_len / 2
}

type Coord = (usize, usize);

#[derive(EnumIter)]
enum Dir { E, N, W, S }

impl Dir {
  fn try_step(&self, (y, x): Coord) -> Option<Coord> {
    match self {
      Self::E => x.checked_add(1).map(|x| (y, x)),
      Self::N => y.checked_sub(1).map(|y| (y, x)),
      Self::W => x.checked_sub(1).map(|x| (y, x)),
      Self::S => y.checked_add(1).map(|y| (y, x)),
    }
  }

  fn try_turn(&self, c: char) -> Option<Self> {
    match (self, c) {
      (Self::E, '-') => Some(Self::E),
      (Self::E, 'J') => Some(Self::N),
      (Self::E, '7') => Some(Self::S),
      (Self::N, '|') => Some(Self::N),
      (Self::N, 'F') => Some(Self::E),
      (Self::N, '7') => Some(Self::W),
      (Self::W, '-') => Some(Self::W),
      (Self::W, 'L') => Some(Self::N),
      (Self::W, 'F') => Some(Self::S),
      (Self::S, '|') => Some(Self::S),
      (Self::S, 'L') => Some(Self::E),
      (Self::S, 'J') => Some(Self::W),
      _ => None,
    }
  } }

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

  /*#[test]
  fn test2_sample() {
    assert_eq!(part2(sample_lines("10a")), 2);
  }

  #[test]
  fn test2() {
    assert_eq!(part2(sample_lines("10")), 1057);
  }*/
}

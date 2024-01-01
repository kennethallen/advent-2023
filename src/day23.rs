use std::{collections::HashSet, cmp::max};

use strum::{EnumIter, IntoEnumIterator};

pub fn part1(lines: impl Iterator<Item=String>) -> usize {
  let map: Vec<Vec<_>> = lines
    .map(|line| line.chars()
      .map(|c| match c {
        '.' => Tile::Path,
        '#' => Tile::Forest,
        '>' => Tile::Slope(Dir::E),
        '^' => Tile::Slope(Dir::N),
        '<' => Tile::Slope(Dir::W),
        'v' => Tile::Slope(Dir::S),
        _ => panic!(),
      })
      .collect())
    .collect();

  let start = (
    0,
    map[0].iter().position(|&t| t == Tile::Path).unwrap(),
  );
  let end = (
    map.len() - 1,
    map[map.len() - 1].iter().position(|&t| t == Tile::Path).unwrap(),
  );
  let bounds = (map.len(), map[0].len());
  dbg!(&start, &end, &bounds);

  let mut max_dist = 0;
  let mut paths = vec![(start, HashSet::from([start]), 0)];
  while let Some((pos, mut visited, dist)) = paths.pop() {
    //println!("Exploring {:?}", &pos);
    if pos == end {
      max_dist = max(max_dist, dist);
    } else {
      let dirs = match map[pos.0][pos.1] {
        Tile::Path => Dir::iter().collect(),
        Tile::Slope(dir) => vec![dir],
        _ => unreachable!(),
      };
      let mut nexts: Vec<_> = dirs.into_iter()
        .filter_map(|dir| dir.try_step(pos, bounds))
        .filter(|&next| !visited.contains(&next))
        .filter(|&(y, x)| map[y][x] != Tile::Forest)
        .collect();
      while nexts.len() > 1 {
        let next = nexts.pop().unwrap();
        let mut next_visited = visited.clone();
        next_visited.insert(next);
        paths.push((next, next_visited, dist+1));
      }
      if let Some(next) = nexts.pop() {
        visited.insert(next);
        paths.push((next, visited, dist+1));
      }
    }
  }
  max_dist
}

type Coord = (usize, usize);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
  Path,
  Forest,
  Slope(Dir),
}

#[derive(Clone, Copy, EnumIter, PartialEq, Eq)]
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
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_util::sample_lines;

  #[test]
  fn test1_sample() {
    assert_eq!(part1(sample_lines("23a")), 94);
  }

  #[test]
  fn test1() {
    assert_eq!(part1(sample_lines("23")), 2178);
  }
}

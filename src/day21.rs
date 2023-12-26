use std::collections::{HashMap, VecDeque};

use bitvec::prelude::*;

pub fn part1(lines: impl Iterator<Item=String>, steps: usize) -> usize {
  let mut start = None;
  let mut map: Vec<BitVec> = vec![];
  for (y, line) in lines.enumerate() {
    let mut row = bitvec!();
    for (x, tile) in line.chars().enumerate() {
      row.push(match tile {
        '#' => true,
        '.' => false,
        'S' => { start = Some((y, x)); false }
        _ => panic!(),
      });
    }
    map.push(row);
  }
  let start = start.unwrap();
  let map = map;

  let mut visited = HashMap::new();
  let mut to_visit = VecDeque::from([(start, 0)]);
  while let Some(((y ,x), dist)) = to_visit.pop_front() {
    if visited.try_insert((y, x), dist).is_ok() && dist < steps {
      let mut consider = |(y, x)| {
        let row: &BitVec = &map[y];
        let tile: bool = row[x];
        if !tile { to_visit.push_back(((y, x), dist+1)); }
      };
      if y > 0 { consider((y-1, x)); }
      if x > 0 { consider((y, x-1)); }
      if y < map.len()-1 { consider((y+1, x)); }
      if x < map[0].len()-1 { consider((y, x+1)); }
    }
  }
  visited.values().filter(|&dist| dist % 2 == 0).count()
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_util::sample_lines;

  #[test]
  fn test1_sample() {
    assert_eq!(part1(sample_lines("21a"), 6), 16);
  }

  #[test]
  fn test1() {
    assert_eq!(part1(sample_lines("21"), 64), 3847);
  }
}

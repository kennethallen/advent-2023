use std::{collections::{BTreeSet, HashMap}, cmp::{min, max}};

use itertools::Itertools;

pub fn part1(lines: impl Iterator<Item=String>) -> usize { process(lines, 2) }
pub fn part2(lines: impl Iterator<Item=String>) -> usize { process(lines, 1_000_000) }

fn process(lines: impl Iterator<Item=String>, gap_mult: usize) -> usize {
  let mut galaxies: Vec<_> = lines
    .enumerate()
    .flat_map(|(y, line)| line.chars()
      .enumerate()
      .filter(|&(_, c)| c == '#')
      .map(|(x, _)| (y, x))
      .collect::<Vec<_>>())
    .collect();

  // galaxies is sorted in ascending y order
  let ys = multiply_gaps(
    galaxies.iter().map(|&(y, _)| y).dedup(),
    gap_mult);
  let xs = multiply_gaps(
    galaxies.iter().map(|&(_, x)| x).collect::<BTreeSet<_>>().into_iter(),
    gap_mult);
  for galaxy in galaxies.iter_mut() {
    galaxy.0 = ys[&galaxy.0];
    galaxy.1 = xs[&galaxy.1];
  }

  galaxies.into_iter()
    .combinations(2)
    .map(|gs| dist(gs[0], gs[1]))
    .sum()
}

fn dist((y0, x0): Coord, (y1, x1): Coord) -> usize {
  max(y0, y1) - min(y0, y1) + max(x0, x1) - min(x0, x1)
}

fn multiply_gaps(sorted_ns: impl Iterator<Item=usize>, gap_mult: usize) -> HashMap<usize, usize> {
  sorted_ns
    .enumerate()
    .map(|(i, n)| (n, (n - i)*gap_mult + i))
    .collect()
}

type Coord = (usize, usize);

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_util::sample_lines;

  #[test]
  fn test1_sample() {
    assert_eq!(part1(sample_lines("11a")), 374);
  }

  #[test]
  fn test1() {
    assert_eq!(part1(sample_lines("11")), 9605127);
  }

  #[test]
  fn test2_sample() {
    assert_eq!(process(sample_lines("11a"), 10), 1030);
    assert_eq!(process(sample_lines("11a"), 100), 8410);
  }

  #[test]
  fn test2() {
    assert_eq!(part2(sample_lines("11")), 458191688761);
  }
}

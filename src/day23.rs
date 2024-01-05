use std::{collections::{HashSet, HashMap}, mem::take, iter::once};

use bit_set::BitSet;
use itertools::Itertools;
use strum::{EnumIter, IntoEnumIterator};

pub fn part1(lines: impl Iterator<Item=String>) -> usize { process(lines, false) }
pub fn part2(lines: impl Iterator<Item=String>) -> usize { process(lines, true) }

fn process(lines: impl Iterator<Item=String>, ignore_slopes: bool) -> usize {
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
  let bounds = (map.len(), map[0].len());

  let start = (
    0,
    map[0].iter().position(|&t| t == Tile::Path).unwrap(),
  );
  let end = (
    map.len() - 1,
    map[map.len() - 1].iter().position(|&t| t == Tile::Path).unwrap(),
  );

  let mut nodes = vec![];
  let mut lookup = HashMap::new();

  for (y, row) in map.iter().enumerate() {
    for (x, &tile) in row.iter().enumerate() {
      if tile != Tile::Forest {
        lookup.insert((y, x), nodes.len());
        nodes.push(Node::default());
      }
    }
  }

  let start_node = lookup[&start];
  let end_node = lookup[&end];

  for (&pos, &node) in &lookup {
    for dir in map[pos.0][pos.1].possible_exits(ignore_slopes) {
      if let Some(nbr) = dir.try_step(pos, bounds) && map[nbr.0][nbr.1] != Tile::Forest {
        let nbr_node = lookup[&nbr];
        nodes[node].outs.insert(nbr_node, 1);
        nodes[nbr_node].ins.insert(node);
      }
    }
  }

  let mut to_fix: Vec<_> = (0..nodes.len()).collect();
  while let Some(node) = to_fix.pop() {
    println!("Considering {} {:?}", node, &nodes[node]);
    debug_assert!(nodes[node].ins.iter().all(|&in_node| nodes[in_node].outs.contains_key(&node)));
    debug_assert!(nodes[node].outs.keys().all(|&out| nodes[out].ins.contains(&node)));
    if node == start_node || node == end_node { continue; }
    if match nodes[node].outs.len() {
      0 => {
        // Remove dead end
        println!("Removing dead end");
        for nbr in take(&mut nodes[node].ins) {
          nodes[nbr].outs.remove(&node).unwrap();
          to_fix.push(nbr);
        }
        true
      },
      1 => {
        println!("Redirecting ins to single out");
        // Redirect ins to single out
        let (next, d0) = take(&mut nodes[node].outs).into_iter().exactly_one().unwrap();
        assert!(nodes[next].ins.remove(&node));
        for nbr in take(&mut nodes[node].ins) {
          let d1 = nodes[nbr].outs.remove(&node).unwrap();
          if nbr == next {
            // This was an effective dead end for this in which could only go back to itself
            to_fix.push(nbr);
          } else {
            // We can discover multiple paths. For this, we only need the longest.
            if let Err(mut e) = nodes[nbr].outs.try_insert(next, d0 + d1) {
              if e.value > *e.entry.get() {
                e.entry.insert(e.value);
              }
              debug_assert!(nodes[next].ins.contains(&nbr));
            } else {
              assert!(nodes[next].ins.insert(nbr));
            }
          }
        }
        true
      },
      2 => {
        println!("Considering simplifying corridor");
        if nodes[node].ins.len() == 2 && nodes[node].outs.keys().all(|&out| nodes[node].ins.contains(&out)) {
          println!("Simplifying corridor");
          // Simplify corridor
          let mut iter = take(&mut nodes[node].outs).into_iter();
          let ((nbr0, dn0), (nbr1, dn1)) = iter.next_tuple().unwrap();
          assert!(iter.is_empty());

          let d0n = nodes[nbr0].outs.remove(&node).unwrap();
          assert_eq!(nodes[nbr0].outs.insert(nbr1, d0n + dn1), None);
          assert!(nodes[nbr0].ins.remove(&node));
          assert!(nodes[nbr0].ins.insert(nbr1));

          let d1n = nodes[nbr1].outs.remove(&node).unwrap();
          assert_eq!(nodes[nbr1].outs.insert(nbr0, d1n + dn0), None);
          assert!(nodes[nbr1].ins.remove(&node));
          assert!(nodes[nbr1].ins.insert(nbr0));

          true
        } else {
          false
        }
      },
      _ => false,
    } {
      // Reassign end node idx
      // TODO actually compact
      nodes[node].ins.clear();
      nodes[node].outs.clear();
    }
  }

  for (i, node) in nodes.iter().enumerate() {
    if node.ins.is_empty() {
      assert!(node.outs.is_empty());
    } else {
      println!("{} {:?}", i, node);
    }
  }
  panic!();

  let mut max_dist = 0;
  let mut paths: Vec<(_, BitSet, _)> = vec![(start_node, once(start_node).collect(), 0)];
  while let Some((pos, mut visited, dist)) = paths.pop() {
    if pos == end_node {
      if dist > max_dist { max_dist = dist; }
    } else {
      let mut using_outs: Vec<_> = nodes[pos].outs.iter()
        .map(|(&out, &next_dist)| (out, next_dist))
        .filter(|&(out, _)| !visited.contains(out))
        .collect();
      while using_outs.len() > 1 {
        let (out, next_dist) = using_outs.pop().unwrap();
        let mut new_visited = visited.clone();
        new_visited.insert(out);
        paths.push((out, new_visited, dist + next_dist));
      }
      if let Ok((out, next_dist)) = using_outs.into_iter().exactly_one() {
        visited.insert(out);
        paths.push((out, visited, dist + next_dist));
      }
    }
  }

  max_dist
}

#[derive(Default, Debug)]
struct Node {
  ins: HashSet<usize>,
  outs: HashMap<usize, usize>,
}

type Coord = (usize, usize);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
  Path,
  Forest,
  Slope(Dir),
}

impl Tile {
  fn possible_exits(&self, ignore_slopes: bool) -> Vec<Dir> {
    if !ignore_slopes && let &Tile::Slope(dir) = self {
      vec![dir]
    } else {
      Dir::iter().collect()
    }
  }
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

  #[test]
  fn test2_sample() {
    assert_eq!(part2(sample_lines("23a")), 154);
  }

  #[test]
  fn test2() {
    assert_eq!(part2(sample_lines("23")), 6486);
  }
}

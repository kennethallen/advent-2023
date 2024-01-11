
use std::collections::{BTreeSet, HashSet, HashMap, BTreeMap};

use arrayvec::ArrayVec;
use itertools::{Itertools, iproduct};
use nom::{IResult, character::complete::{char, alpha1}, multi::separated_list1, combinator::eof, sequence::{separated_pair, terminated}, bytes::complete::tag};

pub fn part1(lines: impl Iterator<Item=String>) -> usize {
  let mut wires = HashMap::<_, HashSet<_>>::new();
  let mut translation = HashMap::new();
  let mut translate = |string: &str| {
    let next_id = translation.len();
    *translation.entry(string.to_string()).or_insert(next_id)
  };
  for line in lines {
    let (a, bs) = parse(&line).unwrap().1;
    let a = translate(a);
    for b in bs {
      let b = translate(b);
      for (l, r) in [(a, b), (b, a)] {
        wires.entry(l).or_default().insert(r);
      }
    }
  }

  let mut nodes_iter = wires.keys();
  let start = *nodes_iter.next().unwrap();
  let paths: ArrayVec<Vec<_>, 3> = nodes_iter
    .find_map(|&end| {
      let mut pathfinder = IndependentPaths { wires: &wires, used: HashSet::new() };
      let mut paths = ArrayVec::<_, 3>::new();
      for _ in 0..3 {
        paths.push(pathfinder.try_block_path(start, end).expect("at least three independent paths should exist"));
      }
      if pathfinder.try_find_path(start, end).is_some() { return None; }

      Some(paths)
    })
    .unwrap()
    .into_iter()
    .map(|path| path.into_iter()
      .tuple_windows()
      .map(canonical_edge)
      .collect())
    .collect();
  
  iproduct!(&paths[0], &paths[1], &paths[2])
    .find_map(|(&e0, &e1, &e2)| {
      let mut trimmed_wires = wires.clone();
      for (a, b) in [e0, e1, e2] {
        for (l, r) in [(a, b), (b, a)] {
          let rem = trimmed_wires.get_mut(&l).unwrap().remove(&r);
          debug_assert!(rem);
        }
      }

      let &start = trimmed_wires.keys().next().unwrap();
      let mut to_explore = BTreeSet::from([start]);
      let mut visited = HashSet::new();
      while let Some(node) = to_explore.pop_first() {
        visited.insert(node);
        for &next in &trimmed_wires[&node] {
          if !visited.contains(&next) {
            to_explore.insert(next);
          }
        }
      }

      if visited.len() != wires.len() {
        Some(visited.len() * (wires.len() - visited.len()))
      } else {
        None
      }
    })
    .unwrap()
}

fn parse(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
  terminated(
    separated_pair(
      alpha1,
      tag(": "),
      separated_list1(char(' '), alpha1),
    ),
    eof,
  )(input)
}

struct IndependentPaths<'a> {
  wires: &'a HashMap<usize, HashSet<usize>>,
  used: HashSet<(usize, usize)>,
}

impl <'a> IndependentPaths<'a> {
  fn try_block_path(&mut self, start: usize, end: usize) -> Option<Vec<usize>> {
    let path = self.try_find_path(start, end)?;
    self.used.extend(path.iter()
      .copied()
      .tuple_windows()
      .map(canonical_edge));
    Some(path)
  }

  fn try_find_path(&self, start: usize, end: usize) -> Option<Vec<usize>> {
    let mut explored = HashMap::new();
    let mut to_explore = BTreeMap::from([(start, vec![start])]);
    while let Some((node, path)) = to_explore.pop_first() {
      if node == end { return Some(path); }
      explored.insert(node, path);
      for &next in &self.wires[&node] {
        if !self.used.contains(&canonical_edge((node, next))) && !explored.contains_key(&next) {
          to_explore.entry(next).or_insert_with_key(|&next| {
            let mut next_path = explored[&node].clone();
            next_path.push(next);
            next_path
          });
        }
      }
    }
    None
  }
}

fn canonical_edge((a, b): (usize, usize)) -> (usize, usize) {
  if a <= b { (a, b) } else { (b, a) }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_util::sample_lines;

  #[test]
  fn test1_sample() {
    assert_eq!(part1(sample_lines("25a")), 54);
  }

  #[test]
  fn test1() {
    assert_eq!(part1(sample_lines("25")), 598120);
  }
}

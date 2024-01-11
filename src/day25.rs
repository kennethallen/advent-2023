use std::{collections::{HashMap, HashSet, BTreeSet}, fs::File, io::Write};

use itertools::Itertools;
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

  let f = File::create("data/day25.dot").unwrap();
  write!(&f, "graph D {{\n").unwrap();
  for (&l, rs) in &wires {
    for &r in rs {
      if l <= r {
        write!(&f, "{} -- {}\n", l, r).unwrap();
      }
    }
  }
  write!(&f, "}}\n").unwrap();
  drop(f);
  
  wires.iter()
    .flat_map(|(&l, v)| v.iter()
      .map(move |&r| (l, r)))
      .filter(|&(l, r)| l <= r)
    .permutations(3)
    .filter_map(|to_cut| {
      let mut trimmed_wires = wires.clone();
      for (a, b) in to_cut {
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
    .next().unwrap()
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
    assert_eq!(part1(sample_lines("25")), 14672);
  }

  /*#[test]
  fn test2_sample() {
    assert_eq!(part2(sample_lines("25a")), 47);
  }

  #[test]
  fn test2() {
    assert_eq!(part2(sample_lines("25")), 6486);
  }*/
}

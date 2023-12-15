use std::cmp::min;

use crate::util::usize;

use nom::{IResult, character::complete::{char, one_of}, multi::{separated_list1, many1}, sequence::{terminated, separated_pair}, combinator::{eof, map}};

pub fn part1(lines: impl Iterator<Item=String>) -> usize {
  lines
    .map(|line| parse(line.as_str()).unwrap().1)
    .filter_map(|(pat, runs)| count_solutions_0(&pat, &runs))
    .sum()
}

fn count_solutions_0(pat: &[Option<bool>], runs: &[usize]) -> Option<usize> {
  let x = count_solutions(pat, runs);
  println!("{:?} {:?} {:?}", x.unwrap_or_default(), pat.iter().map(|&sym| match sym { None => '?', Some(true) => '#', Some(false) => '.' }).collect::<String>(), runs);
  //dbg!(x);
  x
}
fn count_solutions(mut pat: &[Option<bool>], mut runs: &[usize]) -> Option<usize> {
  loop {
    match pat.first().copied() {
      None => return if runs.is_empty() { Some(1) } else { None },
      Some(Some(false)) => pat = &pat[1..],
      Some(Some(true)) => {
        if !could_start_run(pat, runs) { return None; }
        pat = &pat[min(runs[0]+1, pat.len())..];
        runs = &runs[1..];
      }
      Some(None) => if could_start_run(pat, runs) {
        return Some(
          count_solutions_0(&pat[1..], runs).unwrap_or_default()
          + count_solutions_0(&pat[min(runs[0]+1, pat.len())..], &runs[1..]).unwrap_or_default()
        );
      } else {
        // Assume empty
        pat = &pat[1..];
      }
    }
  }
}

// Test if the first run[0]+1 symbols can satisfy the first run.
// Precondition: first pattern symbol known not to be empty.
fn could_start_run(pat: &[Option<bool>], runs: &[usize]) -> bool {
  if let Some(&rem_range) = runs.first() {
    rem_range <= pat.len()
    && !pat[1..rem_range].iter().any(|&sym| sym == Some(false))

    // Must be followed by an empty or a ? (presumed empty) or the end of the pattern
    && pat.get(rem_range).copied() != Some(Some(true))
  } else {
    false
  }
}

fn parse(input: &str) -> IResult<&str, (Vec<Option<bool>>, Vec<usize>)> {
  terminated(
    separated_pair(
      many1(map(
        one_of("?#."),
        |c| match c { '?' => None, '#' => Some(true), '.' => Some(false), _ => unreachable!() },
      )),
      char(' '),
      separated_list1(char(','), usize),
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
    assert_eq!(part1(sample_lines("12a")), 21);
  }

  #[test]
  fn test1() {
    assert_eq!(part1(sample_lines("12")), 7032);
  }

  /*#[test]
  fn test2_sample() {
    assert_eq!(part2(sample_lines("12a")), 2);
  }

  #[test]
  fn test2() {
    assert_eq!(part2(sample_lines("12")), 1057);
  }*/
}

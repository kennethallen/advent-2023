use std::cmp::Ordering;

use crate::util::usize;

use nom::{IResult, character::complete::{char, one_of}, multi::{separated_list1, many1}, sequence::{terminated, separated_pair}, combinator::{eof, map}};

pub fn part1(lines: impl Iterator<Item=String>) -> usize {
  lines
    .map(|line| parse(line.as_str()).unwrap().1)
    .filter_map(|(pat, runs)| count_solutions_0(&pat, &runs))
    .sum()
}

fn count_solutions_0(mut pat: &[Option<bool>], mut runs: &[usize]) -> Option<usize> {
  // Strip front
  loop {
    match pat.first().copied() {
      None => return Some(1),
      Some(Some(false)) => pat = &pat[1..],
      Some(None) => break,
      Some(Some(true)) => {
        let run_len = pat.iter().take_while(|&&sym| sym == Some(true)).count();
        if run_len == 0 { break; }
        match run_len.cmp(runs.first()?) {
          Ordering::Less => {
            let mut new_runs = runs.to_vec();
            new_runs[0] -= run_len;
            return count_solutions_0(&pat[run_len..], &new_runs);
          }
          Ordering::Equal => {
            pat = &pat[run_len..];
            runs = &runs[1..];
          },
          Ordering::Greater => return None, // Observed run longer than specified run
        }
      }
    }
  }
  count_solutions_1(pat, runs)
}

fn count_solutions_1(mut pat: &[Option<bool>], mut runs: &[usize]) -> Option<usize> {
  // Starts with ?
  // Strip back
  loop {
    match pat.last().copied() {
      None => unreachable!("Should always have at least a ? at the beginning"),
      Some(Some(false)) => pat = &pat[..pat.len()-1],
      Some(None) => break,
      Some(Some(true)) => {
        let run_len = pat.iter().rev().take_while(|&&sym| sym == Some(true)).count();
        if run_len == 0 { break; }
        match run_len.cmp(runs.last()?) {
          Ordering::Less => {
            let mut new_runs = runs.to_vec();
            new_runs[runs.len()-1] -= run_len;
            return count_solutions_1(&pat[..pat.len()-run_len], &new_runs);
          }
          Ordering::Equal => {
            pat = &pat[..pat.len()-run_len];
            runs = &runs[..runs.len()-1];
          },
          Ordering::Greater => return None, // Observed run longer than specified run
        }
      }
    }
  }
  count_solutions_2(pat, runs)
}

fn count_solutions_2(pat: &[Option<bool>], runs: &[usize]) -> Option<usize> {
  // Starts and ends with ?
  //if pat.len() == runs.iter().sum::<usize>() + runs.len() - 1 { return 1; } // Minimum
  if runs.is_empty() {
    assert!(pat.iter().all(|&sym| sym != Some(true)));
    return 1;
  }
  if runs.len() == 1 {
    assert!(runs[0] <= pat.len());
    if runs[0] == pat.len() { return 1; }
  }

  //panic!("Can't deduce {:?} {:?}", pat, runs);
  count_solutions_0(&pat[1..], runs) + { // Suppose empty
    // Suppose full
    let run_len = runs[0];
    assert!(run_len < pat.len(), "Huh {:?} {:?}", pat, runs);
    if pat[1..run_len].iter().all(|&sym| sym != Some(false))
        && pat[run_len] != Some(true) {
      count_solutions_1(&pat[1..], &runs[1..])
    } else {
      0
    }
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
    assert_eq!(part1(sample_lines("12")), 1);
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

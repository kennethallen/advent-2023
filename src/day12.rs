use std::{cmp::min, collections::HashMap};

use crate::util::usize;

use nom::{IResult, character::complete::{char, one_of}, multi::{separated_list1, many1}, sequence::{terminated, separated_pair}, combinator::{eof, map}};

pub fn part1(lines: impl Iterator<Item=String>) -> usize {
  let puzzles: Vec<_> = lines
    .map(|line| parse(line.as_str()).unwrap().1)
    .collect();
  let mut memo: Memo = Default::default();
  puzzles.iter()
    .map(|(pat, runs)| count_solutions_memo(&pat, &runs, &mut memo))
    .sum()
}
pub fn part2(lines: impl Iterator<Item=String>) -> usize {
  let puzzles: Vec<_> = lines
    .map(|line| parse(line.as_str()).unwrap().1)
    .map(|(pat, runs)| {
      let mut big_pat = pat.clone();
      let mut big_runs = runs.clone();
      for _ in 0..4 {
        big_pat.push(None);
        big_pat.extend(&pat);
        big_runs.extend(&runs);
      }
      (big_pat, big_runs)
    })
    .collect();
  let mut memo: Memo = Default::default();
  puzzles.iter()
    .map(|(pat, runs)| count_solutions_memo(&pat, &runs, &mut memo))
    .sum()
}

type Memo<'a> = HashMap<(&'a [Option<bool>], &'a [usize]), usize>;

/*fn count_solutions_0<'a>(mut pat: &'a [Option<bool>], mut runs: &'a [usize], memo: &mut Memo<'a>) -> usize {
  /*// Strip adjacent empties
  let mut new_pat = Vec::with_capacity(pat.len());
  let mut last_empty = false;
  for &sym in pat {
    if sym == Some(false) {
      if !last_empty {
        new_pat.push(sym);
        last_empty = true;
      }
    } else {
      new_pat.push(sym);
      last_empty = false;
    }
  }
  let mut pat = new_pat.as_slice();*/

  // Strip certainties from end
  /*loop {
    match pat.last().copied() {
      Some(Some(false)) => pat = &pat[..pat.len()-1],
      Some(Some(true)) => {
        if !could_end_run(&pat, &runs) { return 0; }
        pat = &pat[..pat.len().saturating_sub(runs[runs.len()-1]+1)];
        runs = &runs[..runs.len()-1];
      }
      _ => break,
    }
  }*/

  let x = count_solutions_memo(pat, runs, memo);
  //println!("{:?} {:?} {:?}", x.unwrap_or_default(), pat.iter().map(|&sym| match sym { None => '?', Some(true) => '#', Some(false) => '.' }).collect::<String>(), runs);
  //dbg!(x);
  x
}*/

fn count_solutions_memo<'a>(pat: &'a [Option<bool>], runs: &'a [usize], memo: &mut Memo<'a>) -> usize {
  if let Some(&sols) = memo.get(&(pat, runs)) {
    sols
  } else {
    let sols = count_solutions(pat, runs, memo);
    memo.insert((pat, runs), sols);
    sols
  }
}
fn count_solutions<'a>(mut pat: &'a [Option<bool>], mut runs: &'a [usize], memo: &mut Memo<'a>) -> usize {
  loop {
    match pat.first().copied() {
      None => return if runs.is_empty() { 1 } else { 0 },
      Some(Some(false)) => pat = &pat[1..],
      Some(Some(true)) => {
        if !could_start_run(pat, runs) { return 0; }
        pat = &pat[min(runs[0]+1, pat.len())..];
        runs = &runs[1..];
      }
      Some(None) => if could_start_run(pat, runs) {
        return count_solutions_memo(&pat[1..], runs, memo)
          + count_solutions_memo(&pat[min(runs[0]+1, pat.len())..], &runs[1..], memo);
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
// Test if the last run[runs.len()-1]+1 symbols can satisfy the last run.
// Precondition: last pattern symbol known not to be empty.
/*fn could_end_run(pat: &[Option<bool>], runs: &[usize]) -> bool {
  if let Some(&rem_range) = runs.last() {
    rem_range <= pat.len()
    && !pat[pat.len()-rem_range..pat.len()-1].iter().any(|&sym| sym == Some(false))

    // Must be followed by an empty or a ? (presumed empty) or the end of the pattern
    && !pat.len().checked_sub(rem_range+1).is_some_and(|i| pat[i] != Some(true))
  } else {
    false
  }
}*/

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

  #[test]
  fn test2_sample() {
    assert_eq!(part2(sample_lines("12a")), 525152);
  }

  #[test]
  fn test2() {
    assert_eq!(part2(sample_lines("12")), 1493340882140);
  }
}

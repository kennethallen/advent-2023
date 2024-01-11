use bitvec::{prelude::*, vec::BitVec};
use itertools::Itertools;
use nom::{IResult, sequence::{terminated, pair}, character::complete::one_of, combinator::{eof, map_res, map}, bytes::complete::{tag, take}, character::complete::char};

use crate::util::usize;

pub fn part1(lines: impl Iterator<Item=String>) -> usize { process(lines, 0) }
pub fn part2(lines: impl Iterator<Item=String>) -> usize { process(lines, 1) }

fn process(lines: impl Iterator<Item=String>, instr_idx: usize) -> usize {
  let mut curs: (isize, isize) = (0, 0);
  let mut trench = Trench::default();
  trench.dig_horiz(curs, 1);
  for line in lines {
    let (dir, run) = parse(&line).unwrap().1[instr_idx];
    match dir {
      Dir::R => {
        trench.dig_horiz((curs.0, curs.1 + 1), run);
        curs.1 += isize::try_from(run).unwrap();
      },
      Dir::L => {
        curs.1 -= isize::try_from(run).unwrap();
        trench.dig_horiz(curs, run);
      },
      Dir::D => {
        trench.dig_vert((curs.0 + 1, curs.1), run);
        curs.0 += isize::try_from(run).unwrap();
      },
      Dir::U => {
        curs.0 -= isize::try_from(run).unwrap();
        trench.dig_vert(curs, run);
      },
    };
  }

  trench.lagoon_area()
}

struct Trench {
  col_idxs: Vec<isize>,
  rows: Vec<(isize, BitVec)>,
}

impl Default for Trench {
  fn default() -> Self {
    Self {
      col_idxs: vec![isize::MIN],
      rows: vec![(isize::MIN, bitvec![0])],
    }
  }
}

impl Trench {
  fn dig_horiz(&mut self, (y, x): Coord, len: usize) {
    let px = self.materialize_col(x);
    let px_max = self.materialize_col(x + isize::try_from(len).unwrap());
    let py = self.materialize_row(y);
    self.materialize_row_at(y+1, py+1);
    self.rows[py].1[px..px_max].fill(true);
  }
  fn dig_vert(&mut self, (y, x): Coord, len: usize) {
    let px = self.materialize_col(x);
    self.materialize_col_at(x+1, px+1);
    let py = self.materialize_row(y);
    let py_max = self.materialize_row(y + isize::try_from(len).unwrap());
    for (_, bits) in &mut self.rows[py..py_max] {
      bits.set(px, true);
    }
  }

  fn lagoon_area(&self) -> usize {
    let mut exterior: Vec<_> = self.rows.iter()
      .map(|(_, bits)| bitvec![0; bits.len()])
      .collect();
    let mut to_check = vec![];
    for x in 0..self.col_idxs.len() {
      to_check.push((0, x));
      to_check.push((self.rows.len()-1, x));
    }
    for y in 1..self.rows.len()-1 {
      to_check.push((y, 0));
      to_check.push((y, self.col_idxs.len()-1));
    }

    while let Some((y, x)) = to_check.pop() {
      if !exterior[y][x] && !self.rows[y].1[x] {
        exterior[y].set(x, true);
        if y > 0 { to_check.push((y-1, x)); }
        if y+1 < self.rows.len() { to_check.push((y+1, x)); }
        if x > 0 { to_check.push((y, x-1)); }
        if x+1 < self.col_idxs.len() { to_check.push((y, x+1)); }
      }
    }

    let widths: Vec<_> = self.col_idxs.iter()
      .enumerate()
      .map(|(px, &x)| self.col_idxs[(px+1)%self.col_idxs.len()].wrapping_sub(x))
      .collect();

    let area: isize = exterior.into_iter()
      .enumerate()
      .map(|(py, row)| {
        let height = self.rows[(py+1)%self.rows.len()].0.wrapping_sub(self.rows[py].0);
        let row_sum: isize = row.into_iter()
          .zip_eq(widths.iter())
          .filter(|&(ext, _)| !ext)
          .map(|(_, &width)| width)
          .sum();
        height * row_sum
      })
      .sum();
    area.try_into().unwrap()
  }

  fn materialize_row(&mut self, y: isize) -> usize {
    match self.rows.binary_search_by_key(&y, |&(y, _)| y) {
      Ok(py) => py,
      Err(py) => { self.materialize_row_at_impl(y, py); py },
    }
  }
  fn materialize_row_at(&mut self, y: isize, py: usize) {
    if let Some(&(y0, _)) = self.rows.get(py) && y == y0 { return; }
    self.materialize_row_at_impl(y, py);
  }
  fn materialize_row_at_impl(&mut self, y: isize, py: usize) {
    self.rows.insert(py, (y, self.rows[py-1].1.clone()));
  }

  fn materialize_col(&mut self, x: isize) -> usize {
    match self.col_idxs.binary_search(&x) {
      Ok(px) => px,
      Err(px) => { self.materialize_col_at_impl(x, px); px },
    }
  }
  fn materialize_col_at(&mut self, x: isize, px: usize) {
    if let Some(&x0) = self.col_idxs.get(px) && x == x0 { return; }
    self.materialize_col_at_impl(x, px);
  }
  fn materialize_col_at_impl(&mut self, x: isize, px: usize) {
    self.col_idxs.insert(px, x);
    for (_, bits) in self.rows.iter_mut() {
      bits.insert(px, bits[px-1]);
    }
  }
}

type Coord = (isize, isize);

fn parse(input: &str) -> IResult<&str, [(Dir, usize); 2]> {
  let (input, dir0) = terminated(
    map(
      one_of("UDLR"),
      |c| match c { 'U' => Dir::U, 'D' => Dir::D, 'L' => Dir::L, 'R' => Dir::R, _ => unreachable!() },
    ),
    char(' '),
  )(input)?;
  let (input, run0) = terminated(usize, tag(" (#"))(input)?;
  let (input, run1) = map_res(take(5usize), |s| usize::from_str_radix(s, 16))(input)?;
  let (input, dir1) = terminated(
    map(
      one_of("0123"),
      |c| match c { '0' => Dir::R, '1' => Dir::D, '2' => Dir::L, '3' => Dir::U, _ => unreachable!() },
    ),
    pair(
      char(')'),
      eof,
    ),
  )(input)?;
  Ok((input, [(dir0, run0), (dir1, run1)]))
}

#[derive(Clone, Copy)]
enum Dir { R, U, L, D }

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_util::sample_lines;

  #[test]
  fn test1_sample() {
    assert_eq!(part1(sample_lines("18a")), 62);
  }

  #[test]
  fn test1() {
    assert_eq!(part1(sample_lines("18")), 58550);
  }

  #[test]
  fn test2_sample() {
    assert_eq!(part2(sample_lines("18a")), 952408144115);
  }

  #[test]
  fn test2() {
    assert_eq!(part2(sample_lines("18")), 47452118468566);
  }
}

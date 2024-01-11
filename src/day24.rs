use std::iter::zip;

use crate::util::isize;

use itertools::Itertools;
use nom::{IResult, character::complete::char, multi::{separated_list1, many0}, combinator::{map_res, eof}, sequence::{separated_pair, terminated, preceded}, bytes::complete::tag};
use num::{BigInt, BigRational, Signed};

pub fn part1(lines: impl Iterator<Item=String>, test_range: (isize, isize)) -> usize {
  let hailstones: Vec<_> = lines
    .map(|line| parse(&line).unwrap().1)
    .map(|(pos, vel)| (flatten(pos), flatten(vel)))
    .collect();

  let test_range = (
    BigRational::from(BigInt::from(test_range.0)),
    BigRational::from(BigInt::from(test_range.1)),
  );
  hailstones.iter()
    .tuple_combinations()
    .filter(|&(&(a_pos, a_vel), &(b_pos, b_vel))| {
      let denom = a_vel[1]*b_vel[0] - a_vel[0]*b_vel[1];
      denom != 0 && {
        let a_t = BigRational::new(
          BigInt::from(b_vel[0]*(b_pos[1] - a_pos[1]) + b_vel[1]*(a_pos[0] - b_pos[0])),
          BigInt::from(denom),
        );
        let b_t = BigRational::new(
          BigInt::from(a_vel[0]*(b_pos[1] - a_pos[1]) + a_vel[1]*(a_pos[0] - b_pos[0])),
          BigInt::from(denom),
        );
        !a_t.is_negative() && !b_t.is_negative() && {
          let a = [
            &a_t*BigRational::from(BigInt::from(a_vel[0])) + BigRational::from(BigInt::from(a_pos[0])),
            &a_t*BigRational::from(BigInt::from(a_vel[1])) + BigRational::from(BigInt::from(a_pos[1])),
          ];
          debug_assert_eq!(&a, &[
            &b_t*BigRational::from(BigInt::from(b_vel[0])) + BigRational::from(BigInt::from(b_pos[0])),
            &b_t*BigRational::from(BigInt::from(b_vel[1])) + BigRational::from(BigInt::from(b_pos[1])),
          ]);
          a[0] >= test_range.0 && a[0] <= test_range.1 && a[1] >= test_range.0 && a[1] <= test_range.1
        }
      }
    })
    .count()
}

fn flatten<T: Copy>(xyz: [T; 3]) -> [T; 2] {
  [xyz[0], xyz[1]]
}

pub fn part2(lines: impl Iterator<Item=String>) -> usize {
  let hailstones: Vec<_> = lines
    .map(|line| parse(&line).unwrap().1)
    .collect();

  let (h0_pos, h0_vel) = hailstones[0];
  let (h1_pos, h1_vel) = hailstones[1];
  let exec = |t0, t1| {
    let h0 = op2(&h0_pos, &h0_vel, |p, v| p + t0*v);
    let h1 = op2(&h1_pos, &h1_vel, |p, v| p + t1*v);
    let diff = op2(&h0, &h1, |h0, h1| h0 - h1);
    if !diff.iter().all(|&d0| d0 % (t0 - t1) == 0) { return None; }
    let rv = op1(&diff, |d| d/(t0-t1));
    let rp = op2(&h0, &rv, |h0, rv| h0 - rv*t0);
    //println!("{:?} {:?} {:?} {:?} {:?} {:?} {:?} ", t0, t1, &h0, &h1, &diff, &rv, &rp);

    for &(hp, hv) in &hailstones[2..] {
      let t_numer = op2(&rp, &hp, |rp, hp| rp - hp);
      let t_denom = op2(&hv, &rv, |hv, rv| hv - rv);
      //println!("testing hs {:?} {:?}, {:?}/{:?}", &hp, &hv, &t_numer, &t_denom);
      // Test for any n/0 (n != 0) or n not divisible by d (too strict?)
      if zip(t_numer, t_denom).any(|(n, d)| if d == 0 { n != 0 } else { n % d != 0 }) { return None; }
      // All must be 0/0 or mutually equal
      if !zip(t_numer, t_denom).filter(|&(n, d)| n != 0 || d != 0).map(|(n, d)| n/d).all_equal() { return None; }
    }

    Some(rp)
  };
  for t0 in 2.. {
    if t0 % 1000 == 0 { println!("{}", t0); }
    for t1 in 1..t0 {
      if let Some(pos) = exec(t0, t1) { return pos.into_iter().sum::<isize>().try_into().unwrap(); }
      if let Some(pos) = exec(t1, t0) { return pos.into_iter().sum::<isize>().try_into().unwrap(); }
    }
  }
  unreachable!()
}

type Coord = [isize; 3];
fn op1(a: &Coord, mut f: impl FnMut(isize) -> isize) -> Coord {
  let mut x = [0; 3];
  for i in 0..3 {
    x[i] = f(a[i]);
  }
  x
}
fn op2(a: &Coord, b: &Coord, mut f: impl FnMut(isize, isize) -> isize) -> Coord {
  let mut x = [0; 3];
  for i in 0..3 {
    x[i] = f(a[i], b[i]);
  }
  x
}

fn parse(input: &str) -> IResult<&str, (Coord, Coord)> {
  terminated(
    separated_pair(
      parse_coord,
      tag(" @ "),
      parse_coord,
    ),
    eof,
  )(input)
}

fn parse_coord(input: &str) -> IResult<&str, Coord> {
  map_res(
    separated_list1(
      char(','),
      preceded(many0(char(' ')), isize),
    ),
    |ns| ns.try_into(),
  )(input)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_util::sample_lines;

  #[test]
  fn test1_sample() {
    assert_eq!(part1(sample_lines("24a"), (7, 27)), 2);
  }

  #[test]
  fn test1() {
    assert_eq!(part1(sample_lines("24"), (200000000000000, 400000000000000)), 14672);
  }

  #[test]
  fn test2_sample() {
    assert_eq!(part2(sample_lines("24a")), 47);
  }

  #[test]
  fn test2() {
    assert_eq!(part2(sample_lines("24")), 6486);
  }
}

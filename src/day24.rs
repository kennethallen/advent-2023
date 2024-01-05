use crate::util::isize;

use itertools::Itertools;
use nom::{IResult, character::complete::char, multi::{separated_list1, many0}, combinator::{map_res, eof}, sequence::{separated_pair, terminated, preceded}, bytes::complete::tag};
use num::{BigInt, BigRational, Signed};

pub fn part1(lines: impl Iterator<Item=String>, test_range: (isize, isize)) -> usize {
  let hailstones: Vec<_> = lines
    .map(|line| parse(line.as_str()).unwrap().1)
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

type Coord = [isize; 3];

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
}

use std::{fmt::Debug, iter::zip, mem::{swap, replace, take}};

use crate::util::isize;

use arrayvec::ArrayVec;
use itertools::Itertools;
use nom::{IResult, character::complete::char, multi::{separated_list1, many0}, combinator::{map_res, eof}, sequence::{separated_pair, terminated, preceded}, bytes::complete::tag};
use num::{BigInt, Signed, Zero, BigRational, One};

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

  /*let exec = |t0, t1| {
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
  };*/

  let (h0_pos, h0_vel) = hailstones[0];
  let (h1_pos, h1_vel) = hailstones[1];
  let (h2_pos, h2_vel) = hailstones[2];
  // We are given that hailstone paths do not intersect. Assume they are not coplanar, either
  let r_pos = (0..).find_map(|t1| {
    if t1 % 1000 == 0 { println!("{}", t1); }

    // Find intersection of H2 with the plane formed by H0 and H1(t1)
    // P0 + a*V0 + b*(P1 + t1*V1 - P0) = P2 + t2*V2
    // a*V0 + b*(P1 + t1*V1 - P0) - t2*V2 = P2 - P0
    // This gives three equations in three variables (a, b, t2). Convert to matrix multiplication
    // [V0, P1 + t1*V1 - P0, -V2][a, b, t2] = P2 - P0
    let h1_hit = op2(h1_pos, h1_vel, |p, v| p + t1*v);
    let t2 = match solve_mx_eq_y(
      (0..3).map(|i| [
        h0_vel[i],
        h1_hit[i] - h0_pos[i],
        -h2_vel[i],
      ].map(BigInt::from).map(BigRational::from)).collect::<ArrayVec<_, 3>>().into_inner().unwrap(),
      op2(h2_pos, h0_pos, |p2, p0| BigInt::from(p2 - p0).into()),
    ) {
      Some(mut sol) => take(&mut sol[2]),
      None => return None,
    };
    if t2.is_negative() { return None; }

    let h2_hit = op2(h2_pos, h2_vel, |p, v| &t2*BigInt::from(v) + BigInt::from(p));

    let r_vel = op2(h2_hit.clone(), h1_hit, |h2, h1| (h2 - BigInt::from(h1)) / (&t2 - BigInt::from(t1)));
    let r_pos = op2(h2_hit, r_vel.clone(), |h2, v| h2 - v*&t2);

    // For every hailstone, assert that there is some nonnegative t s.t. r_pos + r_vel*t = h_pos + h_vel*t
    for (i, &(h_pos, h_vel)) in hailstones.iter().enumerate() {
      let h_pos = h_pos.map(BigInt::from).map(BigRational::from);
      let h_vel = h_vel.map(BigInt::from).map(BigRational::from);
      if i == 1 || i == 2 { continue; }

      let mut t_spec = None;
      for comp in 0..3 {
        if r_vel[comp] == h_vel[comp] {
          if r_pos[comp] != h_pos[comp] { return None; }
        } else {
          let t = (&r_pos[comp] - &h_pos[comp]) / (&h_vel[comp] - &r_vel[comp]);
          if t.is_negative() { return None; }
          if let Some(t0) = &t_spec {
            if &t != t0 { return None; }
          } else {
            t_spec = Some(t);
          }
        }
      }
    }
    Some(r_pos)
  }).unwrap();
  let res: BigRational = r_pos.into_iter().sum();
  assert!(res.is_integer());
  res.to_integer().try_into().unwrap()
}

fn cross(a: [BigRational; 3], b: [BigRational; 3]) -> [BigRational; 3] {
  [
    &a[1]*&b[2] - &a[2]*&b[1],
    &a[2]*&b[0] - &a[0]*&b[2],
    &a[0]*&b[1] - &a[1]*&b[0],
  ]
}
fn dot(a: [BigRational; 3], b: [BigRational; 3]) -> BigRational {
  zip(a, b).map(|(a, b)| a * b).sum()
}

fn debug_print(mat_rows: &[[BigRational; 3]; 3], y: &[BigRational; 3]) {
  for i in 0..3 {
    print!("[");
    for j in 0..3 {
      print!("{}, ", mat_rows[i][j]);
    }
    println!("] = [{}]", y[i]);
  }
  println!();
}
fn solve_mx_eq_y(mut mat_rows: [[BigRational; 3]; 3], mut y: [BigRational; 3]) -> Option<[BigRational; 3]> {
  for i in 0..3 {
    //debug_print(&mat_rows, &y);
    {
      let row = i + mat_rows[i..].into_iter().position(|row| !row[i].is_zero())?;
      mat_rows.swap(i, row);
      y.swap(i, row);
    }

    {
      let factor = replace(&mut mat_rows[i][i], BigInt::from(1).into()).recip();
      for col in (i+1)..3 {
        mat_rows[i][col] *= &factor;
      }
      y[i] *= &factor;
    }

    for row in 0..3 {
      if row != i {
        let factor = mat_rows[row][i].clone();
        for col in i..3 {
          mat_rows[row][col] -= &factor * &mat_rows[i][col];
        }
        y[row] -= &factor * &y[i];
      }
    }
  }
  //debug_print(&mat_rows, &y);
  debug_assert!(mat_rows.into_iter()
    .enumerate()
    .all(|(i, row)| row.into_iter()
      .enumerate()
      .all(|(j, val)| if i == j { val.is_one() } else { val.is_zero() })));
  Some(y)
}

type Coord = [isize; 3];

fn op1<I, O: Debug>(a: [I; 3], mut f: impl FnMut(I) -> O) -> [O; 3] {
  let mut x = ArrayVec::new();
  for a in a {
    x.push(f(a));
  }
  x.into_inner().unwrap()
}
fn op2<I0, I1, O: Debug>(a: [I0; 3], b: [I1; 3], mut f: impl FnMut(I0, I1) -> O) -> [O; 3] {
  let mut x = ArrayVec::new();
  for (a, b) in zip(a, b) {
    x.push(f(a, b));
  }
  x.into_inner().unwrap()
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

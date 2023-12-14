use nom::{IResult, character::complete::{i64, u64}, combinator::map};

pub fn usize(input: &str) -> IResult<&str, usize> {
  map(
    u64,
    |n| n.try_into().unwrap(),
  )(input)
}

pub fn isize(input: &str) -> IResult<&str, isize> {
  map(
    i64,
    |n| n.try_into().unwrap(),
  )(input)
}

use nom::{IResult, character::complete::u64};

pub fn usize(input: &str) -> IResult<&str, usize> {
  let (input, n) = u64(input)?;
  Ok((input, n.try_into().unwrap()))
}

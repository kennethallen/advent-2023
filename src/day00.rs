pub fn part1(lines: impl Iterator<Item=String>) -> u32 {
  lines
    .map(|line| {
      let mut chars = line.chars();
      let first_digit;
      let mut last_digit;
      'first: {
        while let Some(c) = chars.next() {
          if let Some(digit) = c.to_digit(10) {
            first_digit = digit;
            last_digit = digit;
            break 'first;
          }
        }
        panic!();
      }
      while let Some(c) = chars.next() {
        if let Some(digit) = c.to_digit(10) {
          last_digit = digit;
        }
      }
      first_digit*10 + last_digit
    })
    .sum()
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_util::sample_lines;

  #[test]
  fn test1() {
    assert_eq!(part1(sample_lines("00a")), 142);
    assert_eq!(part1(sample_lines("00")), 52974);
  }
}
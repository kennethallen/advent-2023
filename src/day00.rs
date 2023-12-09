pub fn part1(lines: impl Iterator<Item=String>) -> u32 {
  lines
    .map(|line| read_line(&line, false))
    .sum()
}
pub fn part2(lines: impl Iterator<Item=String>) -> u32 {
  lines
    .map(|line| read_line(&line, true))
    .sum()
}

fn read_line(line: &str, allow_text: bool) -> u32 {
  let mut char_idxs = line.char_indices();
  let first_digit;
  let mut last_digit;
  'first: {
    while let Some((i, _)) = char_idxs.next() {
      if let Some(digit) = read_digit(&line[i..], allow_text) {
        first_digit = digit;
        last_digit = digit;
        break 'first;
      }
    }
    panic!();
  }
  while let Some((i, _)) = char_idxs.next() {
    if let Some(digit) = read_digit(&line[i..], allow_text) {
      last_digit = digit;
    }
  }
  first_digit*10 + last_digit
}

fn read_digit(from: &str, allow_text: bool) -> Option<u32> {
  let mut chars = from.chars();
  let first_char = chars.next()?;
  if let Some(first_digit) = first_char.to_digit(10) {
    Some(first_digit)
  } else if allow_text {
    match first_char {
      'o' => if chars.take(2).eq("ne".chars()) { Some(1) } else { None },
      't' => match chars.next()? {
        'w' => if chars.take(1).eq("o".chars()) { Some(2) } else { None },
        'h' => if chars.take(3).eq("ree".chars()) { Some(3) } else { None },
        _ => None,
      },
      'f' => match chars.next()? {
        'o' => if chars.take(2).eq("ur".chars()) { Some(4) } else { None },
        'i' => if chars.take(2).eq("ve".chars()) { Some(5) } else { None },
        _ => None,
      },
      's' => match chars.next()? {
        'i' => if chars.take(1).eq("x".chars()) { Some(6) } else { None },
        'e' => if chars.take(3).eq("ven".chars()) { Some(7) } else { None },
        _ => None,
      },
      'e' => if chars.take(4).eq("ight".chars()) { Some(8) } else { None },
      'n' => if chars.take(3).eq("ine".chars()) { Some(9) } else { None },
      _ => None,
    }
  } else {
    None
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_util::sample_lines;

  #[test]
  fn test1_sample() {
    assert_eq!(part1(sample_lines("00a")), 142);
  }

  #[test]
  fn test1() {
    assert_eq!(part1(sample_lines("00")), 52974);
  }

  #[test]
  fn test2_sample() {
    assert_eq!(part2(sample_lines("00b")), 281);
  }

  #[test]
  fn test2() {
    assert_eq!(part2(sample_lines("00")), 53340);
  }
}
use std::collections::HashMap;

use bit_set::BitSet;

pub fn part1(lines: impl Iterator<Item=String>) -> usize {
  let schem = parse(lines);

  let mut inc_nums = BitSet::with_capacity(schem.nums.len());
  for (&(y, x), _) in &schem.syms {
    for dy in -1..=1 {
      for dx in -1..=1 {
        if let Some(&i) = schem.num_find.get(&(y+dy, x+dx)) {
          inc_nums.insert(i);
        }
      }
    }
  }

  inc_nums.into_iter()
    .map(|i| schem.nums[i].1)
    .sum()
}

pub fn part2(lines: impl Iterator<Item=String>) -> usize {
  let schem = parse(lines);

  schem.syms.iter()
    .filter(|(_, &is_gear)| is_gear)
    .flat_map(|(&(y, x), _)| -> Option<usize> {
      let mut inc_nums = BitSet::with_capacity(schem.nums.len());
      for dy in -1..=1 {
        for dx in -1..=1 {
          if let Some(&i) = schem.num_find.get(&(y+dy, x+dx)) {
            inc_nums.insert(i);
            if inc_nums.len() > 2 { return None; }
          }
        }
      }
      if inc_nums.len() == 2 {
        Some(inc_nums.into_iter().map(|i| schem.nums[i].1).product())
      } else { None }
    })
    .sum()
}

type Coord = (isize, isize);

#[derive(Debug)]
struct Number(Coord, usize);

#[derive(Debug, Default)]
struct Schematic {
  nums: Vec<Number>,
  num_find: HashMap<Coord, usize>,
  syms: HashMap<Coord, bool>,
}

fn parse(lines: impl Iterator<Item=String>) -> Schematic {
  let mut schem = Schematic::default();

  for (y, line) in lines.enumerate() {
    let y = y.try_into().unwrap();

    let mut num = None;
    for (x, char) in line.chars().enumerate() {
      let x = x.try_into().unwrap();

      if let Some(digit) = char.to_digit(10) {
        schem.num_find.insert((y, x), schem.nums.len());
        let digit = digit.try_into().unwrap();
        num = Some(match num {
          None => Number((y, x), digit),
          Some(Number(coord, old_digits)) => Number(coord, old_digits*10 + digit),
        });
      } else {
        if let Some(n) = num.take() { schem.nums.push(n); }
        if char != '.' {
          schem.syms.insert((y, x), char == '*');
        }
      }
    }
    schem.nums.extend(num.into_iter());
  }

  schem
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_util::sample_lines;

  #[test]
  fn test1_sample() {
    assert_eq!(part1(sample_lines("03a")), 4361);
  }

  #[test]
  fn test1() {
    assert_eq!(part1(sample_lines("03")), 514969);
  }

  #[test]
  fn test2_sample() {
    assert_eq!(part2(sample_lines("03a")), 467835);
  }

  #[test]
  fn test2() {
    assert_eq!(part2(sample_lines("03")), 78915902);
  }
}

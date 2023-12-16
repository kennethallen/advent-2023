pub fn part1(lines: impl Iterator<Item=String>) -> usize {
  let plat: Vec<Vec<_>> = lines
    .map(|line| line.chars().collect())
    .collect();
  north_load(&plat)
}

fn north_load(plat: &Vec<Vec<char>>) -> usize {
  (0..plat[0].len())
    .map(|x| {
      let mut load = 0;
      let mut next_slot = 0;
      for (y, row) in plat.iter().enumerate() {
        match row[x] {
          'O' => {
            load += plat.len() - next_slot;
            next_slot += 1;
          }
          '.' => (),
          '#' => next_slot = y+1,
          _ => panic!(),
        }
      }
      load
    })
    .sum()
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_util::sample_lines;

  #[test]
  fn test1_sample() {
    assert_eq!(part1(sample_lines("14a")), 136);
  }

  #[test]
  fn test1() {
    assert_eq!(part1(sample_lines("14")), 105623);
  }
}

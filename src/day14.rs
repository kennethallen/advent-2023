use std::collections::HashMap;

pub fn part1(lines: impl Iterator<Item=String>) -> usize {
  let plat = parse(lines);
  (0..plat[0].len())
    .map(|x| {
      let mut load = 0;
      let mut next_slot = 0;
      for (y, row) in plat.iter().enumerate() {
        match row[x] {
          Some(true) => {
            load += plat.len() - next_slot;
            next_slot += 1;
          }
          None => (),
          Some(false) => next_slot = y+1,
        }
      }
      load
    })
    .sum()
}

pub fn part2(lines: impl Iterator<Item=String>) -> usize {
  let target = 1_000_000_000;
  let plat = parse(lines);
  let mut log = vec![plat.clone()];
  let mut uniques = HashMap::from([(plat, 0)]);

  let prefix_len = loop {
    let mut plat = log.last().unwrap().clone();

    //N
    for x in 0..plat[0].len() {
      let mut next_slot = 0;
      for y in 0..plat.len() {
        match plat[y][x] {
          Some(true) => {
            plat[y][x] = None;
            plat[next_slot][x] = Some(true); // Do second in case stone isn't moving
            next_slot += 1;
          },
          None => (),
          Some(false) => next_slot = y+1,
        }
      }
    }
    //W
    for row in plat.iter_mut() {
      let mut next_slot = 0;
      for x in 0..row.len() {
        match row[x] {
          Some(true) => {
            row[x] = None;
            row[next_slot] = Some(true); // Do second in case stone isn't moving
            next_slot += 1;
          },
          None => (),
          Some(false) => next_slot = x+1,
        }
      }
    }
    //S
    for x in 0..plat[0].len() {
      let mut next_slot = plat.len()-1;
      for y in (0..plat.len()).rev() {
        match plat[y][x] {
          Some(true) => {
            plat[y][x] = None;
            plat[next_slot][x] = Some(true); // Do second in case stone isn't moving
            next_slot = next_slot.saturating_sub(1);
          },
          None => (),
          Some(false) => next_slot = y.saturating_sub(1),
        }
      }
    }
    //E
    for row in plat.iter_mut() {
      let mut next_slot = row.len()-1;
      for x in (0..row.len()).rev() {
        match row[x] {
          Some(true) => {
            row[x] = None;
            row[next_slot] = Some(true); // Do second in case stone isn't moving
            next_slot = next_slot.saturating_sub(1);
          },
          None => (),
          Some(false) => next_slot = x.saturating_sub(1),
        }
      }
    }

    if let Err(e) = uniques.try_insert(plat.clone(), uniques.len()) {
      break *e.entry.get();
    }
    log.push(plat);
  };
  let cycle_len = uniques.len() - prefix_len;
  let target_i = if target < prefix_len { target } else { (target-prefix_len) % cycle_len + prefix_len };

  log[target_i].iter()
    .enumerate()
    .map(|(y, row)|
      row.iter().filter(|&&tile| tile == Some(true)).count()
      * (log[0].len() - y))
    .sum()
}

fn parse(lines: impl Iterator<Item=String>) -> Vec<Vec<Option<bool>>> {
  lines
    .map(|line| line.chars()
      .map(|c| match c {
        'O' => Some(true),
        '#' => Some(false),
        '.' => None,
        _ => panic!(),
      })
      .collect())
    .collect()
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

  #[test]
  fn test2_sample() {
    assert_eq!(part2(sample_lines("14a")), 64);
  }

  #[test]
  fn test2() {
    assert_eq!(part2(sample_lines("14")), 98029);
  }
}

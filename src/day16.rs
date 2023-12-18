use std::collections::HashMap;

use enum_map::{EnumMap, Enum};

use crate::util::Coord;

pub fn part1(lines: impl Iterator<Item=String>) -> usize {
  let map: Vec<Vec<char>> = lines
    .map(|line| line.chars().collect())
    .collect();

  let mut energized = HashMap::new();
  energized.insert((0, 0), {
    let mut east = EnumMap::default();
    east[Dir::E] = true;
    east
  });
  let mut to_advance = vec![((0, 0), Dir::E)];
  while let Some((pos, dir)) = to_advance.pop() {
    let leave_dirs: &[Dir] = match (dir, map[pos.0][pos.1]) {
      (Dir::E | Dir::W, '|') => &[Dir::N, Dir::S],
      (Dir::N | Dir::S, '-') => &[Dir::E, Dir::W],
      (Dir::E, '.' | '-') => &[Dir::E],
      (Dir::N, '.' | '|') => &[Dir::N],
      (Dir::W, '.' | '-') => &[Dir::W],
      (Dir::S, '.' | '|') => &[Dir::S],
      (Dir::E, '/') => &[Dir::N],
      (Dir::N, '/') => &[Dir::E],
      (Dir::W, '/') => &[Dir::S],
      (Dir::S, '/') => &[Dir::W],
      (Dir::E, '\\') => &[Dir::S],
      (Dir::S, '\\') => &[Dir::E],
      (Dir::W, '\\') => &[Dir::N],
      (Dir::N, '\\') => &[Dir::W],
      _ => panic!(),
    };
    for &leave_dir in leave_dirs {
      if let Some(new_pos) = try_step(pos, leave_dir, (map.len(), map[0].len())) {
        match energized.try_insert(new_pos, EnumMap::default()) {
          Ok(dirs) => {
            dirs[leave_dir] = true;
            to_advance.push((new_pos, leave_dir));
          },
          Err(mut e) => {
            let dirs = e.entry.get_mut();
            if !dirs[leave_dir] {
              dirs[leave_dir] = true;
              to_advance.push((new_pos, leave_dir));
            }
          },
        }
      }
    }
  }
  dbg!(&energized);
  energized.len()
}

// TODO Refactor to share with day 08
fn try_step((y, x): Coord, dir: Dir, (max_y, max_x): Coord) -> Option<Coord> {
  match dir {
    Dir::E => x.checked_add(1).map(|x| (y, x)).filter(|&(_, x)| x < max_x),
    Dir::N => y.checked_sub(1).map(|y| (y, x)),
    Dir::W => x.checked_sub(1).map(|x| (y, x)),
    Dir::S => y.checked_add(1).map(|y| (y, x)).filter(|&(y, _)| y < max_y),
  }
}

#[derive(Enum, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum Dir { E, N, W, S }


#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_util::sample_lines;

  #[test]
  fn test1_sample() {
    assert_eq!(part1(sample_lines("16a")), 46);
  }

  #[test]
  fn test1() {
    assert_eq!(part1(sample_lines("16")), 7185);
  }

  /*#[test]
  fn test2_sample() {
    assert_eq!(part2(sample_lines("12a")), 525152);
  }

  #[test]
  fn test2() {
    assert_eq!(part2(sample_lines("12")), 1493340882140);
  }*/
}

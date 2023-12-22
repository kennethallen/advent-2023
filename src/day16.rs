use std::{collections::HashMap, iter::once};

use enum_map::{EnumMap, Enum};

use crate::util::Coord;

pub fn part1(lines: impl Iterator<Item=String>) -> usize {
  process(&parse(lines), (0, 0), Dir::E)
}
pub fn part2(lines: impl Iterator<Item=String>) -> usize {
  let map = parse(lines);
  (0..map.len())
    .flat_map(|y| once(((y, 0), Dir::E)).chain(once(((y, map[0].len()-1), Dir::W))))
    .chain(
      (0..map[0].len())
        .flat_map(|x| once(((0, x), Dir::S)).chain(once(((map.len()-1, x), Dir::N))))
    )
    .map(|(pos, dir)| process(&map, pos, dir))
    .max().unwrap()
}

fn parse(lines: impl Iterator<Item=String>) -> Vec<Vec<Tile>> {
  lines
    .map(|line| line.chars().map(Tile::try_parse).map(Option::unwrap).collect())
    .collect()
}

fn process(map: &Vec<Vec<Tile>>, init_pos: Coord, init_dir: Dir) -> usize {
  let mut energized = HashMap::new();
  energized.insert(init_pos, {
    let mut init_dirs = EnumMap::default();
    init_dirs[init_dir] = true;
    init_dirs
  });
  let mut to_advance = vec![(init_pos, init_dir)];
  while let Some((pos, dir)) = to_advance.pop() {
    let leave_dirs: &[Dir] = match (dir, map[pos.0][pos.1]) {
      (Dir::E | Dir::W, Tile::SplitNS) => &[Dir::N, Dir::S],
      (Dir::N | Dir::S, Tile::SplitEW) => &[Dir::E, Dir::W],
      (Dir::E, Tile::Empty | Tile::SplitEW) => &[Dir::E],
      (Dir::N, Tile::Empty | Tile::SplitNS) => &[Dir::N],
      (Dir::W, Tile::Empty | Tile::SplitEW) => &[Dir::W],
      (Dir::S, Tile::Empty | Tile::SplitNS) => &[Dir::S],
      (Dir::E, Tile::MirrorEN) => &[Dir::N],
      (Dir::N, Tile::MirrorEN) => &[Dir::E],
      (Dir::W, Tile::MirrorEN) => &[Dir::S],
      (Dir::S, Tile::MirrorEN) => &[Dir::W],
      (Dir::E, Tile::MirrorES) => &[Dir::S],
      (Dir::S, Tile::MirrorES) => &[Dir::E],
      (Dir::W, Tile::MirrorES) => &[Dir::N],
      (Dir::N, Tile::MirrorES) => &[Dir::W],
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
  /*for y in 0..map.len() {
    for x in 0..map[0].len() {
      print!("{}", match energized.get(&(y, x)) {
        None => '.',
        Some(dirs) => match dirs.iter().filter(|&(_, &v)| v).map(|(k, _)| k).exactly_one() {
          Ok(Dir::E) => '>',
          Ok(Dir::N) => '^',
          Ok(Dir::W) => '<',
          Ok(Dir::S) => 'v',
          Err(_) => format!("{}", dirs.values().filter(|&&b| b).count()).chars().exactly_one().unwrap(),
        }
      });
    }
    println!();
  }*/
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

#[derive(Enum, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Dir { E, N, W, S }

#[derive(Clone, Copy)]
enum Tile { Empty, MirrorEN, MirrorES, SplitEW, SplitNS }

impl Tile {
  fn try_parse(c: char) -> Option<Self> {
    match c {
      '.' => Some(Self::Empty),
      '|' => Some(Self::SplitNS),
      '-' => Some(Self::SplitEW),
      '/' => Some(Self::MirrorEN),
      '\\' => Some(Self::MirrorES),
      _ => None,
    }
  }
}

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

  #[test]
  fn test2_sample() {
    assert_eq!(part2(sample_lines("16a")), 51);
  }

  #[test]
  fn test2() {
    assert_eq!(part2(sample_lines("16")), 7616);
  }
}

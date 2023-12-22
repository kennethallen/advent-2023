use std::{collections::{BinaryHeap, HashMap}, cmp::Ordering};

use crate::util::Coord;

pub fn part1(lines: impl Iterator<Item=String>) -> usize {
  let map: Vec<Vec<u8>> = lines
    .map(|line| line.chars()
      .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
      .collect())
    .collect();

  let bounds = (map.len(), map[0].len());
  let dest = (bounds.0-1, bounds.1-1);
  let mut seen = HashMap::new();
  let mut to_explore = BinaryHeap::from([State {
    //prev: (usize::MAX, usize::MAX),
    pos: (0, 0),
    face: Dir::E,
    run: 0,
    heat_loss: 0,
  }]);
  let x = loop {
    let state = to_explore.pop().unwrap();
    if let Err(mut e) = seen.try_insert((state.pos, state.face), state.run) {
      let old_state = e.entry.get_mut();
      if *old_state <= state.run {
        continue;
      }
      *old_state = e.value;
    }
    //println!("{:?} {:?}", state.pos, state.prev);
    if state.pos == dest { break state.heat_loss; }
    let next_dirs = match state.face {
      Dir::E => [Dir::N, Dir::E, Dir::S],
      Dir::N => [Dir::W, Dir::N, Dir::E],
      Dir::W => [Dir::S, Dir::W, Dir::N],
      Dir::S => [Dir::E, Dir::S, Dir::W],
    };
    for face in next_dirs {
      if let Some(pos) = try_step(state.pos, face, bounds) {
        let run = if face == state.face { state.run + 1 } else { 1 };
        if run <= 3 {
          to_explore.push(State { pos, face, run, 
            //prev: state.pos, 
            heat_loss: state.heat_loss + usize::from(map[pos.0][pos.1]),
          })
        }
      }
    }
  };
  //let mut curs = seen.get(&dest);
  //let mut path = HashSet::new();
  //while let Some(state) = curs {
    //path.insert(state.pos);
    //curs = seen.get(&state.prev);
  //}
  //for y in 0..bounds.0 {
    //for x in 0..bounds.1 {
      //print!("{}", if path.contains(&(y, x)) { '#' } else { '.' });
    //}
    //println!();
  //}
  x
}

#[derive(Debug, Clone)]
struct State {
  pos: Coord,
  face: Dir,
  run: u8,
  heat_loss: usize,
  //prev: Coord,
}

impl PartialEq for State {
  fn eq(&self, other: &Self) -> bool {
    self.heat_loss == other.heat_loss
  }
}

impl Eq for State { }

impl PartialOrd for State {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for State {
  fn cmp(&self, other: &Self) -> Ordering {
    other.heat_loss.cmp(&self.heat_loss)
  }
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
enum Dir { E, N, W, S }

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_util::sample_lines;

  #[test]
  fn test1_sample() {
    assert_eq!(part1(sample_lines("17a")), 102);
  }

  #[test]
  fn test1() {
    assert_eq!(part1(sample_lines("17")), 1023);
  }

  /*#[test]
  fn test2_sample() {
    assert_eq!(part2(sample_lines("17a")), 51);
  }

  #[test]
  fn test2() {
    assert_eq!(part2(sample_lines("17")), 7616);
  }*/
}

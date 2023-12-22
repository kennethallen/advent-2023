use std::{collections::{BinaryHeap, HashMap}, cmp::Ordering};

use crate::util::Coord;

pub fn part1(lines: impl Iterator<Item=String>) -> usize { process(lines, 1, 3) }
pub fn part2(lines: impl Iterator<Item=String>) -> usize { process(lines, 4, 10) }

fn process(lines: impl Iterator<Item=String>, min_run: u8, max_run: u8) -> usize {
  let map: Vec<Vec<u8>> = lines
    .map(|line| line.chars()
      .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
      .collect())
    .collect();

  let bounds = (map.len(), map[0].len());
  let dest = (bounds.0-1, bounds.1-1);
  let mut seen = HashMap::new();
  let mut to_explore = BinaryHeap::from([State::default()]);
  loop {
    let state = to_explore.pop().unwrap();
    if state.pos == dest { break state.heat_loss; }

    if let Err(mut e) = seen.try_insert((state.pos, state.face), state.run) {
      let old_state = e.entry.get_mut();
      if *old_state <= state.run {
        continue;
      }
      *old_state = e.value;
    }

    'try_turn: for face in [state.face.turn_ccw(), state.face.turn_cw()] {
      let mut heat_loss = state.heat_loss;
      let mut pos = state.pos;
      for _ in 0..min_run {
        pos = if let Some(pos) = try_step(pos, face, bounds) { pos } else { continue 'try_turn };
        heat_loss += usize::from(map[pos.0][pos.1]);
      }
      to_explore.push(State { pos, face, heat_loss, run: min_run });
    }
    if state.run < max_run && let Some(pos) = try_step(state.pos, state.face, bounds) {
      to_explore.push(State { pos,
        face: state.face,
        run: state.run + 1, 
        heat_loss: state.heat_loss + usize::from(map[pos.0][pos.1]),
      });
    }
  }
}

#[derive(Debug, Clone, Default)]
struct State {
  pos: Coord,
  face: Dir,
  run: u8,
  heat_loss: usize,
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash, Default)]
enum Dir { #[default] E, N, W, S }

impl Dir {
  fn turn_ccw(&self) -> Self {
    match self {
      Self::E => Self::N,
      Self::N => Self::W,
      Self::W => Self::S,
      Self::S => Self::E,
    }
  }

  fn turn_cw(&self) -> Self {
    match self {
      Self::E => Self::S,
      Self::N => Self::E,
      Self::W => Self::N,
      Self::S => Self::W,
    }
  }
}

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

  #[test]
  fn test2_sample() {
    assert_eq!(part2(sample_lines("17a")), 94);
    assert_eq!(part2(sample_lines("17b")), 71);
  }

  #[test]
  fn test2() {
    assert_eq!(part2(sample_lines("17")), 1165);
  }
}

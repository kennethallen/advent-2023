use std::collections::{HashMap, HashSet, VecDeque};

use nom::{IResult, sequence::{terminated, preceded}, combinator::map, bytes::complete::{tag, take_while}, character::complete::char, branch::alt, multi::separated_list1};

pub fn part1(lines: impl Iterator<Item=String>) -> usize {
  let lines: Vec<_> = lines.collect();
  let mut mods: HashMap<_, _> = lines.iter()
    .map(|line| parse(line.as_str()).unwrap().1)
    .collect();

  let mut backrefs = HashMap::new();
  for (&name, module) in &mods {
    for &dest in &module.dests {
      (match backrefs.try_insert(dest, vec![]) {
        Ok(v) => v,
        Err(e) => e.entry.into_mut(),
      }).push(name);
    }
  }

  let mut pulses = VecDeque::new();
  let mut lows = 0;
  let mut highs = 0;
  for _ in 0..1000 {
    pulses.push_back((None, "broadcaster", false));
    while let Some((from, to, hi)) = pulses.pop_front() {
      *(if hi { &mut highs } else { &mut lows }) += 1;
      if let Some(module) = mods.get_mut(to) {
        let sig = match &mut module.logic {
          ModuleLogic::FlipFlop(mem) => if hi { None } else { *mem = !*mem; Some(*mem) },
          ModuleLogic::Conjunction(mem) => {
            if hi { mem.insert(from.unwrap()); } else { mem.remove(from.unwrap()); }
            Some(if mem.len() == backrefs[to].len() { false } else { true })
          },
          ModuleLogic::Broadcast => Some(hi),
        };
        if let Some(sig) = sig {
          pulses.extend(module.dests.iter().map(|&dest| (Some(to), dest, sig)));
        }
      }
    }
  }

  lows * highs
}

struct Module<'a> {
  logic: ModuleLogic<'a>,
  dests: Vec<&'a str>,
}

enum ModuleLogic<'a> {
  FlipFlop(bool),
  Conjunction(HashSet<&'a str>),
  Broadcast,
}

fn parse<'a>(input: &'a str) -> IResult<&'a str, (&'a str, Module<'a>)> {
  let (input, (name, logic)) = terminated(
    alt((
      map(
        tag("broadcaster"),
        |name| (name, ModuleLogic::Broadcast),
      ),
      map(
        preceded(char('%'), take_while(char::is_lowercase)),
        |name| (name, ModuleLogic::FlipFlop(false)),
      ),
      map(
        preceded(char('&'), take_while(char::is_lowercase)),
        |name| (name, ModuleLogic::Conjunction(Default::default())),
      ),
    )),
    tag(" -> "),
  )(input)?;
  let (input, dests) = separated_list1(
    tag(", "),
    take_while(char::is_lowercase),
  )(input)?;
  Ok((input, (name, Module { logic, dests })))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_util::sample_lines;

  #[test]
  fn test1_sample() {
    assert_eq!(part1(sample_lines("20a")), 32000000);
  }

  #[test]
  fn test1() {
    assert_eq!(part1(sample_lines("20")), 787056720);
  }
}

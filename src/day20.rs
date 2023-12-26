use std::collections::{HashMap, HashSet, VecDeque};

use nom::{IResult, sequence::{terminated, preceded}, combinator::map, bytes::complete::{tag, take_while}, character::complete::char, branch::alt, multi::separated_list1};
use num_integer::lcm;

pub fn part1(lines: impl Iterator<Item=String>) -> usize {
  let lines: Vec<_> = lines.collect();
  let (mut mods, backrefs) = parse(lines.iter());

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

fn parse<'a>(lines: impl Iterator<Item=&'a String>) -> (HashMap<&'a str, Module<'a>>, HashMap<&'a str, Vec<&'a str>>) {
  let mods: HashMap<_, _> = lines
    .map(|line| parse_module(line.as_str()).unwrap().1)
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

  (mods, backrefs)
}

pub fn part2(lines: impl Iterator<Item=String>) -> usize {
  let lines: Vec<_> = lines.collect();
  let (mut mods, backrefs) = parse(lines.iter());

  println!("digraph D {{");
  println!("rx [shape=square]");
  for (&name, module) in &mods {
    println!("{} [shape={}]", name, match module.logic {
      ModuleLogic::Broadcast => "circle",
      ModuleLogic::FlipFlop(_) => "diamond",
      ModuleLogic::Conjunction(_) => "triangle",
    });
  }
  for (&name, module) in &mods {
    for &dest in &module.dests {
      println!("{} -> {}", name, dest);
    }
  }
  println!("}}");

  let mut pulses = VecDeque::new();
  let mut presses = 0;
  let chains = [
    ("sn", "fh", 0b111000001111, ["ng", "vp", "vf", "th", "qb", "nc", "sd", "nl", "bt", "xd", "tn", "tv"]),
    ("lr", "ss", 0b100101001111, ["js", "dc", "dp", "xv", "rm", "hj", "bq", "gk", "hm", "rd", "xl", "gx"]),
    ("tf", "fz", 0b100010110111, ["gr", "cb", "jg", "qn", "td", "zj", "vr", "hq", "kb", "mq", "fl", "gz"]),
    ("hl", "mf", 0b100011010111, ["lb", "nj", "xx", "hb", "qk", "hs", "fp", "xb", "tl", "kg", "px", "tm"]),
  ];
  let mut chain_periods = vec![None; chains.len()];
  loop {
    pulses.push_back((None, "broadcaster", false));
    presses += 1;
    while let Some((from, to, hi)) = pulses.pop_front() {
      //if to == "rx" && !hi { break 'presses presses; }
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

    for (i, (_, _, _, accs)) in chains.iter().enumerate() {
      let zero = accs.iter()
        .map(|&tag| &mods[tag])
        .all(|module| module.logic == ModuleLogic::FlipFlop(false));
      if zero && chain_periods[i].is_none() {
        chain_periods[i] = Some(presses);
      }
    }
    if chain_periods.iter().all(Option::is_some) {
      break chain_periods.into_iter()
        .map(Option::unwrap)
        .reduce(|a, b| lcm(a, b))
        .unwrap();
    }
    //if acc_bits.iter().any(|&b| b == 0) {
      //dbg!(presses);
      //for &(cond, inv, read_mask, accs) in &chains {
        //println!("cond {:?} inv {:?}", mods[cond].logic, mods[inv].logic);
        //for (_, &acc) in accs.iter().enumerate() {
          //print!("{}", if let ModuleLogic::FlipFlop(mem) = mods[acc].logic { if mem { '1' } else { '0' } } else { panic!(); });
        //}
        //print!(" ");
        //for (i, &acc) in accs.iter().enumerate() {
          //let read = (read_mask & 1<<(11-i)) != 0;
          //print!("{}", if !read { '.' } else if let ModuleLogic::FlipFlop(mem) = mods[acc].logic { if mem { '1' } else { '0' } } else { panic!(); });
        //}
        //print!(" ");
        //for (i, &acc) in accs.iter().enumerate() {
          //let read = (read_mask & 1<<(11-i)) != 0;
          //let write = !read || i == 0;
          //print!("{}", if !write { '.' } else if let ModuleLogic::FlipFlop(mem) = mods[acc].logic { if mem { '1' } else { '0' } } else { panic!(); });
        //}
        //println!();
        //acc_bits.push(accs.iter()
          //.map(|&tag| &mods[tag])
          //.enumerate()
          //.map(|(i, module)| {
            //let mem = if let ModuleLogic::FlipFlop(mem) = module.logic { mem } else { unreachable!(); };
            //if mem { 1 << i } else { 0 }
          //})
          //.sum());
      //}
      //println!();
    //}
    //if presses > (1 << 14) { panic!(); }
  }
}

#[derive(Debug)]
struct Module<'a> {
  logic: ModuleLogic<'a>,
  dests: Vec<&'a str>,
}

#[derive(Debug, PartialEq, Eq)]
enum ModuleLogic<'a> {
  FlipFlop(bool),
  Conjunction(HashSet<&'a str>),
  Broadcast,
}

fn parse_module<'a>(input: &'a str) -> IResult<&'a str, (&'a str, Module<'a>)> {
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

  #[test]
  fn test2() {
    assert_eq!(part2(sample_lines("20")), 212986464842911);
  }
}

use std::{collections::HashMap, ops::RangeInclusive, cmp::{max, min}};

use crate::util::usize;

use enum_map::{EnumMap, Enum, enum_map};
use nom::{IResult, character::complete::{char, line_ending, one_of}, multi::{many0, many1, separated_list1}, sequence::{terminated, pair, separated_pair, delimited}, combinator::{eof, map, value}, bytes::complete::take_while, branch::alt};

pub fn part1(file: String) -> usize {
  let (workflows, parts) = parse(file.as_str()).unwrap().1;
  let in_workflow = &workflows["in"];
  parts.into_iter()
    .filter(|part| {
      let mut workflow = in_workflow;
      loop {
        match workflow.process(part) {
          Dest::Accept => break true,
          Dest::Reject => break false,
          Dest::Continue(next) => workflow = &workflows[next],
        }
      }
    })
    .flat_map(|part| part.into_values())
    .sum()
}

pub fn part2(file: String) -> usize {
  let (workflows, _) = parse(file.as_str()).unwrap().1;
  workflows["in"].count_acceptances(
    enum_map! { _ => 1..=4000 },
    |tag| &workflows[tag],
  )
}

#[derive(Clone, Copy)]
enum Dest<'a> {
  Accept,
  Reject,
  Continue(&'a str),
}

impl<'a> Dest<'a> {
  fn parse(input: &'a str) -> IResult<&'a str, Self> {
    alt((
      value(Self::Accept, char('A')),
      value(Self::Reject, char('R')),
      map(parse_name, |tag| Self::Continue(tag)),
    ))(input)
  }

  fn count_acceptances(&self, part_spec: PartSpec, workflow_lookup: impl Copy + Fn(&'a str) -> &Workflow<'a>) -> usize {
    match self {
      Self::Reject => 0,
      Self::Accept => part_spec.into_values().map(|r| r.end() - r.start() + 1).product(),
      Self::Continue(tag) => workflow_lookup(tag).count_acceptances(part_spec, workflow_lookup),
    }
  }
}

#[derive(Enum, Clone, Copy)]
enum Qual { X, M, A, S }

impl Qual {
  fn parse(input: &str) -> IResult<&str, Self> {
    map(
      one_of("xmas"),
      |c| match c { 'x' => Self::X, 'm' => Self::M, 'a' => Self::A, 's' => Self::S, _ => unreachable!() },
    )(input)
  }
}

struct Cond {
  qual: Qual,
  high_pass: bool,
  thresh: usize,
}

impl Cond {
  fn parse(input: &str) -> IResult<&str, Self> {
    let (input, qual) = Qual::parse(input)?;
    let (input, high_pass) = alt((
      value(true, char('>')),
      value(false, char('<')),
    ))(input)?;
    let (input, thresh) = usize(input)?;
    Ok((input, Self { qual, high_pass, thresh }))
  }

  fn test(&self, part: &Part) -> bool {
    if self.high_pass {
      part[self.qual] > self.thresh
    } else {
      part[self.qual] < self.thresh
    }
  }

  fn split(&self, mut spec: PartSpec) -> (Option<PartSpec>, Option<PartSpec>) {
    let mut spec_alt = spec.clone();
    let (start, end) = spec[self.qual].clone().into_inner();
    if self.high_pass {
      (
        if end > self.thresh { spec[self.qual] = max(start, self.thresh+1)..=end; Some(spec) } else { None },
        if start <= self.thresh { spec_alt[self.qual] = start..=min(end, self.thresh); Some(spec_alt) } else { None },
      )
    } else {
      (
        if start < self.thresh { spec[self.qual] = start..=min(end, self.thresh-1); Some(spec) } else { None },
        if end >= self.thresh { spec_alt[self.qual] = max(start, self.thresh)..=end; Some(spec_alt) } else { None },
      )
    }
  }
}

struct Workflow<'a> {
  steps: Vec<(Cond, Dest<'a>)>,
  fallback: Dest<'a>,
}

impl<'a> Workflow<'a> {
  fn parse(input: &'a str) -> IResult<&'a str, Self> {
    let (input, steps) = many0(terminated(
        separated_pair(
          Cond::parse,
          char(':'),
          Dest::parse,
        ),
        char(','),
    ))(input)?;
    let (input, fallback) = Dest::parse(input)?;
    Ok((input, Self { steps, fallback }))
  }

  fn process(&self, part: &Part) -> Dest<'a> {
    self.steps.iter()
      .find(|(cond, _)| cond.test(part))
      .map(|&(_, dest)| dest)
      .unwrap_or(self.fallback)
  }

  fn count_acceptances(&self, mut part_spec: PartSpec, workflow_lookup: impl Copy + Fn(&'a str) -> &Workflow<'a>) -> usize {
    let mut sum = 0;
    for (cond, dest) in &self.steps {
      let (pass, fail) = cond.split(part_spec);
      if let Some(pass) = pass {
        sum += dest.count_acceptances(pass, workflow_lookup);
      }
      if let Some(fail) = fail {
        part_spec = fail;
      } else {
        return sum;
      }
    }
    sum + self.fallback.count_acceptances(part_spec, workflow_lookup)
  }
}

type Part = EnumMap<Qual, usize>;
type PartSpec = EnumMap<Qual, RangeInclusive<usize>>;

fn parse_part(input: &str) -> IResult<&str, Part> {
  let (input, quals) = separated_list1(
    char(','),
    separated_pair(Qual::parse, char('='), usize),
  )(input)?;
  let mut part = EnumMap::default();
  for (qual, val) in quals {
    part[qual] = val;
  }
  Ok((input, part))
}

fn parse(input: &str) -> IResult<&str, (HashMap<&str, Workflow>, Vec<Part>)> {
  pair(
    terminated(
      map(
        many1(terminated(
          separated_pair(
            parse_name,
            char('{'),
            Workflow::parse,
          ),
          pair(char('}'), line_ending),
        )),
        |ws| ws.into_iter().collect(),
      ),
      line_ending,
    ),
    terminated(
      many1(delimited(
        char('{'),
        parse_part,
        pair(char('}'), line_ending),
      )),
      eof,
    ),
  )(input)
}

fn parse_name(input: &str) -> IResult<&str, &str> {
  take_while(char::is_lowercase)(input)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_util::sample_file;

  #[test]
  fn test1_sample() {
    assert_eq!(part1(sample_file("19a")), 19114);
  }

  #[test]
  fn test1() {
    assert_eq!(part1(sample_file("19")), 368964);
  }

  #[test]
  fn test2_sample() {
    assert_eq!(part2(sample_file("19a")), 167409079868000);
  }

  #[test]
  fn test2() {
    assert_eq!(part2(sample_file("19")), 127675188176682);
  }
}

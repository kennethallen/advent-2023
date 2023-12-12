use crate::util::usize;

use enum_map::{Enum, EnumMap};
use nom::{IResult, character::complete::{char, one_of}, multi::count, sequence::{terminated, separated_pair}, combinator::{eof, map}};

pub fn part1(lines: impl Iterator<Item=String>) -> usize { process(lines, false) }
pub fn part2(lines: impl Iterator<Item=String>) -> usize { process(lines, true) }

fn process(lines: impl Iterator<Item=String>, jokers: bool) -> usize {
  let mut hands: Vec<_> = lines
    .map(|line| Hand::parse(line.as_str(), jokers).unwrap().1)
    .collect();

  hands.sort();
  
  hands.into_iter()
    .enumerate()
    .map(|(i, hand)| hand.bid * (i+1))
    .sum()
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Enum, Debug, Clone, Copy)]
enum Card {
  Joker, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
}

impl Card {
  fn parse(input: &str, jokers: bool) -> IResult<&str, Self> {
    map(
      one_of("23456789TJQKA"),
      |c| match c {
        '2' => Card::Two,
        '3' => Card::Three,
        '4' => Card::Four,
        '5' => Card::Five,
        '6' => Card::Six,
        '7' => Card::Seven,
        '8' => Card::Eight,
        '9' => Card::Nine,
        'T' => Card::Ten,
        'J' => if jokers { Card::Joker } else { Card::Jack },
        'Q' => Card::Queen,
        'K' => Card::King,
        'A' => Card::Ace,
        _ => unreachable!(),
      }
    )(input)
  }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
  hand_type: HandType,
  cards: [Card; 5],
  bid: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
  HighCard, OnePair, TwoPair, ThreeOfAKind, FullHouse, FourOfAKind, FiveOfAKind,
}

impl HandType {
  fn calc(cards: &[Card; 5]) -> HandType {
    let mut counts: EnumMap<Card, usize> = EnumMap::default();
    for &card in cards {
      counts[card] += 1;
    }

    let jokers = counts[Card::Joker];

    let mut metacounts = [0usize; 6];
    for (card, count) in counts.into_iter() {
      if card != Card::Joker {
        metacounts[count] += 1;
      }
    }

    match jokers + metacounts.iter().rposition(|&n| n > 0).unwrap() {
      5 => HandType::FiveOfAKind,
      4 => HandType::FourOfAKind,
      3 => match (jokers, metacounts[2]) {
        (2, 0) => HandType::ThreeOfAKind,
        (1, 2) => HandType::FullHouse,
        (1, 1) => HandType::ThreeOfAKind,
        (0, 1) => HandType::FullHouse,
        (0, 0) => HandType::ThreeOfAKind,
        _ => unreachable!(),
      },
      2 => match (jokers, metacounts[2]) {
        (1, 0) => HandType::OnePair,
        (0, 2) => HandType::TwoPair,
        (0, 1) => HandType::OnePair,
        _ => unreachable!(),
      }
      1 => HandType::HighCard,
      _ => unreachable!(),
    }
  }
}

impl Hand {
  fn parse(input: &str, jokers: bool) -> IResult<&str, Self> {
    map(
      terminated(
        separated_pair(
          count(|input| Card::parse(input, jokers), 5),
          char(' '),
          usize,
        ),
        eof,
      ),
      |(cards, bid)| {
        let cards = cards.try_into().unwrap();
        Self { cards, bid, hand_type: HandType::calc(&cards) }
      },
    )(input)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_util::sample_lines;

  #[test]
  fn test1_sample() {
    assert_eq!(part1(sample_lines("07a")), 6440);
  }

  #[test]
  fn test1() {
    assert_eq!(part1(sample_lines("07")), 250058342);
  }

  #[test]
  fn test2_sample() {
    assert_eq!(part2(sample_lines("07a")), 5905);
  }

  #[test]
  fn test2() {
    assert_eq!(part2(sample_lines("07")), 250506580);
  }
}

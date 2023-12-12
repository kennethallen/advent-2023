use crate::util::usize;

use enum_map::{Enum, EnumMap};
use nom::{IResult, character::complete::{char, one_of}, multi::count, sequence::{terminated, separated_pair}, combinator::{eof, map}};

pub fn part1(lines: impl Iterator<Item=String>) -> usize {
  let mut hands: Vec<_> = lines
    .map(|line| Hand::parse(line.as_str()).unwrap().1)
    .collect();

  hands.sort();
  
  hands.into_iter()
    .enumerate()
    .map(|(i, hand)| hand.bid * (i+1))
    .sum()
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Enum, Debug, Clone, Copy)]
enum Card {
  Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
}

impl Card {
  fn parse(input: &str) -> IResult<&str, Self> {
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
        'J' => Card::Jack,
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
    for card in cards {
      counts[*card] += 1;
    }

    let mut metacounts = [0usize; 6];
    for count in counts.into_values() {
      metacounts[count] += 1;
    }

    if metacounts[5] >= 1 {
      HandType::FiveOfAKind
    } else if metacounts[4] >= 1 {
      HandType::FourOfAKind
    } else if metacounts[3] >= 1 {
      if metacounts[2] >= 1 {
        HandType::FullHouse
      } else {
        HandType::ThreeOfAKind
      }
    } else if metacounts[2] >= 1 {
      if metacounts[2] >= 2 {
        HandType::TwoPair
      } else {
        HandType::OnePair
      }
    } else {
      HandType::HighCard
    }
  }
}

impl Hand {
  fn parse(input: &str) -> IResult<&str, Self> {
    map(
      terminated(
        separated_pair(
          count(Card::parse, 5),
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
}

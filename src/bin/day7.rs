use std::{collections::HashMap, fmt::Display, ops::Deref, str::FromStr};

use aoc2023::aoc;

#[derive(PartialEq, Eq, Clone, Copy)]
struct Hand([u8; 5]);

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.0 {
            write!(f, "{}", c as char)?
        }
        Ok(())
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank_pair().cmp(&other.rank_pair())
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    pub fn hand_type(&self) -> HandType {
        let counts = self.0.iter().fold(HashMap::new(), |mut freq, c| {
            *freq.entry(*c).or_insert(0u8) += 1;
            freq
        });

        let distinct = counts.len();
        let max_freq = counts.values().copied().max().unwrap();

        match (distinct, max_freq) {
            (1, _) => HandType::FiveOfAKind,
            (2, 4) => HandType::FourOfAKind,
            (2, 3) => HandType::FullHouse,
            (3, 3) => HandType::ThreeOfAKind,
            (3, 2) => HandType::TwoPair,
            (4, _) => HandType::OnePair,
            (5, _) => HandType::High,
            _ => unreachable!(),
        }
    }

    fn rank_array(&self) -> [u8; 5] {
        self.0.map(|c| card_rank(c))
    }

    fn rank_pair(&self) -> (u8, [u8; 5]) {
        (self.hand_type().rank(), self.rank_array())
    }
}

#[derive(Debug, Clone, Copy)]
enum HandType {
    FiveOfAKind = 0, // 1 distinct, _
    FourOfAKind,     // 2 distinct, max freq 4
    FullHouse,       // 2 distinct, max freq 3
    ThreeOfAKind,    // 3 distinct, max freq 3
    TwoPair,         // 3 distinct, max freq 2
    OnePair,         // 4 distinct, _
    High,            // 5 distinct, _
}

impl HandType {
    pub const fn rank(&self) -> u8 {
        (HandType::High as u8) - (*self as u8)
    }
}

fn card_rank(c: u8) -> u8 {
    match c {
        b'A' => 14,
        b'K' => 13,
        b'Q' => 12,
        b'J' => 11,
        b'T' => 10,
        b'2'..=b'9' => c - b'0',
        _ => panic!("Invalid card {}", c),
    }
}

#[cfg(test)]
mod test {
    use crate::card_rank;

    #[test]
    fn test_card_rank() {
        assert_eq!(card_rank(b'2'), 2);
        assert_eq!(card_rank(b'9'), 9);
    }
}

#[derive(Clone, Copy)]
struct Round {
    hand: Hand,
    bid: usize,
}

#[derive(Debug)]
struct ParseRoundError(String);

impl FromStr for Round {
    type Err = ParseRoundError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s
            .split_once(' ')
            .ok_or_else(|| ParseRoundError(s.to_owned()))?;

        Ok(Self {
            hand: Hand(
                hand.as_bytes()
                    .try_into()
                    .map_err(|_| ParseRoundError(s.to_owned()))?,
            ),
            bid: bid.parse().map_err(|_| ParseRoundError(s.to_owned()))?,
        })
    }
}

struct Rounds(Vec<Round>);

impl FromStr for Rounds {
    type Err = ParseRoundError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        ))
    }
}

impl Deref for Rounds {
    type Target = Vec<Round>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

mod part1 {
    use super::*;

    pub fn calculate(rounds: &Rounds) -> usize {
        let mut ranked: Vec<_> = rounds.0.clone();

        ranked.sort_by(|a, b| a.hand.cmp(&b.hand));

        ranked
            .iter()
            .enumerate()
            .map(|(i, round)| {
                #[cfg(test)]
                println!(
                    "{} -> {} {}, {:?}=({})",
                    i + 1,
                    round.hand,
                    round.bid,
                    round.hand.hand_type(),
                    round.hand.hand_type().rank(),
                );
                (i + 1) * round.bid
            })
            .sum()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let rounds = parse_input(&aoc::example::example_string("day7.txt"));

            assert_eq!(calculate(&rounds), 6440);
        }
    }
}

fn parse_input(s: &str) -> Rounds {
    s.parse().unwrap()
}

fn main() {
    let cli = aoc::cli::parse();

    let rounds: Rounds = parse_input(&cli.input_string());

    println!("Part 1: {}", part1::calculate(&rounds));
}

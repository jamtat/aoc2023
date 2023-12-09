use std::{cmp, collections::HashMap, fmt::Display, marker::PhantomData, ops::Deref, str::FromStr};

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

trait HandRank {
    fn hand_type(hand: &Hand) -> HandType;
    fn card_rank(card: u8) -> u8;
}

struct HandRanker<T: HandRank>(PhantomData<T>);

impl<T: HandRank> HandRanker<T> {
    pub fn new() -> Self {
        Self(Default::default())
    }

    fn rank_array(hand: &Hand) -> [u8; 5] {
        hand.0.map(|c| T::card_rank(c))
    }

    fn rank_pair(hand: &Hand) -> (u8, [u8; 5]) {
        (T::hand_type(hand).rank(), Self::rank_array(hand))
    }

    fn cmp(a: &Hand, b: &Hand) -> cmp::Ordering {
        Self::rank_pair(a).cmp(&Self::rank_pair(b))
    }

    pub fn rank<'a>(rounds: &'a Rounds) -> Vec<(usize, &'a Round)> {
        let mut ranked: Vec<&Round> = rounds.0.iter().collect();

        ranked.sort_by(|a, b| Self::cmp(&a.hand, &b.hand));

        ranked
            .into_iter()
            .enumerate()
            .map(|(i, round)| {
                #[cfg(test)]
                println!(
                    "{} -> {} {}, {:?}=({})",
                    i + 1,
                    round.hand,
                    round.bid,
                    T::hand_type(&round.hand),
                    T::hand_type(&round.hand).rank(),
                );
                (i + 1, round)
            })
            .collect()
    }
}

mod part1 {
    use super::*;

    struct HandRankPart1;

    impl HandRank for HandRankPart1 {
        fn hand_type(hand: &Hand) -> HandType {
            let counts = hand.0.iter().fold(HashMap::new(), |mut freq, c| {
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
    }

    pub fn calculate(rounds: &Rounds) -> usize {
        HandRanker::<HandRankPart1>::rank(rounds)
            .iter()
            .map(|(i, round)| i * round.bid)
            .sum()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_card_rank() {
            assert_eq!(HandRankPart1::card_rank(b'2'), 2);
            assert_eq!(HandRankPart1::card_rank(b'9'), 9);
        }

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

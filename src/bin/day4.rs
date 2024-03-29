use aoc2023::aoc;

#[derive(Debug)]
struct Input {
    cards: Vec<Card>,
}

impl Input {
    pub fn get_card(&self, id: usize) -> Option<&Card> {
        self.cards.get(id - 1)
    }

    pub fn cards_won_from_card(&self, id: usize) -> impl Iterator<Item = &Card> {
        let card = self.get_card(id).unwrap();
        let count = card.win_count();

        (id + 1..(id + count + 1)).filter_map(|id| self.get_card(id))
    }
}

#[derive(Debug)]
struct Card {
    id: usize,
    winners: Vec<usize>,
    have: Vec<usize>,
}

impl Card {
    pub fn win_count(&self) -> usize {
        self.have
            .iter()
            .filter(|have| self.winners.contains(have))
            .count()
    }

    pub fn points(&self) -> usize {
        let win_count = self.win_count();
        if win_count > 0 {
            2usize.pow((win_count - 1) as u32)
        } else {
            0
        }
    }
}

fn parse_input(lines: impl Iterator<Item = String>) -> Input {
    let cards = lines.map(|line| parse_line(&line)).collect();
    Input { cards }
}

fn parse_line(line: &str) -> Card {
    let (id, rest) = line.split_once(": ").unwrap();
    let id = id
        .split_ascii_whitespace()
        .nth(1)
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let (winners, have) = rest.split_once(" | ").unwrap();

    Card {
        id,
        winners: winners
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect(),
        have: have
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect(),
    }
}

mod part1 {
    use super::*;

    pub fn calculate(input: &Input) -> usize {
        input.cards.iter().map(|card| card.points()).sum()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = parse_input(aoc::example::example_lines("day4.txt"));

            assert_eq!(calculate(&input), 13);
        }
    }
}

mod part2 {
    use std::collections::HashMap;

    use super::*;

    pub fn calculate(input: &Input) -> usize {
        let mut card_count: HashMap<usize, usize> =
            input.cards.iter().map(|card| (card.id, 1)).collect();

        for card in &input.cards {
            let copies = *card_count.get(&card.id).unwrap();

            for won_card in input.cards_won_from_card(card.id) {
                *card_count.get_mut(&won_card.id).unwrap() += copies;
            }
        }

        card_count.values().sum()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = parse_input(aoc::example::example_lines("day4.txt"));

            assert_eq!(calculate(&input), 30);
        }
    }
}

fn main() {
    let cli = aoc::cli::parse();

    let input = parse_input(cli.line_reader());

    println!("Part 1: {}", part1::calculate(&input));
    println!("Part 2: {}", part2::calculate(&input));
}

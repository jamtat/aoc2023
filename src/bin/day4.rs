use aoc2023::aoc;

#[derive(Debug)]
struct Input {
    cards: Vec<Card>,
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
fn main() {
    let cli = aoc::cli::parse();

    let input = parse_input(cli.line_reader());

    println!("Part 1: {}", part1::calculate(&input));
}

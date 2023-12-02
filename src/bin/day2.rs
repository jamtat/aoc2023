use std::collections::HashMap;

use aoc2023::aoc;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Colour {
    Red,
    Blue,
    Green,
}

#[derive(Debug)]
pub struct Game {
    id: usize,
    max: HashMap<Colour, usize>,
}

fn parse_game(s: &str) -> Game {
    let (id, s) = s.split_once(": ").expect("Has a colon");
    let id = *(&id[5..].parse::<usize>().expect("Id is a number"));

    let mut max = HashMap::new();
    let re = Regex::new(r"(\d+) ([a-z]+)").unwrap();
    for (_, [count, colour]) in re.captures_iter(s).map(|c| c.extract()) {
        let colour = parse_colour(colour).expect("valid colour");
        let count: usize = count.parse().expect("valid cound");
        let current_max = *max.entry(colour).or_insert(0);
        if count > current_max {
            max.insert(colour, count);
        }
    }

    Game { id, max }
}

fn parse_colour(s: &str) -> Option<Colour> {
    match s {
        "red" => Some(Colour::Red),
        "green" => Some(Colour::Green),
        "blue" => Some(Colour::Blue),
        _ => None,
    }
}

mod part1 {
    use super::*;

    pub fn calculate(games: &Vec<Game>) -> usize {
        let limits: &[(Colour, usize)] =
            &[(Colour::Red, 12), (Colour::Green, 13), (Colour::Blue, 14)];

        games
            .iter()
            .filter(|game| {
                limits
                    .iter()
                    .all(|&(colour, limit)| *game.max.get(&colour).unwrap_or(&0) <= limit)
            })
            .map(|game| game.id)
            .sum()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let games = parse_input(aoc::example::example_lines("day2.txt"));
            assert_eq!(calculate(&games), 8);
        }
    }
}

fn parse_input(lines: impl Iterator<Item = String>) -> Vec<Game> {
    lines.map(|line| parse_game(&line)).collect()
}

fn main() {
    let cli = aoc::cli::parse();

    let games = parse_input(cli.line_reader());

    println!("Part 1: {}", part1::calculate(&games));
}

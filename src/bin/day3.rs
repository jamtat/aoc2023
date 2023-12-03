use std::collections::HashSet;

use aoc2023::aoc;

#[derive(Debug)]
struct Schematic {
    parts: Vec<Part>,
    symbols: Vec<Symbol>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl From<(isize, isize)> for Point {
    fn from((x, y): (isize, isize)) -> Self {
        Point { x, y }
    }
}

#[derive(Debug)]
struct Part {
    number: usize,
    points: HashSet<Point>,
}

#[derive(Debug)]
struct Symbol {
    char: char,
    point: Point,
}

fn parse_input(lines: impl Iterator<Item = String>) -> Schematic {
    let mut parts: Vec<Part> = vec![];
    let mut symbols: Vec<Symbol> = vec![];

    let mut current_number: Option<usize> = None;
    let mut current_points: HashSet<Point> = HashSet::new();

    for (y, line) in lines.enumerate() {
        let y = y as isize;
        for (x, c) in line.chars().enumerate() {
            let x = x as isize;

            if c.is_ascii_digit() {
                let n: usize = (c as u8 - b'0').into();
                current_number = Some(current_number.unwrap_or(0) * 10 + n);

                for x in (x - 1)..=(x + 1) {
                    for y in (y - 1)..=(y + 1) {
                        current_points.insert((x, y).into());
                    }
                }

                continue;
            }

            if let Some(n) = current_number {
                // Need to add current point
                parts.push(Part {
                    number: n,
                    points: current_points.clone(),
                });
                // Reset the accumulator
                current_number = None;
                current_points = Default::default();
            }

            if c != '.' {
                symbols.push(Symbol {
                    char: c,
                    point: (x, y).into(),
                });
            }
        }
    }

    Schematic { parts, symbols }
}

mod part1 {
    use super::*;

    pub fn calculate(schematic: &Schematic) -> usize {
        let symbol_points = schematic
            .symbols
            .iter()
            .map(|s| s.point)
            .collect::<HashSet<_>>();

        schematic
            .parts
            .iter()
            .filter(|part| {
                part.points
                    .iter()
                    .any(|point| symbol_points.contains(point))
            })
            .map(|part| part.number)
            .sum()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let schematic = parse_input(aoc::example::example_lines("day3.txt"));

            assert_eq!(calculate(&schematic), 4361);
        }
    }
}

fn main() {
    let cli = aoc::cli::parse();

    let schematic = parse_input(cli.line_reader());

    println!("Part 1: {}", part1::calculate(&schematic))
}

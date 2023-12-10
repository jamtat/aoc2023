use std::{
    collections::HashMap,
    fmt::{Display, Write},
    str::from_utf8,
};

use aoc2023::aoc;

type Node = [u8; 3];

trait StartEnd {
    fn is_start(&self) -> bool;
    fn is_end(&self) -> bool;
}

impl StartEnd for Node {
    fn is_start(&self) -> bool {
        self[2] == b'A'
    }

    fn is_end(&self) -> bool {
        self[2] == b'Z'
    }
}

#[derive(Debug)]
struct Input {
    steps: Vec<Step>,
    network: HashMap<Node, (Node, Node)>,
}

impl Input {
    pub fn iter_steps(&self) -> StepsIter {
        StepsIter::new(&self.steps)
    }

    pub fn next(&self, start: &Node, step: Step) -> &Node {
        let node = self.network.get(start).unwrap();
        match step {
            Step::L => &node.0,
            Step::R => &node.1,
        }
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for step in &self.steps {
            write!(f, "{:?}", step)?;
        }
        f.write_str("\n\n")?;

        for (k, (l, r)) in &self.network {
            writeln!(
                f,
                "{} => ({}, {})",
                from_utf8(k).unwrap(),
                from_utf8(l).unwrap(),
                from_utf8(r).unwrap()
            )?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
enum Step {
    L,
    R,
}

impl Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Step::L => 'L',
            Step::R => 'R',
        })
    }
}

struct StepsIter<'a> {
    steps: &'a Vec<Step>,
    idx: usize,
}

impl<'a> StepsIter<'a> {
    pub fn new(steps: &'a Vec<Step>) -> Self {
        Self { steps, idx: 0 }
    }
}

impl<'a> Iterator for StepsIter<'a> {
    type Item = Step;

    fn next(&mut self) -> Option<Self::Item> {
        let step = self.steps[self.idx];
        self.idx = (self.idx + 1) % self.steps.len();
        Some(step)
    }
}

mod part1 {
    use super::*;

    pub fn calculate(input: &Input) -> usize {
        let mut node: &Node = b"AAA";

        for (i, step) in input.iter_steps().enumerate() {
            if node == b"ZZZ" {
                return i;
            }
            node = input.next(node, step);
        }

        unreachable!()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            assert_eq!(
                calculate(&parse_input(aoc::example::example_lines("day8_1.txt"))),
                2
            );
            assert_eq!(
                calculate(&parse_input(aoc::example::example_lines("day8_2.txt"))),
                6
            );
        }
    }
}

mod part2 {
    use super::*;
    use aoc2023::quant::QuantIter;
    use itertools::Itertools;

    pub fn calculate(input: &Input) -> usize {
        let starts: Vec<_> = input
            .network
            .keys()
            .cloned()
            .filter(Node::is_start)
            .collect();

        starts
            .into_iter()
            .map(|start| {
                let mut node = &start;
                let mut to_end = 0;
                let mut step_iter = input.iter_steps();
                let _end = loop {
                    let step = step_iter.next().unwrap();
                    if node.is_end() {
                        break *node;
                    }
                    node = input.next(&node, step);
                    to_end += 1;
                };

                #[cfg(test)]
                println!(
                    "{} -> {} to_end={}",
                    from_utf8(&start).unwrap(),
                    from_utf8(&_end).unwrap(),
                    to_end
                );

                to_end
            })
            .lcm()
            .unwrap()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            assert_eq!(
                calculate(&parse_input(aoc::example::example_lines("day8_3.txt"))),
                6
            );
        }
    }
}

fn parse_input(mut lines: impl Iterator<Item = String>) -> Input {
    let steps = lines.next().expect("Has steps");
    let steps = steps
        .chars()
        .into_iter()
        .filter_map(|step| match step {
            'L' => Some(Step::L),
            'R' => Some(Step::R),
            _ => None,
        })
        .collect();

    drop(lines.next());

    let network = lines
        .map(|line| {
            let line = line.as_bytes();

            (
                line[0..3].try_into().unwrap(),
                (
                    line[7..10].try_into().unwrap(),
                    line[12..15].try_into().unwrap(),
                ),
            )
        })
        .collect();

    Input { steps, network }
}

fn main() {
    let cli = aoc::cli::parse();

    let input = parse_input(cli.line_reader());

    println!("Part 1: {}", part1::calculate(&input));
    println!("Part 2: {}", part2::calculate(&input));
}

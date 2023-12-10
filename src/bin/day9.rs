use aoc2023::aoc;

type Sequence = Vec<isize>;

fn parse_input(lines: impl Iterator<Item = String>) -> Vec<Sequence> {
    lines
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect()
}

mod part1 {
    use itertools::Itertools;

    use super::*;

    pub fn calculate(sequences: &Vec<Sequence>) -> isize {
        sequences.iter().map(extrapolate).sum()
    }

    fn extrapolate(seq: &Sequence) -> isize {
        if is_all_zero(seq) {
            0
        } else {
            seq.last().unwrap() + extrapolate(&diff_seq(seq))
        }
    }

    fn diff_seq(seq: &Sequence) -> Sequence {
        seq.iter().tuple_windows().map(|(a, b)| b - a).collect()
    }

    fn is_all_zero(seq: &Sequence) -> bool {
        seq.iter().all(|x| *x == 0)
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let sequences = parse_input(aoc::example::example_lines("day9.txt"));

            assert_eq!(calculate(&sequences), 114)
        }
    }
}

fn main() {
    let cli = aoc::cli::parse();

    let sequences = parse_input(cli.line_reader());

    println!("Part 1: {}", part1::calculate(&sequences))
}

use aoc2023::aoc;

mod part1 {
    use super::*;

    fn parse_line(line: &str) -> usize {
        let mut digits = line.bytes().filter(|&b| b'0' <= b && b <= b'9');

        let first = digits.next().expect("Has one digit");
        let last = digits.last().unwrap_or(first);

        format!("{}{}", first as char, last as char)
            .parse()
            .unwrap()
    }

    pub fn calculate(input: &str) -> usize {
        input.lines().map(|line| parse_line(&line)).sum()
    }

    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day1_1.txt");

            assert_eq!(calculate(&input), 142);
        }
    }
}

fn main() {
    let cli = aoc::cli::parse();

    let input = cli.input_string();

    println!("Part 1: {}", part1::calculate(&input));
}

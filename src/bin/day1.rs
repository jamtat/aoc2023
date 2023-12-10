use aoc2023::aoc;

mod part1 {

    fn parse_line(line: &str) -> usize {
        let mut digits = line
            .bytes()
            .filter(|&b| b'0' < b && b <= b'9')
            .map(|b| (b - b'0') as usize);

        let first = digits.next().expect("Has one digit");
        let last = digits.last().unwrap_or(first);

        first * 10 + last
    }

    pub fn calculate(input: &str) -> usize {
        input.lines().map(|line| parse_line(line)).sum()
    }

    #[cfg(test)]
    mod test {
        use super::calculate;
        use aoc2023::aoc;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day1_1.txt");

            assert_eq!(calculate(&input), 142);
        }
    }
}

mod part2 {
    static NUMBERS: phf::Map<&'static [u8], usize> = phf::phf_map!(
        b"one" => 1,
        b"two" => 2,
        b"three" => 3,
        b"four" => 4,
        b"five" => 5,
        b"six" => 6,
        b"seven" => 7,
        b"eight" => 8,
        b"nine" => 9,
        b"1" => 1,
        b"2" => 2,
        b"3" => 3,
        b"4" => 4,
        b"5" => 5,
        b"6" => 6,
        b"7" => 7,
        b"8" => 8,
        b"9" => 9,
    );

    fn parse_line(line: &str) -> usize {
        let s = line.as_bytes();

        let numbers = (0..s.len())
            .filter_map(|i| {
                let s = &s[i..s.len()];
                for (word, n) in &NUMBERS {
                    if s.starts_with(word) {
                        return Some(*n);
                    }
                }
                None
            })
            .collect::<Vec<_>>();

        numbers.first().unwrap() * 10 + numbers.last().unwrap()
    }

    pub fn calculate(input: &str) -> usize {
        input
            .lines()
            .map(|line| {
                let number = parse_line(&line);

                #[cfg(test)]
                eprintln!("{} {}", line, number);

                number
            })
            .sum()
    }

    #[cfg(test)]
    mod test {
        use super::calculate;
        use aoc2023::aoc;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day1_2.txt");

            assert_eq!(calculate(&input), 281);
        }
    }
}

fn main() {
    let cli = aoc::cli::parse();

    let input = cli.input_string();

    println!("Part 1: {}", part1::calculate(&input));
    println!("Part 2: {}", part2::calculate(&input));
}

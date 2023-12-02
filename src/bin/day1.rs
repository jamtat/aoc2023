use aoc2023::aoc;

mod part1 {

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

    fn parse_digit(s: &[u8]) -> Option<usize> {
        match s {
            b"0" => Some(0),
            b"1" | b"one" => Some(1),
            b"2" | b"two" => Some(2),
            b"3" | b"three" => Some(3),
            b"4" | b"four" => Some(4),
            b"5" | b"five" => Some(5),
            b"6" | b"six" => Some(6),
            b"7" | b"seven" => Some(7),
            b"8" | b"eight" => Some(8),
            b"9" | b"nine" => Some(9),
            _ => None,
        }
    }

    fn parse_next_digit(s: &[u8]) -> (&[u8], Option<usize>) {
        let mut s = s;
        while !s.is_empty() {
            // Check if any of the next 5 bytes are a parse-able digit
            for i in 1..=5 {
                if s.len() < i {
                    break;
                }
                if let Some(digit) = parse_digit(&s[0..i]) {
                    // Don't consume the entire length of the parsed bytes, only shift forward
                    // by 1 position otherwise this doesn't produce the right answer.
                    // The provided example didn't make this clear.
                    return (&s[1..], Some(digit));
                }
            }
            s = &s[1..];
        }
        return (s, None);
    }

    fn parse_line(line: &str) -> usize {
        let mut last: Option<usize> = None;
        let s = line.as_bytes();
        let (mut s, first) = parse_next_digit(s);
        let first = first.unwrap();

        let last = loop {
            let (rest, digit) = parse_next_digit(s);
            s = rest;
            if digit.is_none() {
                break last.unwrap_or(first);
            }
            last = digit;
        };

        format!("{first}{last}").parse().unwrap()
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

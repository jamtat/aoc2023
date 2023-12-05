use aoc2023::aoc;
use nom;

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

impl Almanac {
    pub fn convert(&self, seed: usize) -> usize {
        self.maps
            .iter()
            .fold(seed, |seed, ranges| ranges.try_convert(seed))
    }

    pub fn locations(&self) -> Vec<usize> {
        self.seeds.iter().map(|seed| self.convert(*seed)).collect()
    }
}

#[derive(Debug)]
struct Range {
    src: usize,
    dst: usize,
    length: usize,
}

impl Range {
    pub fn new(src: usize, dst: usize, length: usize) -> Self {
        Self { src, dst, length }
    }
    pub fn convert(&self, src: usize) -> Option<usize> {
        (src >= self.src && src < self.src + self.length).then(|| self.dst + (src - self.src))
    }
}

#[derive(Debug)]
struct Map {
    name: String,
    ranges: Vec<Range>,
}

impl Map {
    pub fn new(name: String, ranges: Vec<Range>) -> Self {
        Self { name, ranges }
    }
    pub fn convert(&self, src: usize) -> Option<usize> {
        for range in &self.ranges {
            if let Some(dst) = range.convert(src) {
                return Some(dst);
            }
        }
        None
    }

    pub fn try_convert(&self, src: usize) -> usize {
        self.convert(src).unwrap_or(src)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_range_convert() {
        let range = Range::new(98, 50, 2);
        assert_eq!(range.convert(97), None);
        assert_eq!(range.convert(98), Some(50));
        assert_eq!(range.convert(99), Some(51));
        assert_eq!(range.convert(100), None);
    }
}

mod part1 {
    use super::*;

    pub fn calculate(almanac: &Almanac) -> usize {
        *almanac.locations().iter().min().expect("has min location")
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let almanac = input::parse(&aoc::example::example_string("day5.txt"));

            assert_eq!(calculate(&almanac), 35);
        }
    }
}

mod input {
    use aoc::parse::*;

    use super::*;

    pub fn parse(s: &str) -> Almanac {
        let (s, seeds) = nom::sequence::preceded(
            nom::bytes::complete::tag("seeds: "),
            nom::multi::separated_list1(nom::character::complete::space1, parse_number::<usize>),
        )(s)
        .unwrap();

        let (_, maps) = nom::multi::many1(parse_map)(s).unwrap();

        Almanac { seeds, maps }
    }

    fn parse_range(s: &str) -> nom::IResult<&str, Range> {
        let (s, (dst, _, src, _, length)) = nom::sequence::tuple((
            parse_number::<usize>,
            nom::character::complete::space1,
            parse_number::<usize>,
            nom::character::complete::space1,
            parse_number::<usize>,
        ))(s)?;

        Ok((s, Range::new(src, dst, length)))
    }

    fn parse_map(s: &str) -> nom::IResult<&str, Map> {
        const MAP_HEADER_END: &'static str = " map:\n";
        let (s, name) = nom::bytes::complete::take_until1(MAP_HEADER_END)(s)?;
        let name = name.trim();
        let (s, ranges) = nom::sequence::preceded(
            nom::bytes::complete::tag(MAP_HEADER_END),
            nom::multi::separated_list1(nom::character::complete::char('\n'), parse_range),
        )(s)?;

        Ok((s, Map::new(name.to_owned(), ranges)))
    }
}

fn main() {
    let cli = aoc::cli::parse();

    let almanac = input::parse(&cli.input_string());

    println!("Part 1: {}", part1::calculate(&almanac));
}

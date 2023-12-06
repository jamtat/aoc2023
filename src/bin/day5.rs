use aoc2023::{aoc, arr_chunks};
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
            .fold(seed, |seed, map| map.try_convert(seed))
    }

    pub fn convert_seed_range(&self, sr: &SourceRange) -> Vec<SourceRange> {
        self.maps.iter().fold(vec![sr.clone()], |srs, map| {
            srs.into_iter()
                .flat_map(|sr| map.convert_source_range(&sr))
                .collect()
        })
    }

    pub fn locations(&self) -> Vec<usize> {
        self.seeds.iter().map(|seed| self.convert(*seed)).collect()
    }

    pub fn seed_ranges(&self) -> Vec<SourceRange> {
        arr_chunks(self.seeds.iter().cloned())
            .map(|[start, length]| SourceRange::new(start, length))
            .collect()
    }

    pub fn locations_seed_ranges(&self) -> Vec<SourceRange> {
        self.seed_ranges()
            .iter()
            .flat_map(|sr| self.convert_seed_range(sr))
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct SourceRange {
    start: usize,
    length: usize,
}

impl SourceRange {
    pub fn new(start: usize, length: usize) -> Self {
        Self { start, length }
    }

    pub fn from_start_end(start: usize, end: usize) -> Self {
        Self::new(start, end - start + 1)
    }

    pub fn end(&self) -> usize {
        self.start + self.length - 1
    }
}

#[derive(Debug)]
struct Range {
    src: usize,
    dst: usize,
    length: usize,
}

impl Range {
    fn src_end(&self) -> usize {
        self.src + self.length - 1
    }
    pub fn new(src: usize, dst: usize, length: usize) -> Self {
        Self { src, dst, length }
    }
    pub fn convert(&self, src: usize) -> Option<usize> {
        (src >= self.src && src < self.src + self.length).then(|| self.dst + (src - self.src))
    }
    pub fn convert_source_range(
        &self,
        sr: &SourceRange,
    ) -> (Vec<SourceRange>, Option<SourceRange>) {
        if sr.start > self.src_end() || sr.end() < self.src {
            // No overlap
            (vec![sr.clone()], None)
        } else {
            let mut out_unchanged = vec![];
            // Portion before overlap
            if sr.start < self.src {
                out_unchanged.push(SourceRange::from_start_end(sr.start, self.src - 1))
            }

            // Portion after overlap

            if sr.end() > self.src_end() {
                out_unchanged.push(SourceRange::from_start_end(self.src_end() + 1, sr.end()))
            }

            (
                out_unchanged,
                Some(SourceRange::from_start_end(
                    self.convert(sr.start.max(self.src)).unwrap(),
                    self.convert(sr.end().min(self.src_end())).unwrap(),
                )),
            )
        }
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

    pub fn convert_source_range(&self, sr: &SourceRange) -> Vec<SourceRange> {
        let mut converted = vec![];
        let mut unchanged = vec![sr.clone()];

        for range in &self.ranges {
            let queue = unchanged.clone();
            unchanged.clear();

            for sr in queue.into_iter() {
                let (sr_unchanged, sr_converted) = range.convert_source_range(&sr);
                if let Some(sr_converted) = sr_converted {
                    converted.push(sr_converted);
                }
                unchanged.extend(sr_unchanged.into_iter());
            }
        }

        converted.extend(unchanged.into_iter());

        converted
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

mod part1 {
    use super::*;

    pub fn calculate(almanac: &Almanac) -> usize {
        almanac
            .locations()
            .iter()
            .cloned()
            .min()
            .expect("has min location")
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

mod part2 {
    use super::*;

    pub fn calculate(almanac: &Almanac) -> usize {
        almanac
            .locations_seed_ranges()
            .iter()
            .map(|sr| sr.start)
            .min()
            .expect("has min location")
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let almanac = input::parse(&aoc::example::example_string("day5.txt"));

            assert_eq!(calculate(&almanac), 46);
        }
    }
}

fn main() {
    let cli = aoc::cli::parse();

    let almanac = input::parse(&cli.input_string());

    println!("Part 1: {}", part1::calculate(&almanac));
    println!("Part 2: {}", part2::calculate(&almanac));
}

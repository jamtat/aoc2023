use aoc2023::aoc;

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    pub fn calculate_distance(&self, held_time: usize) -> usize {
        if held_time >= self.time {
            0
        } else {
            (self.time - held_time) * held_time
        }
    }

    pub fn winner_range(&self) -> (usize, usize) {
        // f64 seems critical for precision here.
        // It's highly likely there's some better way of actually
        // implementing the quadratic formula that means you don't lose all this wonderful precision
        let b = -(self.time as f64);
        let c = -(self.distance as f64);

        let rhs = (b * b + 4.0 * c).sqrt();
        let x1 = (b - rhs) / -2.0;
        let x2 = (b + rhs) / -2.0;
        let (x1, x2) = (x1.min(x2), x1.max(x2));

        #[cfg(test)]
        println!("x1={x1}, x2={x2}");
        // Have to round up and down because it's an inequality.
        // Could maybe add an epsilon to distance instead?
        ((x1 + 1.0).floor() as usize, (x2 - 1.0).ceil() as usize)
    }

    pub fn record_count(&self) -> usize {
        let (start, end) = self.winner_range();

        #[cfg(test)]
        println!("{start}->{end}, {}", (end - start) + 1);
        (end - start) + 1
    }
}

struct Races(Vec<Race>);

impl Races {
    pub fn smushed_time(&self) -> usize {
        let time = self
            .0
            .iter()
            .map(|race| race.time.to_string())
            .collect::<String>();

        time.parse().unwrap()
    }

    pub fn smushed_distance(&self) -> usize {
        let distance = self
            .0
            .iter()
            .map(|race| race.distance.to_string())
            .collect::<String>();

        distance.parse().unwrap()
    }

    pub fn smushed_race(&self) -> Race {
        Race {
            time: self.smushed_time(),
            distance: self.smushed_distance(),
        }
    }
}

fn parse_input(s: &str) -> Races {
    let mut lines = s.lines();
    let mut times = lines.next().unwrap().split_ascii_whitespace();
    let _ = times.next();
    let times: Vec<usize> = times.map(|time| time.parse().unwrap()).collect();

    let mut dists = lines.next().unwrap().split_ascii_whitespace();
    let _ = dists.next();
    let dists: Vec<usize> = dists.map(|dist| dist.parse().unwrap()).collect();

    Races(
        times
            .iter()
            .zip(dists.iter())
            .map(|(&time, &distance)| Race { time, distance })
            .collect(),
    )
}

mod part1 {
    use super::*;

    pub fn calculate(races: &Races) -> usize {
        races
            .0
            .iter()
            .fold(1, |acc, race| acc * race.record_count())
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let races = parse_input(&aoc::example::example_string("day6.txt"));

            assert_eq!(calculate(&races), 288);
        }
    }
}

mod part2 {
    use super::*;

    pub fn calculate(races: &Races) -> usize {
        races.smushed_race().record_count()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let races = parse_input(&aoc::example::example_string("day6.txt"));

            assert_eq!(calculate(&races), 71503);
        }
    }
}

fn main() {
    let cli = aoc::cli::parse();

    let races = parse_input(&cli.input_string());

    println!("Part 1: {}", part1::calculate(&races));
    println!("Part 2: {}", part2::calculate(&races));
}

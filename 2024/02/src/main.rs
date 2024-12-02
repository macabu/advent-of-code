fn main() {
    let input = include_str!("input.txt");

    dbg!(part_one(input));
    dbg!(part_two(input));
}

enum Direction {
    Decreasing,
    Increasing,
}

struct Report(Vec<i16>);

impl Report {
    fn new(line: &[i16]) -> Self {
        Self(line.to_owned())
    }

    fn is_safe(&self) -> bool {
        let mut direction: Option<Direction> = None;

        for levels in self.0.windows(2) {
            assert!(levels.len() == 2);

            match levels[0] - levels[1] {
                1..=3 => match direction {
                    None => {
                        direction = Some(Direction::Decreasing);
                        continue;
                    }
                    Some(Direction::Decreasing) => {
                        continue;
                    }
                    Some(Direction::Increasing) => {
                        return false;
                    }
                },
                -3..=-1 => match direction {
                    None => {
                        direction = Some(Direction::Increasing);
                        continue;
                    }
                    Some(Direction::Increasing) => {
                        continue;
                    }
                    Some(Direction::Decreasing) => {
                        return false;
                    }
                },
                _ => return false,
            }
        }

        true
    }

    fn is_safe_with_tolerance(&self) -> bool {
        if self.is_safe() {
            return true;
        }

        let mut permutations: Vec<_> = vec![];

        for i in 0..self.0.len() {
            permutations.push(
                self.0
                    .clone()
                    .into_iter()
                    .enumerate()
                    .filter(|(idx, _)| idx != &i)
                    .map(|(_, level)| level)
                    .collect::<Vec<_>>(),
            );
        }

        let safes = permutations
            .into_iter()
            .filter(|report| Report::new(report).is_safe())
            .count();

        safes > 0
    }
}

impl From<&str> for Report {
    fn from(line: &str) -> Self {
        Report::new(
            &line
                .split_whitespace()
                .map(|level| level.parse().unwrap())
                .collect::<Vec<_>>(),
        )
    }
}

fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(Report::from)
        .filter(|report| report.is_safe())
        .count() as u32
}

fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(Report::from)
        .filter(|report| report.is_safe_with_tolerance())
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        assert_eq!(2, part_one(test_input));
    }

    #[test]
    fn test_part2() {
        let test_input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        assert_eq!(4, part_two(test_input));
    }
}

use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");

    dbg!(part_one(input));
    dbg!(part_two(input));
}

fn part_one(input: &str) -> u32 {
    let map: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut antennas: HashMap<char, Vec<(usize, usize)>> = std::collections::HashMap::new();

    // Locate all antenas and their locations.
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let frequency = map[y][x];

            if frequency == '.' {
                continue;
            }

            antennas
                .entry(frequency)
                .and_modify(|locations| locations.push((y, x)))
                .or_insert(vec![(y, x)]);
        }
    }

    let mut unique_antinodes = HashSet::new();

    // For each type of frequency (character), calculate antinodes.
    for locations in antennas.values() {
        for i in 0..locations.len() {
            let (y, x) = locations[i];

            // Look into the other locations and then calculate the antinodes.
            for j in i..locations.len() - 1 {
                let (other_y, other_x) = locations[j + 1];

                let (diff_y, diff_x) = (
                    (y as isize - other_y as isize),
                    (x as isize - other_x as isize),
                );

                let (new_y, new_x) = ((y as isize + diff_y), (x as isize + diff_x));
                let (new_other_y, new_other_x) =
                    ((other_y as isize - diff_y), (other_x as isize - diff_x));

                unique_antinodes.insert((new_y, new_x));
                unique_antinodes.insert((new_other_y, new_other_x));
            }
        }
    }

    let limit = (map.len() - 1) as isize;

    // Filter invalid ones.
    unique_antinodes
        .into_iter()
        .filter(|(y, x)| x >= &0 && x <= &limit && y >= &0 && y <= &limit)
        .count() as u32
}

fn part_two(_input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part1() {
        assert_eq!(14, part_one(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(34, part_two(TEST_INPUT));
    }
}

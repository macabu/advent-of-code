use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");

    dbg!(part_one(input));
    dbg!(part_two(input));
}

fn point_is_adjacent(origin: &(usize, usize), other: &(usize, usize)) -> bool {
    // Left of origin.
    if origin.0 + 1 == other.0 && origin.1 == other.1 {
        return true;
    }

    // Right of origin.
    if origin.0 == other.0 + 1 && origin.1 == other.1 {
        return true;
    }

    // Above origin.
    if origin.0 == other.0 && origin.1 + 1 == other.1 {
        return true;
    }

    // Below origin.
    if origin.0 == other.0 && origin.1 == other.1 + 1 {
        return true;
    }

    return false;
}

fn part_one(input: &str) -> u64 {
    // Initiate map.
    let topographic_map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // Find and group all positions by their height.
    let mut points_by_height: HashMap<u32, Vec<(usize, usize)>> = HashMap::new();
    for y in 0..topographic_map.len() {
        for x in 0..topographic_map[y].len() {
            if topographic_map[y][x] == '.' {
                continue;
            }

            points_by_height
                .entry(topographic_map[y][x].to_digit(10).unwrap())
                .and_modify(|points| points.push((y, x)))
                .or_insert(vec![(y, x)]);
        }
    }

    let mut valid_trails: HashMap<&(usize, usize), HashSet<&(usize, usize)>> = HashMap::new();

    // Starting from 0, walk the trails.
    let starting_points = points_by_height.get(&0).unwrap();
    for starting_point in starting_points {
        let mut points_to_check = vec![starting_point];

        // Apply BFS to check adjacent points and calculate valid trails endings.
        while let Some(current_point) = points_to_check.pop() {
            let current_level = topographic_map[current_point.0][current_point.1]
                .to_digit(10)
                .unwrap();

            // The trail is completed and the ending point can be added.
            if current_level == 9 {
                valid_trails
                    .entry(starting_point)
                    .or_insert_with(HashSet::new)
                    .insert(current_point);

                continue;
            }

            if let Some(next_points) = points_by_height.get(&(current_level + 1)) {
                for next_point in next_points {
                    if point_is_adjacent(current_point, next_point) {
                        points_to_check.push(next_point);
                    }
                }
            }
        }
    }

    valid_trails
        .values()
        .fold(0, |acc, valid| acc + valid.len() as u64)
}

fn part_two(_input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part1() {
        assert_eq!(36, part_one(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(81, part_two(TEST_INPUT));
    }
}

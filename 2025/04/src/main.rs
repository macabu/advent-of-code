use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    dbg!(part_one(input));
    dbg!(part_two(input));
}

fn part_one(input: &str) -> u64 {
    run(input, false)
}

fn part_two(input: &str) -> u64 {
    run(input, true)
}

fn run(input: &str, remove_cells: bool) -> u64 {
    let mut hm = HashMap::new();

    let mut lines_count = 0;
    input.lines().enumerate().for_each(|(y, line)| {
        lines_count += 1;
        line.chars().enumerate().for_each(|(x, cell)| {
            hm.insert((x as i32, y as i32), cell);
        })
    });

    let mut accessible = 0;

    for _ in 0..=50 {
        for y in 0..lines_count {
            for x in 0..lines_count {
                let cell = hm.get(&(x as i32, y as i32));
                if cell.is_none() {
                    continue;
                }

                let cell = *cell.unwrap();
                if cell == '.' {
                    continue;
                }

                let directions = [
                    (0, -1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                    (0, 1),
                    (-1, 1),
                    (-1, 0),
                    (-1, -1),
                ];

                let mut rolls_of_paper = 0;
                for (dx, dy) in directions.iter() {
                    match hm.get(&(x as i32 + dx, y as i32 + dy)) {
                        Some(&'@') => rolls_of_paper += 1,
                        _ => continue,
                    }
                }

                if rolls_of_paper < 4 {
                    accessible += 1;

                    if remove_cells {
                        if let Some(v) = hm.get_mut(&(x as i32, y as i32)) {
                            *v = '.';
                        };
                    }
                }
            }
        }

        if !remove_cells {
            break;
        }
    }

    accessible
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        assert_eq!(13, part_one(test_input));
    }

    #[test]
    fn test_part2() {
        let test_input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        assert_eq!(43, part_two(test_input));
    }
}

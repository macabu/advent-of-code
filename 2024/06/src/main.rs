use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");

    dbg!(part_one(input));
    dbg!(part_two(input));
}

const FREE: char = '.';
const GUARD: char = '^';
const OBSTACLE: char = '#';

const UP: (i32, i32) = (-1, 0);
const RIGHT: (i32, i32) = (0, 1);
const DOWN: (i32, i32) = (1, 0);
const LEFT: (i32, i32) = (0, -1);

// Move up->right->down->left then cycle back.
const DIRECTIONS: [(i32, i32); 4] = [UP, RIGHT, DOWN, LEFT];

fn part_one(input: &str) -> u32 {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // start by moving up.
    let mut direction_index = 0;

    // distinct positions visited.
    let mut positions_visited = HashSet::new();

    // y increases down.
    // x increases to the right.
    'outer: for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] != GUARD {
                continue;
            }

            // initial position also counts and we are not guaranteed to come back here.
            positions_visited.insert((y, x));

            let (mut dir_y, mut dir_x) = DIRECTIONS[direction_index];

            let mut cur_y = (y as i32 + dir_y) as usize;
            let mut cur_x = (x as i32 + dir_x) as usize;

            'inner: loop {
                match map.get(cur_y) {
                    Some(row) => match row.get(cur_x) {
                        Some(cell) => match cell {
                            &FREE | &GUARD => {
                                positions_visited.insert((cur_y, cur_x));

                                cur_y = (cur_y as i32 + dir_y) as usize;
                                cur_x = (cur_x as i32 + dir_x) as usize;

                                continue 'inner;
                            }
                            &OBSTACLE => {
                                // backtrack one cell.
                                cur_y = (cur_y as i32 - dir_y) as usize;
                                cur_x = (cur_x as i32 - dir_x) as usize;

                                // change direction.
                                direction_index += 1;
                                (dir_y, dir_x) = DIRECTIONS[direction_index % DIRECTIONS.len()];

                                continue 'inner;
                            }
                            &c => unreachable!("non mapped: {:?}", c),
                        },
                        None => {
                            break 'outer;
                        }
                    },
                    None => {
                        break 'outer;
                    }
                }
            }
        }
    }

    positions_visited.len() as u32
}

fn part_two(_input: &str) -> u32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part1() {
        assert_eq!(41, part_one(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(6, part_two(TEST_INPUT));
    }
}

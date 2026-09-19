fn main() {
    let input = include_str!("input.txt");

    dbg!(part_one(input));
    dbg!(part_two(input));
}

fn part_one(input: &str) -> i32 {
    input.chars().fold(0, |acc, dir| match dir {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => unreachable!(),
    })
}

fn part_two(input: &str) -> u32 {
    let mut basement_position = 0;
    let mut acc = 0;
    for dir in input.chars() {
        basement_position += 1;
        acc = if dir == '(' { acc + 1 } else { acc - 1 };
        if acc == -1 {
            break;
        }
    }
    basement_position
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = "";

        assert_eq!(0, part_one(test_input));
    }

    #[test]
    fn test_part2() {
        let test_input = "";

        assert_eq!(0, part_two(test_input));
    }
}

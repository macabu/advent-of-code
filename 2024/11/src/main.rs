fn main() {
    let input = include_str!("input.txt");

    dbg!(part_one(input));
    dbg!(part_two(input));
}

fn apply_rules(number: u128) -> Vec<u128> {
    match number {
        0 => vec![1],
        n => {
            let s = n.to_string();
            if s.len() % 2 == 0 {
                let (left, right) = s.split_at(s.len() / 2);

                vec![
                    left.parse::<u128>().unwrap(),
                    right.parse::<u128>().unwrap(),
                ]
            } else {
                vec![n * 2024]
            }
        }
    }
}

fn part_one(input: &str) -> u128 {
    let mut stones = input
        .split_ascii_whitespace()
        .map(|stone| stone.parse::<u128>().unwrap())
        .collect::<Vec<_>>();

    for _i in 0..25 {
        let mut x = vec![];
        for stone in stones.iter() {
            x.append(&mut apply_rules(*stone));
        }

        stones = x;
    }

    stones.len() as u128
}

fn part_two(input: &str) -> u128 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "125 17";

    #[test]
    fn test_part1() {
        assert_eq!(55312, part_one(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part_two(TEST_INPUT));
    }
}

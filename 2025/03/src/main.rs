fn main() {
    let input = include_str!("input.txt");

    dbg!(part_one(input));
    dbg!(part_two(input));
}

fn part_one(input: &str) -> u64 {
    use itertools::Itertools;

    input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .tuple_combinations::<(_, _)>()
                .map(|(a, b)| format!("{a}{b}").parse::<u64>().unwrap())
                .max()
                .unwrap()
        })
        .sum()
}

fn part_two(input: &str) -> u64 {
    use itertools::Itertools;

    input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .tuple_combinations::<(_, _, _, _, _, _, _, _, _, _, _, _)>()
                .map(|(a, b, c, d, e, f, g, h, i, j, k, l)| {
                    format!("{a}{b}{c}{d}{e}{f}{g}{h}{i}{j}{k}{l}")
                        .parse::<u64>()
                        .unwrap()
                })
                .max()
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = "987654321111111
        811111111111119
        234234234234278
        818181911112111";

        assert_eq!(357, part_one(test_input));

        let test_input = "123456789";

        assert_eq!(89, part_one(test_input));
    }

    #[test]
    fn test_part2() {
        let test_input = "987654321111111
811111111111119
234234234234278
818181911112111";

        assert_eq!(3121910778619, part_two(test_input));
    }
}

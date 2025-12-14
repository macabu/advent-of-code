fn main() {
    let input = include_str!("input.txt");

    dbg!(part_one(input));
    dbg!(part_two(input));
}

fn part_one(input: &str) -> u64 {
    let (fresh, available) = input.split_once("\n\n").unwrap();

    let mut fresh_ingredients = vec![];

    fresh.lines().for_each(|line| {
        let (start, end) = line.split_once('-').unwrap();
        let start: u64 = start.parse().unwrap();
        let end: u64 = end.parse().unwrap();

        fresh_ingredients.push((start, end));
    });

    fresh_ingredients.dedup();
    fresh_ingredients.sort();

    available
        .lines()
        .map(|line| {
            let ingredient: u64 = line.parse().unwrap();

            for (start, end) in fresh_ingredients.iter() {
                if ingredient >= *start && ingredient <= *end {
                    return 1;
                }
            }

            0
        })
        .sum::<u64>()
}

fn part_two(input: &str) -> u64 {
    let (fresh, _) = input.split_once("\n\n").unwrap();

    let mut fresh_ingredients = vec![];

    fresh.lines().for_each(|line| {
        let (start, end) = line.split_once('-').unwrap();
        let start: u64 = start.parse().unwrap();
        let end: u64 = end.parse().unwrap();

        fresh_ingredients.push((start, end));
    });

    fresh_ingredients.dedup();
    fresh_ingredients.sort();

    // find overlapping ranges and rebuild them
    // diff between end-start
    // sum

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

        assert_eq!(3, part_one(test_input));
    }

    #[test]
    fn test_part2() {
        let test_input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

        assert_eq!(14, part_two(test_input));
    }
}

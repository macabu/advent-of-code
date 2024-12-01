fn main() {
    let input = include_str!("input.txt");

    dbg!(part_one(input));
    dbg!(part_two(input));
}

fn part_one(input: &str) -> u32 {
    let mut list_left = Vec::with_capacity(input.len());
    let mut list_right = Vec::with_capacity(input.len());

    for line in input.lines() {
        let mut split = line.split_whitespace();

        list_left.push(split.next().unwrap().parse::<i32>().unwrap());
        list_right.push(split.next().unwrap().parse::<i32>().unwrap());
    }

    list_left.sort();
    list_right.sort();

    list_left
        .into_iter()
        .zip(list_right)
        .fold(0, |acc, (left, right)| acc + left.abs_diff(right))
}

fn part_two(input: &str) -> u32 {
    let mut list_left = Vec::with_capacity(input.len());
    let mut frequencies = std::collections::HashMap::with_capacity(input.len());

    for line in input.lines() {
        let mut split = line.split_whitespace();

        list_left.push(split.next().unwrap().parse::<i32>().unwrap());

        let line_right = split.next().unwrap().parse::<i32>().unwrap();
        let count = frequencies.get(&line_right).unwrap_or(&0);
        frequencies.insert(line_right, *count + 1);
    }

    list_left.iter().fold(0, |acc, line_left| {
        let frequency = frequencies.get(line_left).unwrap_or(&0);

        acc + (frequency * line_left) as u32
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = "3   4
4   3
2   5
1   3
3   9
3   3";

        assert_eq!(11, part_one(test_input));
    }

    #[test]
    fn test_part2() {
        let test_input = "3   4
4   3
2   5
1   3
3   9
3   3";

        assert_eq!(31, part_two(test_input));
    }
}

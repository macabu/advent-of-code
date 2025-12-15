fn main() {
    let input = include_str!("input.txt");

    dbg!(part_one(input));
    dbg!(part_two(input));
}

fn part_one(input: &str) -> u64 {
    let mut rev_it = input.lines().rev();

    let ops = rev_it
        .next()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>();

    let numbers = rev_it
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut grand_total = 0;

    for i in 0..ops.len() {
        let mut total = if ops[i] == "+" { 0 } else { 1 };

        for j in 0..numbers.len() {
            match ops[i] {
                "+" => total += numbers[j][i],
                "*" => total *= numbers[j][i],
                op => unreachable!("unknown operator {op}"),
            };
        }

        grand_total += total;
    }

    grand_total
}

fn part_two(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

        assert_eq!(4277556, part_one(test_input));
    }

    #[test]
    fn test_part2() {
        let test_input = "
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

        assert_eq!(3263827, part_two(test_input));
    }
}

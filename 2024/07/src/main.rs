use itertools::{repeat_n, Itertools};

fn main() {
    let input = include_str!("input.txt");

    dbg!(part_one(input));
    dbg!(part_two(input));
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Mul,
    Concat,
}

impl Operator {
    fn eval(&self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Operator::Add => lhs + rhs,
            Operator::Mul => lhs * rhs,
            Operator::Concat => format!("{lhs}{rhs}").parse::<u64>().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Equation {
    test_value: u64,
    operands: Vec<u64>,
}

impl Equation {
    fn eval(&self, operands: &[u64], available_operators: &[Operator]) -> Vec<u64> {
        let permutations = repeat_n(available_operators, operands.len() - 1)
            .multi_cartesian_product()
            .collect_vec();

        let mut results = vec![];

        for permutation in permutations {
            let mut lhs = operands[0];

            for i in 0..operands.len() - 1 {
                let rhs = operands[i + 1];

                lhs = permutation[i].eval(lhs, rhs);
            }

            results.push(lhs);
        }

        results
    }

    fn can_be_made_true(&self, available_operators: &[Operator]) -> bool {
        self.eval(&self.operands, available_operators)
            .into_iter()
            .find(|result| result == &self.test_value)
            .is_some()
    }
}

impl From<&str> for Equation {
    fn from(line: &str) -> Self {
        let mut it = line.split(":");

        let test_value = it
            .next()
            .and_then(|raw_test_value| raw_test_value.parse::<u64>().ok())
            .unwrap();

        let operands = it
            .next()
            .and_then(|raw_operands| {
                Some(
                    raw_operands
                        .trim()
                        .split(" ")
                        .map(|raw_operand| raw_operand.parse::<u64>().unwrap())
                        .collect(),
                )
            })
            .unwrap();

        Equation {
            test_value,
            operands,
        }
    }
}

fn solve(input: &str, operators: &[Operator]) -> u64 {
    input
        .lines()
        .map(Equation::from)
        .filter_map(|equation| {
            equation
                .can_be_made_true(&operators)
                .then(|| equation.test_value)
        })
        .sum()
}

fn part_one(input: &str) -> u64 {
    let operators = vec![Operator::Add, Operator::Mul];
    solve(input, &operators)
}

fn part_two(input: &str) -> u64 {
    let operators = vec![Operator::Add, Operator::Mul, Operator::Concat];
    solve(input, &operators)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part1() {
        assert_eq!(3749, part_one(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(11387, part_two(TEST_INPUT));
    }
}

fn main() {
    let input = include_str!("input.txt");

    dbg!(part_one(input));
    dbg!(part_two(input));
}

fn part_one(input: &str) -> u32 {
    scan_memory(input, false)
}

fn part_two(input: &str) -> u32 {
    scan_memory(input, true)
}

fn scan_memory(input: &str, with_do: bool) -> u32 {
    let mut sum = 0;
    let mut enabled = true;

    let mut it = input.chars();
    while let Some(char) = it.next() {
        if with_do && char == 'd' && it.next() == Some('o') {
            match it.next() {
                Some('(') => {
                    if let Some(')') = it.next() {
                        enabled = true;
                        continue;
                    }
                }
                Some('n') => {
                    if it.next() == Some('\'')
                        && it.next() == Some('t')
                        && it.next() == Some('(')
                        && it.next() == Some(')')
                    {
                        enabled = false;
                        continue;
                    }
                }
                _ => continue,
            }
        }

        if !enabled {
            continue;
        }

        if char == 'm' && it.next() == Some('u') && it.next() == Some('l') && it.next() == Some('(')
        {
            let mut lhs = String::new();
            let mut following_tok = None;

            while let Some(number_candidate) = it.next() {
                match number_candidate {
                    n if n.is_numeric() => lhs += &n.to_string(),
                    c => {
                        following_tok = Some(c);
                        break;
                    }
                }
            }

            if !lhs.is_empty() && following_tok.is_some_and(|t| t == ',') {
                let mut rhs = String::new();
                let mut following_tok = None;

                while let Some(number_candidate) = it.next() {
                    match number_candidate {
                        n if n.is_numeric() => rhs += &n.to_string(),
                        c => {
                            following_tok = Some(c);
                            break;
                        }
                    }
                }

                if !rhs.is_empty() && following_tok.is_some_and(|t| t == ')') {
                    let mul = lhs.parse::<i32>().unwrap() * rhs.parse::<i32>().unwrap();

                    sum += mul;
                }
            }
        }
    }

    sum as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        assert_eq!(161, part_one(test_input));
    }

    #[test]
    fn test_part2() {
        let test_input =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        assert_eq!(48, part_two(test_input));
    }
}

fn main() {
    let input = include_str!("input.txt");

    let (part_one, part_two) = run(input);

    dbg!(part_one);
    dbg!(part_two);
}

fn run(input: &str) -> (u32, u32) {
    let mut it = input.split("\n\n");
    let raw_page_ordering_rules = it.next().unwrap();
    let raw_page_numbers_update = it.next().unwrap();

    let mut sucessors_for_key = std::collections::HashMap::new();

    raw_page_ordering_rules.lines().for_each(|line| {
        let mut it = line.split("|");
        let ancestor = it.next().and_then(|page| page.parse::<u32>().ok()).unwrap();
        let successor = it.next().and_then(|page| page.parse::<u32>().ok()).unwrap();

        sucessors_for_key
            .entry(ancestor)
            .and_modify(|v: &mut Vec<u32>| v.push(successor))
            .or_insert(vec![successor]);
    });

    let page_numbers_update = raw_page_numbers_update
        .lines()
        .map(|line| {
            line.split(",")
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut incorrect_pages = vec![];

    let mut sum_correct = 0;
    let mut sum_incorrect = 0;

    for update in page_numbers_update {
        let mut page_in_order = false;

        for (i, number) in update.iter().enumerate() {
            let remaining_numbers = &update[i + 1..update.len()];

            page_in_order = match sucessors_for_key.get(number) {
                Some(sucessors) => {
                    let is_in_right_order = remaining_numbers
                        .iter()
                        .all(|remaining_number| sucessors.contains(remaining_number));

                    is_in_right_order || remaining_numbers.is_empty()
                }
                None => remaining_numbers.is_empty(),
            };
            if !page_in_order {
                break;
            }
        }

        if page_in_order {
            sum_correct += update[update.len() / 2];
        } else {
            incorrect_pages.push(update);
        }
    }

    for page in incorrect_pages {
        let mut corrected_page = page.clone();

        corrected_page.sort_by(|a, b| match sucessors_for_key.get(a) {
            None => std::cmp::Ordering::Greater,
            Some(numbers) if numbers.contains(b) => std::cmp::Ordering::Less,
            _ => std::cmp::Ordering::Equal,
        });

        sum_incorrect += corrected_page[corrected_page.len() / 2];
    }

    (sum_correct, sum_incorrect)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part1() {
        assert_eq!(143, run(TEST_INPUT).0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(123, run(TEST_INPUT).1);
    }
}

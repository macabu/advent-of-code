fn main() {
    let input = include_str!("input.txt");

    dbg!(part_one(input));
    dbg!(part_two(input));
}

fn part_one(input: &str) -> u64 {
    input
        .split(',')
        .map(|range| {
            let mut parts = range.split('-');

            (
                parts.next().unwrap().parse::<u64>().unwrap(),
                parts.next().unwrap().parse::<u64>().unwrap(),
            )
        })
        .map(|(first, last)| (first..=last).filter(|id| is_invalid_pt1(*id)))
        .flatten()
        .sum::<u64>()
}

fn is_invalid_pt1(id: u64) -> bool {
    // id = 123123
    // digits = 6
    // cutoff = 10^(6/2) = 10^3 = 1000
    // lower_half = 123123 % 1000 = 123
    // upper_half = 123123 / 1000 = 123

    let digits = 1 + id.ilog10();
    if digits % 2 != 0 {
        return false;
    }

    let cutoff = 10u64.pow(digits / 2);

    let lower_half = id % cutoff;
    let upper_half = id / cutoff;

    lower_half == upper_half
}

fn part_two(input: &str) -> u64 {
    input
        .split(',')
        .map(|range| {
            let mut parts = range.split('-');

            (
                parts.next().unwrap().parse::<u64>().unwrap(),
                parts.next().unwrap().parse::<u64>().unwrap(),
            )
        })
        .map(|(first, last)| (first..=last).filter(|id| is_invalid_pt2(*id)))
        .flatten()
        .sum::<u64>()
}

// X 1/2/3/1/2/3/1/2/3
// 12/31/23/123
// X 123/123/123/123
// X 1231/2312/3123
// 12312/31231/23
// X 123123/123123
fn is_invalid_pt2(id: u64) -> bool {
    let digits = 1 + id.ilog10() as u64;

    for divisor in (1..digits).rev() {
        if digits % divisor != 0 {
            continue;
        }

        let cutoff = 10u64.pow(divisor as u32);

        // every time we divide the number by the cutoff, "shift right" by the divisor digits amount
        let mut secnd_part = id % cutoff;
        let mut first_part = id / cutoff;

        while first_part > 0 {
            let next = first_part % cutoff;
            if next != secnd_part {
                // exit early if the chunks dont match
                break;
            }

            secnd_part = next;
            first_part /= cutoff;
        }

        if first_part == 0 {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        assert_eq!(1227775554, part_one(test_input));
    }

    #[test]
    fn test_part2() {
        let test_input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        assert_eq!(4174379265, part_two(test_input));
    }

    #[test]
    fn test_part2_is_invalid_pt2() {
        assert!(is_invalid_pt2(1132));
    }
}

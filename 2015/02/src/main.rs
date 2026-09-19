fn main() {
    let input = include_str!("input.txt");

    dbg!(part_one(input));
    dbg!(part_two(input));
}

fn part_one(input: &str) -> u32 {
    input.lines().fold(0, |acc, line| {
        let dim = line
            .split('x')
            .map(|dim| dim.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        assert!(dim.len() == 3);

        let (l, w, h) = (dim[0], dim[1], dim[2]);

        let lw = l * w;
        let wh = w * h;
        let hl = h * l;
        let min = lw.min(wh).min(hl);

        let wrapping_paper = (2 * lw + 2 * wh + 2 * hl) + min;

        acc + wrapping_paper
    })
}

fn part_two(input: &str) -> u32 {
    input.lines().fold(0, |acc, line| {
        let dim = line
            .split('x')
            .map(|dim| dim.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        assert!(dim.len() == 3);

        let (l, w, h) = (dim[0], dim[1], dim[2]);

        let max = l.max(w).max(h);

        let p = (l + l) + (w + w) + (h + h) - (max + max);
        let v = l * w * h;
        let ribbon = p + v;

        acc + ribbon
    })
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

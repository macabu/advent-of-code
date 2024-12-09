fn main() {
    let input = include_str!("input.txt");

    dbg!(part_one(input));
    dbg!(part_two(input));
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Block {
    File(u64), // id
    FreeSpace,
}

fn part_one(input: &str) -> u64 {
    let disk_layout = input
        .char_indices()
        .flat_map(|(i, c)| {
            let kind = match i % 2 == 0 {
                true => {
                    let id = if i == 0 { 0 } else { i / 2 } as u64;
                    Block::File(id)
                }
                false => Block::FreeSpace,
            };

            let frequency = c.to_digit(10).expect("not a digit") as usize;

            [kind].repeat(frequency)
        })
        .collect::<Vec<_>>();

    let mut optimized_disk_layout = [Block::FreeSpace].repeat(disk_layout.len());

    let mut rev_i = disk_layout.len();

    for i in 0..disk_layout.len() {
        match disk_layout[i] {
            Block::File(_) => {
                if i < rev_i {
                    optimized_disk_layout[i] = disk_layout[i];
                }
            }
            Block::FreeSpace => {
                for j in (i..rev_i).rev() {
                    if let Block::File(_) = disk_layout[j] {
                        optimized_disk_layout[i] = disk_layout[j];
                        rev_i = j;

                        break;
                    }
                }
            }
        }
    }

    optimized_disk_layout
        .iter()
        .enumerate()
        .fold(0, |acc, (i, digit)| match digit {
            Block::File(id) => acc + (i as u64 * id),
            _ => acc,
        })
}

fn part_two(_input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part1() {
        assert_eq!(1928, part_one(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(2858, part_two(TEST_INPUT));
    }
}

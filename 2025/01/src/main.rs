fn main() {
    let input = include_str!("input.txt");

    dbg!(part_one(input));
    dbg!(part_two(input));
}

#[derive(Debug)]
enum Rotation {
    Left(i32),
    Right(i32),
}

impl Rotation {
    fn from_str(s: &str) -> Self {
        let (dir, moves) = s.split_at(1);
        let moves = moves.parse::<i32>().unwrap();

        match dir {
            "L" => Rotation::Left(moves),
            "R" => Rotation::Right(moves),
            _ => unreachable!("Invalid rotation"),
        }
    }

    fn raw_moves(&self) -> i32 {
        match self {
            Rotation::Left(moves) => *moves,
            Rotation::Right(moves) => *moves,
        }
    }

    fn rotate(&self, current: i32) -> i32 {
        match self {
            Rotation::Left(moves) if current - (moves % 100) < 0 => current - (moves % 100) + 100,
            Rotation::Right(moves) if current + (moves % 100) >= 100 => {
                current + (moves % 100) - 100
            }
            Rotation::Left(moves) => current - (moves % 100),
            Rotation::Right(moves) => current + (moves % 100),
        }
    }

    fn rotate2(&self, current: i32) -> (i32, i32) {
        match self {
            Rotation::Left(moves) if current - (moves % 100) <= 0 => {
                // if current == 0 {
                //     return (current - (moves % 100) + 100, moves / 100);
                // };
                (current - (moves % 100) + 100, moves / 100 + 1)
            }
            Rotation::Right(moves) if current + (moves % 100) >= 100 => {
                if current == 0 {
                    return (current + (moves % 100) - 100, moves / 100);
                };
                (current + (moves % 100) - 100, moves / 100 + 1)
            }
            Rotation::Left(moves) => (current - (moves % 100), moves / 100),
            Rotation::Right(moves) => (current + (moves % 100), moves / 100),
        }
    }
}

fn part_one(input: &str) -> u32 {
    let mut zeros = 0;

    input
        .lines()
        .into_iter()
        .map(Rotation::from_str)
        .fold(50, |mut acc, rot| {
            acc = rot.rotate(acc);
            if acc == 0 {
                zeros += 1;
            }
            acc
        });

    zeros
}

fn part_two(input: &str) -> u32 {
    let mut zeros = 0;

    input
        .lines()
        .into_iter()
        .map(Rotation::from_str)
        .fold(50, |mut acc, rot| {
            let (acc2, z2) = rot.rotate2(acc);
            zeros += z2;
            acc = acc2;
            acc
            /*
            let moves = rot.raw_moves();
            for _ in 0..moves {
                acc = match rot {
                    Rotation::Left(_) => {
                        if acc - 1 < 0 {
                            99
                        } else {
                            acc - 1
                        }
                    }
                    Rotation::Right(_) => {
                        if acc + 1 == 100 {
                            0
                        } else {
                            acc + 1
                        }
                    }
                };
                if acc == 0 {
                    zeros += 1;
                }
            }
            acc
            */
        });

    zeros as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        assert_eq!(3, part_one(test_input));
    }

    #[test]
    fn test_part2() {
        let test_input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        assert_eq!(6, part_two(test_input));
    }
}

fn main() {
    let input = include_str!("input.txt");

    dbg!(part_one(input));
    dbg!(part_two(input));
}

fn find_word(grid: &[Vec<char>], word: &str, y: usize, x: usize, dir_y: i32, dir_x: i32) -> u32 {
    let mut y_search = y;
    let mut x_search = x;

    for letter in word.chars() {
        match grid.get(y_search).and_then(|v| v.get(x_search)) {
            Some(grid_letter) if grid_letter == &letter => {
                y_search = (y_search as i32 + dir_y) as usize;
                x_search = (x_search as i32 + dir_x) as usize;
                continue;
            }
            _ => {
                return 0;
            }
        }
    }

    1
}

fn part_one(input: &str) -> u32 {
    let word_search = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut found = 0;

    let word = "XMAS";

    for y in 0..word_search.len() {
        for x in 0..word_search[y].len() {
            // Right
            found += find_word(&word_search, word, y, x, 0, 1);

            // Left
            if x > 2 {
                found += find_word(&word_search, word, y, x, 0, -1);
            }

            // Down
            found += find_word(&word_search, word, y, x, 1, 0);

            // Up
            if y > 2 {
                found += find_word(&word_search, word, y, x, -1, 0);
            }

            // Diagonal Down+Right
            found += find_word(&word_search, word, y, x, 1, 1);

            // Diagonal Down+Left
            if x > 2 {
                found += find_word(&word_search, word, y, x, 1, -1);
            }

            // Diagonal Up+Right
            if y > 2 {
                found += find_word(&word_search, word, y, x, -1, 1);
            }

            // Diagonal Up+Left
            if y > 2 && x > 2 {
                found += find_word(&word_search, word, y, x, -1, -1);
            }
        }
    }

    found
}

fn find_a(
    grid: &[Vec<char>],
    y: usize,
    x: usize,
    dir_y: i32,
    dir_x: i32,
) -> Option<(usize, usize)> {
    let mut y_search = y;
    let mut x_search = x;

    let mut y_a = 0;
    let mut x_a = 0;

    for letter in "MAS".chars() {
        match grid.get(y_search).and_then(|v| v.get(x_search)) {
            Some(grid_letter) if grid_letter == &letter => {
                if letter == 'A' {
                    y_a = y_search;
                    x_a = x_search;
                }

                y_search = (y_search as i32 + dir_y) as usize;
                x_search = (x_search as i32 + dir_x) as usize;
                continue;
            }
            _ => {
                return None;
            }
        }
    }

    Some((y_a, x_a))
}

fn part_two(input: &str) -> u32 {
    let word_search = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // Crosses
    let mut hm: std::collections::HashMap<(usize, usize), u32> = std::collections::HashMap::new();

    for y in 0..word_search.len() {
        for x in 0..word_search[y].len() {
            // Diagonal Down+Right
            if let Some((yf, xf)) = find_a(&word_search, y, x, 1, 1) {
                hm.entry((yf, xf)).and_modify(|e| *e += 1).or_insert(1);
            }

            // Diagonal Down+Left
            if x > 1 {
                if let Some((yf, xf)) = find_a(&word_search, y, x, 1, -1) {
                    hm.entry((yf, xf)).and_modify(|e| *e += 1).or_insert(1);
                }
            }

            // Diagonal Up+Right
            if y > 1 {
                if let Some((yf, xf)) = find_a(&word_search, y, x, -1, 1) {
                    hm.entry((yf, xf)).and_modify(|e| *e += 1).or_insert(1);
                }
            }

            // Diagonal Up+Left
            if y > 1 && x > 1 {
                if let Some((yf, xf)) = find_a(&word_search, y, x, -1, -1) {
                    hm.entry((yf, xf)).and_modify(|e| *e += 1).or_insert(1);
                }
            }
        }
    }

    hm.values().filter(|value| value == &&2).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        assert_eq!(18, part_one(test_input));
    }

    #[test]
    fn test_part2() {
        let test_input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        assert_eq!(9, part_two(test_input));
    }
}

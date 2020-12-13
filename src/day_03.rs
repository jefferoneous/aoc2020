use std::fs::File;
use std::io::{BufRead, BufReader, Error as IoError};
use std::path::PathBuf;

fn count_trees(grid: &Vec<String>, h_delta: u8, v_delta: u8) -> usize {
    let mut h_pos = 0;
    let mut v_pos = 0;
    let mut count: usize = 0;

    let grid_width = grid[0].len();

    loop {
        h_pos += h_delta as usize;
        v_pos += v_delta as usize;

        if v_pos >= grid.len() {
            break;
        }

        if let Some(c) = grid[v_pos].chars().nth(h_pos % grid_width) {
            if c == '#' {
                count += 1;
            }
        }
    }

    count
}

fn part_one(list: &Vec<String>) {
    println!("Part One\n========");
    let count = count_trees(&list, 3, 1);
    println!("Trees encountered: {}", count);
}

fn part_two(list: &Vec<String>) {
    println!("Part Two\n========");

    let slopes: Vec<(u8, u8)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let result = slopes
        .iter()
        .map(|(x, y)| count_trees(&list, *x, *y))
        .fold(1, |p, c| p * c);

    println!("Trees encountered: {}", result);
}

fn load_list_from_file(path: PathBuf) -> Result<Vec<String>, IoError> {
    let input = File::open(path)?;
    let buf = BufReader::new(input);
    let result = buf.lines().map(|l| l.unwrap()).collect();

    Ok(result)
}

pub fn run(path: PathBuf) {
    match load_list_from_file(path) {
        Ok(list) => {
            part_one(&list);
            part_two(&list);
        }
        Err(e) => eprintln!("Error occurred while reading input file: {}", e),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_data() -> Vec<String> {
        let raw_data = [
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ];
        raw_data.iter().map(|l| l.to_string()).collect()
    }

    #[test]
    fn counts_correct_number_of_trees() {
        let grid = test_data();

        let count = count_trees(&grid, 3, 1);
        assert_eq!(7, count);
    }
}

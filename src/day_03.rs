fn count_trees(grid: &[String], h_delta: u8, v_delta: u8) -> usize {
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

pub fn part_one(data: &[String]) {
    let count = count_trees(data, 3, 1);
    println!("Trees encountered: {}", count);
}

pub fn part_two(data: &[String]) {
    let slopes: Vec<(u8, u8)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let result = slopes
        .iter()
        .map(|(x, y)| count_trees(&data, *x, *y))
        .fold(1, |p, c| p * c);

    println!("Trees encountered: {}", result);
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

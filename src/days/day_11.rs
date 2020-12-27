use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// Kind of a cheat. I created this type in a different project.
use super::grid::Grid;

use super::DayRunner;

pub fn runner(data: Vec<String>) -> DayRunner {
    DayRunner::new(data, Some(part_one), Some(part_two))
}

fn part_one(data: &[String]) {
    let mut grid = parse_into_grid(data);
    while tick(&mut grid, 4, false) {}
    let occupied_seats = count_occupied_seats(&grid);
    println!("Occupied seats: {}", occupied_seats);
}

fn part_two(data: &[String]) {
    let mut grid = parse_into_grid(data);
    while tick(&mut grid, 5, true) {}
    let occupied_seats = count_occupied_seats(&grid);
    println!("Occupied seats: {}", occupied_seats);
}

fn parse_into_grid(data: &[String]) -> Grid {
    let columns = data[0].len() as u32;
    let rows = data.len() as u32;

    Grid::new(
        rows,
        columns,
        data.iter().map(|s| s.chars()).flatten().collect(),
    )
    .unwrap()
}

fn tick(grid: &mut Grid, tolerance: u32, visible: bool) -> bool {
    let previous_hash = calculate_hash(&grid);

    let analysis = match visible {
        false => analyze_neighbors(&grid),
        true => analyze_visible_neighbors(&grid),
    };

    for (r, c, state, neighbor_count) in analysis {
        match state {
            'L' => {
                if neighbor_count == 0 {
                    grid.set(r, c, '#');
                }
            }
            '#' => {
                if neighbor_count >= tolerance {
                    grid.set(r, c, 'L');
                }
            }
            _ => {}
        }
    }

    calculate_hash(&grid) != previous_hash
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn count_occupied_neighbors(grid: &Grid, row: u32, column: u32) -> u32 {
    grid.neighbors(row, column)
        .iter()
        .filter(|&&state| state == '#')
        .count() as u32
}

fn count_nearest_visible_occupied_neighbors(grid: &Grid, row: u32, column: u32) -> u32 {
    grid.visible_neighbors(row, column)
        .iter()
        .filter(|&&state| state == '#')
        .count() as u32
}

fn count_occupied_seats(grid: &Grid) -> u32 {
    grid.count_cells_in_state('#')
}

fn analyze_neighbors(grid: &Grid) -> Vec<(u32, u32, char, u32)> {
    let mut results = vec![];

    for r in 0..grid.get_rows() {
        for c in 0..grid.get_columns() {
            results.push((r, c, grid.get(r, c), count_occupied_neighbors(&grid, r, c)));
        }
    }

    results
}

fn analyze_visible_neighbors(grid: &Grid) -> Vec<(u32, u32, char, u32)> {
    let mut results = vec![];

    for r in 0..grid.get_rows() {
        for c in 0..grid.get_columns() {
            results.push((
                r,
                c,
                grid.get(r, c),
                count_nearest_visible_occupied_neighbors(&grid, r, c),
            ));
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day_11_counts_occupied_neighbors() {
        let grid = Grid::new(3, 3, "LL#..###L".chars().collect()).unwrap();

        assert_eq!(4, count_occupied_neighbors(&grid, 1, 1));
    }

    #[test]
    fn day_11_counts_all_occupied_seats() {
        let grid = Grid::new(4, 4, "LL#..###LLL..###".chars().collect()).unwrap();

        assert_eq!(7, count_occupied_seats(&grid));
    }

    #[test]
    fn day_11_fills_all_seats_from_start() {
        let mut grid: Grid = Grid::new(
            10,
            10,
            "L.LL.LL.LLLLLLLLL.LLL.L.L..L..LLLL.LL.LLL.LL.LL.LLL.LLLLL.LL..L.L.....LLLLLLLLLLL.LLLLLL.LL.LLLLL.LL".chars().collect()).unwrap();
        let expected: Grid = Grid::new(
            10,
            10,
            "#.##.##.#########.###.#.#..#..####.##.###.##.##.###.#####.##..#.#.....###########.######.##.#####.##".chars().collect()).unwrap();

        let state_changed = tick(&mut grid, 4, false);

        assert_eq!(expected, grid);
        assert!(state_changed);
    }

    #[test]
    fn day_11_correctly_updates_second_round() {
        let mut grid: Grid = Grid::new(
            10,
            10,
            "#.##.##.#########.###.#.#..#..####.##.###.##.##.###.#####.##..#.#.....###########.######.##.#####.##".chars().collect()).unwrap();
        let expected: Grid = Grid::new(
            10,
            10,
            "#.LL.L#.###LLLLLL.L#L.L.L..L..#LLL.LL.L##.LL.LL.LL#.LLLL#.##..L.L.....#LLLLLLLL##.LLLLLL.L#.#LLLL.##".chars().collect()).unwrap();

        let state_changed = tick(&mut grid, 4, false);

        assert_eq!(expected, grid);
        assert!(state_changed);
    }

    #[test]
    fn day_11_detects_no_change_in_grid() {
        let mut grid: Grid = Grid::new(
            10,
            10,
            "#.#L.L#.###LLL#LL.L#L.#.L..#..#L##.##.L##.#L.LL.LL#.#L#L#.##..L.L.....#L#L##L#L##.LLLLLL.L#.#L#L#.##".chars().collect()).unwrap();

        assert_eq!(false, tick(&mut grid, 4, false));
    }

    #[test]
    fn day_11_counts_nearest_visible_neighbors() {
        let grid = Grid::new(
            9,
            9,
            ".......#...........#..................#L....#....#.............#...........#....."
                .chars()
                .collect(),
        )
        .unwrap();

        assert_eq!(7, count_nearest_visible_occupied_neighbors(&grid, 4, 3));
    }
}

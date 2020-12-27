use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use super::grid::Grid;

use super::DayRunner;

pub fn runner(data: Vec<String>) -> DayRunner {
    DayRunner::new(data, Some(part_one), None)
}

type Floor = Grid<char>;

fn part_one(data: &[String]) {
    let mut grid = parse_into_grid(data);
    while tick(&mut grid) {}
    let occupied_seats = count_occupied_seats(&grid);
    println!("Occupied seats: {}", occupied_seats);
}

fn parse_into_grid(data: &[String]) -> Floor {
    let columns = "LLLLLL.LLLLLL.LLLLL.LLLLLLLLLLLLLLL.LLLLLLL.LLLLL.L.LLLLLLL.LLLLLLLLLLLLLL.LLL.L.L.LLLLLLLLL.LLLLL".len() as u32;
    let rows = 97;
    Floor::with_cells(
        rows,
        columns,
        data.iter().map(|s| s.chars()).flatten().collect(),
    )
    .unwrap()
}

fn tick(grid: &mut Floor) -> bool {
    let previous_hash = calculate_hash(&grid);

    for (r, c, state, neighbor_count) in analyze_grid(&grid) {
        match state {
            'L' => {
                if neighbor_count == 0 {
                    grid.set(r, c, '#');
                }
            }
            '#' => {
                if neighbor_count >= 4 {
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

fn count_occupied_neighbors(grid: &Floor, row: u32, column: u32) -> u32 {
    grid.neighbors(row, column)
        .iter()
        .filter(|&&state| state == '#')
        .count() as u32
}

fn count_occupied_seats(grid: &Floor) -> u32 {
    grid.count_cells_in_state('#')
}

fn analyze_grid(grid: &Floor) -> Vec<(u32, u32, char, u32)> {
    let mut results = vec![];

    for r in 0..grid.get_rows() {
        for c in 0..grid.get_columns() {
            results.push((r, c, grid.get(r, c), count_occupied_neighbors(&grid, r, c)));
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day_11_counts_occupied_neighbors() {
        let grid = Grid::with_cells(3, 3, "LL#..###L".chars().collect()).unwrap();

        assert_eq!(4, count_occupied_neighbors(&grid, 1, 1));
    }

    #[test]
    fn day_11_counts_all_occupied_seats() {
        let grid = Grid::with_cells(4, 4, "LL#..###LLL..###".chars().collect()).unwrap();

        assert_eq!(7, count_occupied_seats(&grid));
    }

    #[test]
    fn day_11_fills_all_seats_from_start() {
        let mut grid: Floor = Grid::with_cells(
            10,
            10,
            "L.LL.LL.LLLLLLLLL.LLL.L.L..L..LLLL.LL.LLL.LL.LL.LLL.LLLLL.LL..L.L.....LLLLLLLLLLL.LLLLLL.LL.LLLLL.LL".chars().collect()).unwrap();
        let expected: Floor = Grid::with_cells(
            10,
            10,
            "#.##.##.#########.###.#.#..#..####.##.###.##.##.###.#####.##..#.#.....###########.######.##.#####.##".chars().collect()).unwrap();

        let state_changed = tick(&mut grid);

        assert_eq!(expected, grid);
        assert!(state_changed);
    }

    #[test]
    fn day_11_correctly_updates_second_round() {
        let mut grid: Floor = Grid::with_cells(
            10,
            10,
            "#.##.##.#########.###.#.#..#..####.##.###.##.##.###.#####.##..#.#.....###########.######.##.#####.##".chars().collect()).unwrap();
        let expected: Floor = Grid::with_cells(
            10,
            10,
            "#.LL.L#.###LLLLLL.L#L.L.L..L..#LLL.LL.L##.LL.LL.LL#.LLLL#.##..L.L.....#LLLLLLLL##.LLLLLL.L#.#LLLL.##".chars().collect()).unwrap();

        let state_changed = tick(&mut grid);

        assert_eq!(expected, grid);
        assert!(state_changed);
    }

    #[test]
    fn day_11_detects_no_change_in_grid() {
        let mut grid: Floor = Grid::with_cells(
            10,
            10,
            "#.#L.L#.###LLL#LL.L#L.#.L..#..#L##.##.L##.#L.LL.LL#.#L#L#.##..L.L.....#L#L##L#L##.LLLLLL.L#.#L#L#.##".chars().collect()).unwrap();

        assert_eq!(false, tick(&mut grid));
    }
}

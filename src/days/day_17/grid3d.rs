use std::collections::HashSet;

type Coord3d = (i64, i64, i64);

const NEIGHBOR_DELTAS: [Coord3d; 26] = [
    (-1, -1, -1),
    (-1, -1, 0),
    (-1, -1, 1),
    (-1, 0, -1),
    (-1, 0, 0),
    (-1, 0, 1),
    (-1, 1, -1),
    (-1, 1, 0),
    (-1, 1, 1),
    (0, -1, -1),
    (0, -1, 0),
    (0, -1, 1),
    (0, 0, -1),
    (0, 0, 1),
    (0, 1, -1),
    (0, 1, 0),
    (0, 1, 1),
    (1, -1, -1),
    (1, -1, 0),
    (1, -1, 1),
    (1, 0, -1),
    (1, 0, 0),
    (1, 0, 1),
    (1, 1, -1),
    (1, 1, 0),
    (1, 1, 1),
];

pub struct Grid3d {
    active_coords: HashSet<Coord3d>,
}

impl Grid3d {
    pub fn new(raw_cells: &[&[&str]]) -> Self {
        let mut active_cells: HashSet<Coord3d> = HashSet::new();
        let oz = (raw_cells.len() / 2) as i64;

        for (iz, grid) in raw_cells.iter().enumerate() {
            let oy = (grid.len() / 2) as i64;
            for (iy, row) in grid
                .iter()
                .rev()
                .enumerate()
                .filter(|(_, s)| s.contains('#'))
            {
                let ox = (row.len() / 2) as i64;
                row.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '#')
                    .for_each(|(ix, _)| {
                        let x = (ix as i64) - ox;
                        let y = (iy as i64) - oy;
                        let z = (iz as i64) - oz;
                        active_cells.insert((x, y, z));
                    });
            }
        }

        Self {
            active_coords: active_cells,
        }
    }

    pub fn tick(&mut self) {
        let mut new_active_coords: HashSet<Coord3d> = HashSet::new();

        let mut candidates: HashSet<Coord3d> = HashSet::new();
        candidates.extend(self.active_coords.iter());

        for coord in &self.active_coords {
            candidates.extend(
                NEIGHBOR_DELTAS
                    .iter()
                    .map(|(dx, dy, dz)| (coord.0 + *dx, coord.1 + *dy, coord.2 + *dz)),
            )
        }

        for candidate in candidates {
            let count = NEIGHBOR_DELTAS
                .iter()
                .map(|(dx, dy, dz)| (candidate.0 + *dx, candidate.1 + *dy, candidate.2 + *dz))
                .filter(|c| self.active_coords.contains(c))
                .count();

            if self.active_coords.contains(&candidate) {
                if count == 2 || count == 3 {
                    new_active_coords.insert(candidate);
                }
            } else {
                if count == 3 {
                    new_active_coords.insert(candidate);
                }
            }
        }

        self.active_coords = new_active_coords;
    }

    pub fn run(&mut self, cycles: u32) {
        for _ in 0..cycles {
            self.tick();
        }
    }

    pub fn active_cube_count(&self) -> u32 {
        self.active_coords.len() as u32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day_17_grid3d_parses_state() {
        let grid = Grid3d::new(&vec![&vec![".#.", "..#", "###"][..]]);
        let coords: Vec<Coord3d> = vec![(-1, -1, 0), (0, -1, 0), (1, -1, 0), (1, 0, 0), (0, 1, 0)];
        let mut expected: HashSet<Coord3d> = HashSet::new();
        for coord in coords {
            expected.insert(coord);
        }

        assert_eq!(expected, grid.active_coords);
    }

    #[test]
    fn day_17_grid3d_generates_correct_next_state() {
        let mut grid = Grid3d::new(&vec![&vec![".#.", "..#", "###"][..]]);
        let expected = Grid3d::new(&vec![
            &vec![".....", ".....", ".#...", "...#.", "..#.."][..],
            &vec![".....", ".....", ".#.#.", "..##.", "..#.."][..],
            &vec![".....", ".....", ".#...", "...#.", "..#.."][..],
        ]);

        grid.tick();

        assert_eq!(expected.active_coords, grid.active_coords);
    }
}

use std::collections::{HashMap, HashSet};
use std::fmt::{Binary, Display};
use std::num::ParseIntError;

pub fn part_one(data: &[&str]) {
    if let Ok(tiles) = parse_tiles(&data) {
        let ids: HashSet<u64> = find_corner_tile_ids(&tiles);
        let product: u64 = ids.iter().product();
        println!("Product: {}", product);
    } else {
        println!("An error occurred");
    }
}

pub fn part_two(data: &[&str]) {
    if let Ok(tiles) = parse_tiles(&data) {
        let arranged_tiles = arrange_tiles(tiles);
        let composite_image = compose_image(&arranged_tiles);
        let sea_monsters_removed = remove_sea_monsters(&composite_image);
        let roughness = calculate_roughness(&sea_monsters_removed);

        println!("Roughness: {}", roughness);
    } else {
        println!("An error occurred");
    }
}

type Tiles = HashMap<u64, Tile>;

fn reverse_bits(n: u128, width: u8) -> u128 {
    let mut g = n;
    let mut result = 0;
    for i in (0..width).rev() {
        result += (g & 1) << i;
        g = g >> 1;
    }

    result
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Tile {
    id: u64,
    image_data: Vec<u64>,
    image_dimension: u64,
    edge_values: Vec<u64>,
    oriented: bool,
    affixed: bool,
    top: Option<u64>,
    right: Option<u64>,
    bottom: Option<u64>,
    left: Option<u64>,
}

impl Tile {
    fn parse(data: &[&str]) -> Result<Self, ParseIntError> {
        let id: u64 = data[0]
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse()?;

        let image_dimension = data[0].len() as u64;
        let image_data = parse_image(&data[1..]);

        let mut result = Self {
            id,
            image_data,
            image_dimension,
            edge_values: vec![],
            oriented: false,
            affixed: false,
            top: None,
            right: None,
            bottom: None,
            left: None,
        };
        result.calculate_edge_values();

        Ok(result)
    }

    fn neighbor_count(&self) -> u8 {
        [self.top, self.right, self.bottom, self.left]
            .iter()
            .filter_map(|s| *s)
            .count() as u8
    }

    fn rotate_cw(&mut self) {
        let mut new_image_data: Vec<u64> = vec![0; self.image_dimension as usize];

        self.image_data
            .iter()
            .rev()
            .map(|n| reverse_bits(*n as u128, self.image_dimension as u8))
            .for_each(|n| {
                for i in 0..self.image_dimension as usize {
                    let bit = (n >> i) & 1;
                    new_image_data[i] = (new_image_data[i] << 1) + bit as u64;
                }
            });

        self.image_data = new_image_data;
        self.calculate_edge_values();

        let x = self.top;
        self.top = self.left;
        self.left = self.bottom;
        self.bottom = self.right;
        self.right = x;
    }

    fn rotate_ccw(&mut self) {
        let mut new_image_data: Vec<u64> = vec![0; self.image_dimension as usize];

        self.image_data.iter().for_each(|n| {
            for i in 0..self.image_dimension as usize {
                let bit = (n >> i) & 1;
                new_image_data[i] = (new_image_data[i] << 1) + bit;
            }
        });

        self.image_data = new_image_data;
        self.calculate_edge_values();

        let x = self.top;
        self.top = self.right;
        self.right = self.bottom;
        self.bottom = self.left;
        self.left = x;
    }

    fn rotate_180(&mut self) {
        self.image_data = self
            .image_data
            .iter()
            .rev()
            .map(|n| reverse_bits(*n as u128, self.image_dimension as u8) as u64)
            .collect();
        self.calculate_edge_values();

        let x = self.left;
        self.left = self.right;
        self.right = x;
        let x = self.top;
        self.top = self.bottom;
        self.bottom = x;
    }

    fn flip_right_to_left(&mut self) {
        self.image_data = self
            .image_data
            .iter()
            .map(|n| reverse_bits(*n as u128, self.image_dimension as u8) as u64)
            .collect();
        self.calculate_edge_values();

        let x = self.left;
        self.left = self.right;
        self.right = x;
    }

    fn flip_top_to_bottom(&mut self) {
        self.image_data = self.image_data.iter().rev().cloned().collect();
        self.calculate_edge_values();

        let x = self.top;
        self.top = self.bottom;
        self.bottom = x;
    }

    /// - an edge value is the value resulting from reading each '#'
    ///   and '.' in the edge as 1 and 0, respectively, and interpreting
    ///   the result as a binary value
    ///   - top and bottom edges are read left-to-right, MSB first
    ///   - left and right edges are read top-to-bottom, MSB first
    /// - edge values are in a vec of len 8 holding pairs of values
    /// - the pairs correspond to the top, right, bottom, and left
    ///   edges, in that order
    /// - the first value in each pair is the direct numeric value
    ///   of the "bits" along the edge, as explained above
    /// - the second value is computed from the reversal of the bits in
    ///   the first value
    fn calculate_edge_values(&mut self) {
        let mut result = vec![];

        // top
        result.push(self.image_data[0]);
        result.push(reverse_bits(self.image_data[0] as u128, self.image_dimension as u8) as u64);

        // right
        result.push(
            self.image_data
                .iter()
                .rev()
                .enumerate()
                .map(|(i, n)| (n & 1) << i)
                .sum(),
        );
        result.push(
            self.image_data
                .iter()
                .enumerate()
                .map(|(i, n)| (n & 1) << i)
                .sum(),
        );

        // bottom
        result.push(reverse_bits(
            self.image_data[self.image_data.len() - 1] as u128,
            self.image_dimension as u8,
        ) as u64);
        result.push(self.image_data[self.image_data.len() - 1]);

        // left
        result.push(
            self.image_data
                .iter()
                .enumerate()
                .map(|(i, n)| {
                    (reverse_bits(*n as u128, self.image_dimension as u8) as u64 & 1) << i
                })
                .sum(),
        );
        result.push(
            self.image_data
                .iter()
                .rev()
                .enumerate()
                .map(|(i, n)| {
                    (reverse_bits(*n as u128, self.image_dimension as u8) as u64 & 1) << i
                })
                .sum(),
        );

        self.edge_values = result;
    }
}

fn parse_image(data: &[&str]) -> Vec<u64> {
    let mut result: Vec<u64> = vec![];

    for line in data {
        let n = line
            .chars()
            .map(|c| match c {
                '#' => 1,
                _ => 0,
            } as u64)
            .fold(0, |acc, n| (acc << 1) + n);
        result.push(n);
    }

    result
}

fn flip_image<T: Copy>(image: &Vec<T>) -> Vec<T> {
    let new_image_data: Vec<T> = image.iter().rev().copied().collect();

    new_image_data
}

fn parse_tiles(data: &[&str]) -> Result<Tiles, ParseIntError> {
    let mut tiles: Tiles = HashMap::new();

    for d in data.split(|l| l.is_empty()) {
        let tile = Tile::parse(d)?;
        tiles.insert(tile.id, tile);
    }

    Ok(tiles)
}

fn find_neighbor_ids(tiles: &Tiles) -> HashMap<u64, Vec<u64>> {
    let mut result = HashMap::new();

    for (id, tile) in tiles {
        for edge_value in tile.edge_values.iter().step_by(2) {
            if let Some(&other_id) =
                tiles
                    .iter()
                    .filter(|(id, _)| **id != tile.id)
                    .find_map(|(id, t)| {
                        if t.edge_values.contains(edge_value) {
                            Some(id)
                        } else {
                            None
                        }
                    })
            {
                result.entry(*id).or_insert(vec![]).push(other_id);
            }
        }
    }

    result
}

fn arrange_tiles(tiles: Tiles) -> Tiles {
    let mut result = tiles.clone();

    fn connect_neighbors(tile_id: u64, result: &mut Tiles, neighbor_ids: &HashMap<u64, Vec<u64>>) {
        if result.get(&tile_id).unwrap().affixed {
            return;
        }

        fn get_common_edge_positions(first: &Vec<u64>, second: &Vec<u64>) -> (usize, usize) {
            for (p1, v1) in first.iter().enumerate().step_by(2) {
                if let Some(p2) = second.iter().position(|v2| *v2 == *v1) {
                    return (p1, p2);
                }
            }
            (0, 0)
        }

        for other_id in neighbor_ids.get(&tile_id).cloned().unwrap() {
            let tile = result.get(&tile_id).cloned().unwrap();
            let other_tile = result.get(&other_id).cloned().unwrap();

            let (pos, other_pos) =
                get_common_edge_positions(&tile.edge_values, &other_tile.edge_values);

            fn orient_neighbor(tile: &mut Tile, pos: usize, other_pos: usize) {
                if tile.oriented {
                    return;
                }

                let target_side = (2 + pos as i8 / 2) % 4;
                let other_side = other_pos as i8 / 2;

                match target_side - other_side {
                    2 | -2 => tile.rotate_180(),
                    -1 | 3 => tile.rotate_ccw(),
                    -3 | 1 => tile.rotate_cw(),
                    _ => {}
                }
                if (pos + other_pos) % 2 == 0 {
                    match pos / 2 {
                        0 | 2 => tile.flip_right_to_left(),
                        _ => tile.flip_top_to_bottom(),
                    }
                }

                tile.oriented = true;
            }

            match pos / 2 {
                0 => {
                    result.entry(tile.id).and_modify(|e| e.top = Some(other_id));
                }
                1 => {
                    result
                        .entry(tile.id)
                        .and_modify(|e| e.right = Some(other_id));
                }
                2 => {
                    result
                        .entry(tile.id)
                        .and_modify(|e| e.bottom = Some(other_id));
                }
                3 => {
                    result
                        .entry(tile.id)
                        .and_modify(|e| e.left = Some(other_id));
                }
                _ => {}
            };
            result.entry(other_id).and_modify(|e| {
                orient_neighbor(e, pos, other_pos);
            });
        }

        result.entry(tile_id).and_modify(|e| e.affixed = true);

        let tile = result.get(&tile_id).unwrap();
        [tile.top, tile.right, tile.bottom, tile.left]
            .iter()
            .filter_map(|s| *s)
            .for_each(|id| {
                connect_neighbors(id, result, neighbor_ids);
            });
    }

    let neighbor_ids = find_neighbor_ids(&tiles);
    if let Some(current_tile_id) =
        neighbor_ids
            .iter()
            .find_map(|(id, v)| if v.len() == 2 { Some(*id) } else { None })
    {
        result
            .entry(current_tile_id)
            .and_modify(|tile| tile.oriented = true);
        connect_neighbors(current_tile_id, &mut result, &neighbor_ids);
    }

    result
}

fn find_corner_tile_ids(tiles: &Tiles) -> HashSet<u64> {
    find_neighbor_ids(&tiles)
        .iter()
        .filter_map(|(id, v)| if v.len() == 2 { Some(id) } else { None })
        .cloned()
        .collect()
}

fn collect_tile_centers(tiles: &Tiles) -> Vec<Vec<Vec<u64>>> {
    const MASK: u64 = 0b0111111110;

    let mut result = vec![];

    let mut leftmost_tile = tiles
        .values()
        .find(|t| t.neighbor_count() == 2 && t.right.is_some() && t.bottom.is_some());
    let mut current_tile = leftmost_tile;

    while let Some(left_tile) = leftmost_tile {
        let mut jv = vec![];
        while let Some(cur_tile) = current_tile {
            let mut kv = vec![];
            for k in cur_tile.image_data.iter().skip(1).take(8) {
                kv.push((k & MASK) >> 1);
            }
            jv.push(kv);
            if let Some(id) = cur_tile.right {
                current_tile = tiles.get(&id);
            } else {
                current_tile = None;
            }
        }
        result.push(jv);
        if let Some(id) = left_tile.bottom {
            leftmost_tile = tiles.get(&id);
            current_tile = leftmost_tile;
        } else {
            leftmost_tile = None;
        }
    }

    result
}

fn compose_image(tiles: &Tiles) -> Vec<u128> {
    let mut result = vec![];

    let centers = collect_tile_centers(&tiles);

    for row in centers {
        for i in 0..8 {
            let n = row
                .iter()
                .map(|tile| tile[i])
                .fold(0, |acc, v| (acc << 8) + v as u128);
            result.push(n);
        }
    }

    result
}

fn format_image<T: Display + Binary>(image: &Vec<T>, width: usize) -> String {
    let mut result = String::new();

    for row in image {
        result += &format!("{:0w$b}\n", row, w = width);
    }

    result
}

fn remove_sea_monsters(image: &Vec<u128>) -> Vec<u128> {
    let mut result = image.clone();

    fn apply_mask(image: &Vec<u128>) -> Option<Vec<u128>> {
        let mut working_image = image.clone();
        let sea_monster_mask: Vec<u128> = vec![
            0b00000000000000000010,
            0b10000110000110000111,
            0b01001001001001001000,
        ];

        const MASK_WIDTH: usize = 20;

        let mut found_sea_monster = false;

        for iteration in 0..4 {
            println!("Scanning image for sea monsters");
            println!(
                "Image: \n{}",
                format_image(&working_image, working_image.len())
            );
            for image_row in 0..(working_image.len() - sea_monster_mask.len()) {
                for shift in 0..(working_image.len() - MASK_WIDTH) {
                    if sea_monster_mask
                        .iter()
                        .enumerate()
                        .all(|(i, m)| (*m << shift) & working_image[image_row + i] == (*m << shift))
                    {
                        for (i, m) in sea_monster_mask.iter().enumerate() {
                            working_image[image_row + i] =
                                (*m << shift) ^ working_image[image_row + i];
                        }
                        println!("Sea monster found");
                        found_sea_monster = true;
                    }
                }
            }
            if found_sea_monster {
                return Some(working_image);
            }
            if iteration < 3 {
                println!(
                    "Scan #{} found no sea monsters. Rotating image.",
                    iteration + 1
                );
                working_image = rotate_image(working_image);
            }
        }

        None
    }

    fn rotate_image(image: Vec<u128>) -> Vec<u128> {
        let mut new_image: Vec<u128> = vec![0; image.len()];

        image
            .iter()
            .rev()
            .map(|n| reverse_bits(*n, image.len() as u8))
            .for_each(|n| {
                for i in 0..image.len() {
                    let bit = (n >> i) & 1;
                    new_image[i] = (new_image[i] << 1) + bit;
                }
            });

        new_image
    }

    if let Some(result) = apply_mask(&result) {
        return result;
    } else {
        println!("No sea monsters found after 3 rotations. Flipping image and trying again.");
        result = flip_image(&result);
        if let Some(result) = apply_mask(&result) {
            return result;
        }
    }

    result
}

fn count_set_bits(n: u128, width: u8) -> u64 {
    (0..width).fold(0, |acc, i| acc + ((n >> i) & 1)) as u64
}

fn calculate_roughness(image: &Vec<u128>) -> u64 {
    image
        .iter()
        .fold(0, |acc, r| acc + count_set_bits(*r, 12 * 8))
}

#[cfg(test)]
mod test;

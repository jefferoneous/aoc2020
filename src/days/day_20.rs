use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;

pub fn part_one(data: &[&str]) {
    if let Ok(ids) = find_corner_tile_ids(&data) {
        let product: u64 = ids.iter().product();
        println!("Product: {}", product);
    } else {
        println!("An error occurred");
    }
}

pub fn part_two(_data: &[&str]) {
    todo!("do something and print the result");
}

struct Tile {
    id: u64,
    image_data: String,
}

impl Tile {
    fn parse(data: &[&str]) -> Result<Self, ParseIntError> {
        let id: u64 = data[0]
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse()?;

        let image_data = String::from(data[1..].join("\n"));

        Ok(Self { id, image_data })
    }

    fn edge_values(&self) -> Vec<u64> {
        let mut result = vec![];

        result.push(calculate_edge_value(
            self.image_data.lines().nth(0).unwrap(),
        ));
        result.push(calculate_edge_value(
            &self
                .image_data
                .lines()
                .nth(0)
                .unwrap()
                .chars()
                .rev()
                .collect::<String>(),
        ));
        result.push(calculate_edge_value(
            &self
                .image_data
                .lines()
                .map(|s| s.chars().last().unwrap())
                .collect::<String>(),
        ));
        result.push(calculate_edge_value(
            &self
                .image_data
                .lines()
                .map(|s| s.chars().last().unwrap())
                .rev()
                .collect::<String>(),
        ));
        result.push(calculate_edge_value(
            self.image_data.lines().last().unwrap(),
        ));
        result.push(calculate_edge_value(
            &self
                .image_data
                .lines()
                .last()
                .unwrap()
                .chars()
                .rev()
                .collect::<String>(),
        ));
        result.push(calculate_edge_value(
            &self
                .image_data
                .lines()
                .map(|s| s.chars().nth(0).unwrap())
                .collect::<String>(),
        ));
        result.push(calculate_edge_value(
            &self
                .image_data
                .lines()
                .map(|s| s.chars().nth(0).unwrap())
                .rev()
                .collect::<String>(),
        ));

        result
    }
}

fn parse_tiles(data: &[&str]) -> Result<Vec<Tile>, ParseIntError> {
    data.split(|l| l.is_empty()).map(Tile::parse).collect()
}

fn calculate_edge_value(edge: &str) -> u64 {
    edge.chars().fold(0, |acc, c| match c {
        '#' => 2 * acc + 1,
        _ => 2 * acc,
    })
}

fn collate_edge_values(tiles: &Vec<Tile>) -> HashMap<u64, Vec<u64>> {
    let mut result: HashMap<u64, Vec<u64>> = HashMap::new();

    for tile in tiles {
        for edge_value in tile.edge_values() {
            result.entry(edge_value).or_insert(vec![]).push(tile.id);
        }
    }

    result
}

fn find_corner_tile_ids(data: &[&str]) -> Result<HashSet<u64>, ParseIntError> {
    let tiles = parse_tiles(&data)?;
    let edge_values = collate_edge_values(&tiles);

    let mut id_counts: HashMap<u64, u64> = HashMap::new();

    for id in edge_values
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v)
        .flatten()
    {
        let count = id_counts.entry(*id).or_insert(0);
        *count += 1;
    }

    let mut result: HashSet<u64> = HashSet::new();

    for id in id_counts
        .iter()
        .filter_map(|(k, v)| if *v == 4 { Some(*k) } else { None })
    {
        result.insert(id);
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use std::collections::{HashMap, HashSet};

    use super::*;

    fn test_data() -> Vec<&'static str> {
        vec![
            "Tile 2311:",
            "..##.#..#.",
            "##..#.....",
            "#...##..#.",
            "####.#...#",
            "##.##.###.",
            "##...#.###",
            ".#.#.#..##",
            "..#....#..",
            "###...#.#.",
            "..###..###",
            "",
            "Tile 1951:",
            "#.##...##.",
            "#.####...#",
            ".....#..##",
            "#...######",
            ".##.#....#",
            ".###.#####",
            "###.##.##.",
            ".###....#.",
            "..#.#..#.#",
            "#...##.#..",
            "",
            "Tile 1171:",
            "####...##.",
            "#..##.#..#",
            "##.#..#.#.",
            ".###.####.",
            "..###.####",
            ".##....##.",
            ".#...####.",
            "#.##.####.",
            "####..#...",
            ".....##...",
            "",
            "Tile 1427:",
            "###.##.#..",
            ".#..#.##..",
            ".#.##.#..#",
            "#.#.#.##.#",
            "....#...##",
            "...##..##.",
            "...#.#####",
            ".#.####.#.",
            "..#..###.#",
            "..##.#..#.",
            "",
            "Tile 1489:",
            "##.#.#....",
            "..##...#..",
            ".##..##...",
            "..#...#...",
            "#####...#.",
            "#..#.#.#.#",
            "...#.#.#..",
            "##.#...##.",
            "..##.##.##",
            "###.##.#..",
            "",
            "Tile 2473:",
            "#....####.",
            "#..#.##...",
            "#.##..#...",
            "######.#.#",
            ".#...#.#.#",
            ".#########",
            ".###.#..#.",
            "########.#",
            "##...##.#.",
            "..###.#.#.",
            "",
            "Tile 2971:",
            "..#.#....#",
            "#...###...",
            "#.#.###...",
            "##.##..#..",
            ".#####..##",
            ".#..####.#",
            "#..#.#..#.",
            "..####.###",
            "..#.#.###.",
            "...#.#.#.#",
            "",
            "Tile 2729:",
            "...#.#.#.#",
            "####.#....",
            "..#.#.....",
            "....#..#.#",
            ".##..##.#.",
            ".#.####...",
            "####.#.#..",
            "##.####...",
            "##..#.##..",
            "#.##...##.",
            "",
            "Tile 3079:",
            "#.#.#####.",
            ".#..######",
            "..#.......",
            "######....",
            "####.#..#.",
            ".#...#.##.",
            "#.#####.##",
            "..#.###...",
            "..#.......",
            "..#.###...",
        ]
    }

    #[test]
    fn day_20_calculate_edge_value() {
        let edge = ".#...##.#.";
        let value = calculate_edge_value(edge);

        assert_eq!(282, value);
    }

    #[test]
    fn day_20_calculate_edge_values_of_tile() -> Result<(), ParseIntError> {
        let tile = vec![
            "Tile 1234:",
            "#.#.#####.",
            ".#..######",
            "..#.......",
            "######....",
            "####.#..#.",
            ".#...#.##.",
            "#.#####.##",
            "..#.###...",
            "..#.......",
            "..#.###...",
        ];
        let expected = vec![702, 501, 264, 66, 184, 116, 616, 89];

        let tile = Tile::parse(&tile)?;

        assert_eq!(expected, tile.edge_values());

        Ok(())
    }

    #[test]
    fn day_20_parse_tiles() -> Result<(), ParseIntError> {
        let data = test_data();
        let tiles = parse_tiles(&data)?;

        assert_eq!(9, tiles.len());

        Ok(())
    }

    #[test]
    fn day_20_parse_tiles_into_edge_values() -> Result<(), ParseIntError> {
        let expected = vec![
            vec![210, 300, 89, 616, 231, 924, 498, 318],  // 2311
            vec![710, 397, 498, 318, 564, 177, 841, 587], // 1951
            vec![966, 399, 288, 18, 24, 96, 902, 391],    // 1171
            vec![948, 183, 234, 348, 210, 300, 576, 9],   // 1427
            vec![848, 43, 18, 288, 948, 183, 565, 689],   // 1489
            vec![542, 481, 116, 184, 234, 348, 966, 399], // 2473
            vec![161, 532, 565, 689, 85, 680, 456, 78],   // 2971
            vec![85, 680, 576, 9, 710, 397, 271, 962],    // 2729
            vec![702, 501, 264, 66, 184, 116, 616, 89],   // 3079
        ];

        let data = test_data();
        let tiles = parse_tiles(&data)?;

        let edge_values: Vec<Vec<u64>> = tiles.iter().map(|t| t.edge_values()).collect();

        assert_eq!(expected, edge_values);

        Ok(())
    }

    fn expected_edge_collation() -> HashMap<u64, Vec<u64>> {
        let mut expected: HashMap<u64, Vec<u64>> = HashMap::new();
        expected.insert(210, vec![2311]);
        expected.insert(300, vec![2311]);
        expected.insert(89, vec![2311]);
        expected.insert(616, vec![2311]);
        expected.insert(231, vec![2311]);
        expected.insert(924, vec![2311]);
        expected.insert(498, vec![2311]);
        expected.insert(318, vec![2311]);
        expected.insert(710, vec![1951]);
        expected.insert(397, vec![1951]);
        expected.get_mut(&498).unwrap().push(1951);
        expected.get_mut(&318).unwrap().push(1951);
        expected.insert(564, vec![1951]);
        expected.insert(177, vec![1951]);
        expected.insert(841, vec![1951]);
        expected.insert(587, vec![1951]);
        expected.insert(966, vec![1171]);
        expected.insert(399, vec![1171]);
        expected.insert(288, vec![1171]);
        expected.insert(18, vec![1171]);
        expected.insert(24, vec![1171]);
        expected.insert(96, vec![1171]);
        expected.insert(902, vec![1171]);
        expected.insert(391, vec![1171]);
        expected.insert(948, vec![1427]);
        expected.insert(183, vec![1427]);
        expected.insert(234, vec![1427]);
        expected.insert(348, vec![1427]);
        expected.get_mut(&210).unwrap().push(1427);
        expected.get_mut(&300).unwrap().push(1427);
        expected.insert(576, vec![1427]);
        expected.insert(9, vec![1427]);
        expected.insert(848, vec![1489]);
        expected.insert(43, vec![1489]);
        expected.get_mut(&18).unwrap().push(1489);
        expected.get_mut(&288).unwrap().push(1489);
        expected.get_mut(&948).unwrap().push(1489);
        expected.get_mut(&183).unwrap().push(1489);
        expected.insert(565, vec![1489]);
        expected.insert(689, vec![1489]);
        expected.insert(542, vec![2473]);
        expected.insert(481, vec![2473]);
        expected.insert(116, vec![2473]);
        expected.insert(184, vec![2473]);
        expected.get_mut(&234).unwrap().push(2473);
        expected.get_mut(&348).unwrap().push(2473);
        expected.get_mut(&966).unwrap().push(2473);
        expected.get_mut(&399).unwrap().push(2473);
        expected.insert(161, vec![2971]);
        expected.insert(532, vec![2971]);
        expected.get_mut(&565).unwrap().push(2971);
        expected.get_mut(&689).unwrap().push(2971);
        expected.insert(85, vec![2971]);
        expected.insert(680, vec![2971]);
        expected.insert(456, vec![2971]);
        expected.insert(78, vec![2971]);
        expected.get_mut(&85).unwrap().push(2729);
        expected.get_mut(&680).unwrap().push(2729);
        expected.get_mut(&576).unwrap().push(2729);
        expected.get_mut(&9).unwrap().push(2729);
        expected.get_mut(&710).unwrap().push(2729);
        expected.get_mut(&397).unwrap().push(2729);
        expected.insert(271, vec![2729]);
        expected.insert(962, vec![2729]);
        expected.insert(702, vec![3079]);
        expected.insert(501, vec![3079]);
        expected.insert(264, vec![3079]);
        expected.insert(66, vec![3079]);
        expected.get_mut(&184).unwrap().push(3079);
        expected.get_mut(&116).unwrap().push(3079);
        expected.get_mut(&616).unwrap().push(3079);
        expected.get_mut(&89).unwrap().push(3079);

        expected
    }

    #[test]
    fn day_20_parse_tiles_into_edge_collation() -> Result<(), ParseIntError> {
        let expected = expected_edge_collation();

        let data = test_data();
        let tiles = parse_tiles(&data)?;
        let edge_values = collate_edge_values(&tiles);

        assert_eq!(expected, edge_values);

        Ok(())
    }

    #[test]
    fn day_20_finds_center_tile_id() -> Result<(), ParseIntError> {
        let data = test_data();
        let mut expected: HashSet<u64> = HashSet::new();
        expected.insert(1951);
        expected.insert(3079);
        expected.insert(2971);
        expected.insert(1171);

        assert_eq!(expected, find_corner_tile_ids(&data)?);

        Ok(())
    }
}

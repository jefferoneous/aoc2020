use std::collections::HashSet;

use super::*;

fn rotate_tiles(tiles: &mut Tiles) {
    tiles.iter_mut().for_each(|(_, tile)| tile.rotate_cw());
}

fn flip_tiles(tiles: &mut Tiles) {
    tiles
        .iter_mut()
        .for_each(|(_, tile)| tile.flip_top_to_bottom());
}

#[test]
fn day_20_reverse_bits() {
    assert_eq!(0b0001010011, reverse_bits(0b1100101000, 10));
}

#[test]
fn day_20_parse_image() {
    let data = vec![
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

    let expected = vec![
        0b1010111110,
        0b0100111111,
        0b0010000000,
        0b1111110000,
        0b1111010010,
        0b0100010110,
        0b1011111011,
        0b0010111000,
        0b0010000000,
        0b0010111000,
    ];

    assert_eq!(expected, parse_image(&data));
}

#[test]
fn day_20_calculates_edge_values_of_tile() -> Result<(), ParseIntError> {
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
    let expected = vec![
        0b1010111110,
        0b0111110101,
        0b0100001000,
        0b0001000010,
        0b0001110100,
        0b0010111000,
        0b0001011001,
        0b1001101000,
    ];

    let tile = Tile::parse(&tile)?;

    assert_eq!(&expected, &tile.edge_values);

    Ok(())
}

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
fn day_20_parses_tiles() -> Result<(), ParseIntError> {
    let data = test_data();
    let tiles = parse_tiles(&data)?;

    assert_eq!(9, tiles.len());

    Ok(())
}

#[test]
fn day_20_tiles_generate_correct_edge_values() -> Result<(), ParseIntError> {
    let mut expected: HashMap<u64, Vec<u64>> = HashMap::new();
    expected.insert(
        2311,
        vec![
            0b0011010010,
            0b0100101100,
            0b0001011001,
            0b1001101000,
            0b1110011100,
            0b0011100111,
            0b0100111110,
            0b0111110010,
        ],
    );
    expected.insert(
        1951,
        vec![
            0b1011000110,
            0b0110001101,
            0b0111110010,
            0b0100111110,
            0b0010110001,
            0b1000110100,
            0b1001001011,
            0b1101001001,
        ],
    );
    expected.insert(
        1171,
        vec![
            0b1111000110,
            0b0110001111,
            0b0100100000,
            0b0000010010,
            0b0001100000,
            0b0000011000,
            0b0110000111,
            0b1110000110,
        ],
    );
    expected.insert(
        1427,
        vec![
            0b1110110100,
            0b0010110111,
            0b0011101010,
            0b0101011100,
            0b0100101100,
            0b0011010010,
            0b0000001001,
            0b1001000000,
        ],
    );
    expected.insert(
        1489,
        vec![
            0b1101010000,
            0b0000101011,
            0b0000010010,
            0b0100100000,
            0b0010110111,
            0b1110110100,
            0b1010110001,
            0b1000110101,
        ],
    );
    expected.insert(
        2473,
        vec![
            0b1000011110,
            0b0111100001,
            0b0001110100,
            0b0010111000,
            0b0101011100,
            0b0011101010,
            0b0110001111,
            0b1111000110,
        ],
    );
    expected.insert(
        2971,
        vec![
            0b0010100001,
            0b1000010100,
            0b1000110101,
            0b1010110001,
            0b1010101000,
            0b0001010101,
            0b0001001110,
            0b0111001000,
        ],
    );
    expected.insert(
        2729,
        vec![
            0b0001010101,
            0b1010101000,
            0b1001000000,
            0b0000001001,
            0b0110001101,
            0b1011000110,
            0b1111000010,
            0b0100001111,
        ],
    );
    expected.insert(
        3079,
        vec![
            0b1010111110,
            0b0111110101,
            0b0100001000,
            0b0001000010,
            0b0001110100,
            0b0010111000,
            0b0001011001,
            0b1001101000,
        ],
    );

    let data = test_data();
    let tiles = parse_tiles(&data)?;

    for tile in tiles.values() {
        let expected_values = expected.get(&tile.id).cloned().unwrap();
        assert_eq!(expected_values, tile.edge_values);
    }

    Ok(())
}

#[test]
fn day_20_tile_rotates_cw() -> Result<(), ParseIntError> {
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
    let expected_image_data: Vec<u64> = vec![
        0b0001011001,
        0b0000111010,
        0b1111011101,
        0b0001011000,
        0b1011001011,
        0b1011111011,
        0b1011000011,
        0b0000100011,
        0b0001110011,
        0b0001000010,
    ];
    let mut tile = Tile::parse(&tile)?;

    tile.rotate_cw();

    assert_eq!(expected_image_data, tile.image_data);

    Ok(())
}

#[test]
fn day_20_tile_rotates_ccw() -> Result<(), ParseIntError> {
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
    let expected_image_data: Vec<u64> = vec![
        0b0100001000,
        0b1100111000,
        0b1100010000,
        0b1100001101,
        0b1101111101,
        0b1101001101,
        0b0001101000,
        0b1011101111,
        0b0101110000,
        0b1001101000,
    ];

    let mut tile = Tile::parse(&tile)?;

    tile.rotate_ccw();

    assert_eq!(expected_image_data, tile.image_data);

    Ok(())
}

#[test]
fn day_20_tile_rotates_180() -> Result<(), ParseIntError> {
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
    let expected_image_data: Vec<u64> = vec![
        0b0001110100,
        0b0000000100,
        0b0001110100,
        0b1101111101,
        0b0110100010,
        0b0100101111,
        0b0000111111,
        0b0000000100,
        0b1111110010,
        0b0111110101,
    ];

    let mut tile = Tile::parse(&tile)?;

    tile.rotate_180();

    assert_eq!(expected_image_data, tile.image_data);

    Ok(())
}

#[test]
fn day_20_tile_flips_right_to_left() -> Result<(), ParseIntError> {
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
    let expected_image_data: Vec<u64> = vec![
        0b0111110101,
        0b1111110010,
        0b0000000100,
        0b0000111111,
        0b0100101111,
        0b0110100010,
        0b1101111101,
        0b0001110100,
        0b0000000100,
        0b0001110100,
    ];

    let mut tile = Tile::parse(&tile)?;

    tile.flip_right_to_left();

    assert_eq!(expected_image_data, tile.image_data);

    Ok(())
}

#[test]
fn day_20_tile_flips_top_to_bottom() -> Result<(), ParseIntError> {
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
    let expected_image_data: Vec<u64> = vec![
        0b0010111000,
        0b0010000000,
        0b0010111000,
        0b1011111011,
        0b0100010110,
        0b1111010010,
        0b1111110000,
        0b0010000000,
        0b0100111111,
        0b1010111110,
    ];

    let mut tile = Tile::parse(&tile)?;
    tile.flip_top_to_bottom();

    assert_eq!(expected_image_data, tile.image_data);

    Ok(())
}

#[test]
fn day_20_finds_neighbor_ids() -> Result<(), ParseIntError> {
    let mut expected: HashMap<u64, Vec<u64>> = HashMap::new();
    expected.insert(1951, vec![2311, 2729]);
    expected.insert(3079, vec![2311, 2473]);
    expected.insert(2971, vec![1489, 2729]);
    expected.insert(1171, vec![2473, 1489]);
    expected.insert(2311, vec![1951, 3079, 1427]);
    expected.insert(2729, vec![1951, 1427, 2971]);
    expected.insert(2473, vec![3079, 1427, 1171]);
    expected.insert(1489, vec![1427, 2971, 1171]);
    expected.insert(1427, vec![2311, 2729, 2473, 1489]);

    let data = test_data();
    let tiles = parse_tiles(&data)?;
    let neighbor_ids = find_neighbor_ids(&tiles);

    for (id, ids) in neighbor_ids {
        let expected_ids = expected.get(&id).unwrap();
        assert!(
            expected_ids.iter().all(|i| ids.contains(i)),
            format!("\n\nExpected: {:?}\nActual: {:?}\n\n", expected_ids, ids)
        );
    }

    Ok(())
}

#[test]
fn day_20_finds_corner_tile_ids() -> Result<(), ParseIntError> {
    let mut expected: HashSet<u64> = HashSet::new();
    expected.insert(1951);
    expected.insert(3079);
    expected.insert(2971);
    expected.insert(1171);

    let data = test_data();
    let tiles = parse_tiles(&data)?;
    let arranged_tiles = arrange_tiles(tiles);

    assert_eq!(expected, find_corner_tile_ids(&arranged_tiles));

    Ok(())
}

#[test]
fn day_20_arranges_tiles() -> Result<(), ParseIntError> {
    let mut expected: HashMap<u64, Tile> = HashMap::new();
    expected.insert(
        1951,
        Tile {
            id: 1951,
            image_data: vec![],
            image_dimension: 0,
            edge_values: vec![],
            oriented: true,
            affixed: true,
            top: None,
            right: Some(2311),
            bottom: Some(2729),
            left: None,
        },
    );
    expected.insert(
        2311,
        Tile {
            id: 2311,
            image_data: vec![],
            image_dimension: 0,
            edge_values: vec![],
            oriented: true,
            affixed: true,
            top: None,
            right: Some(3079),
            bottom: Some(1427),
            left: Some(1951),
        },
    );
    expected.insert(
        3079,
        Tile {
            id: 3079,
            image_data: vec![],
            image_dimension: 0,
            edge_values: vec![],
            oriented: true,
            affixed: true,
            top: None,
            right: None,
            bottom: Some(2473),
            left: Some(2311),
        },
    );
    expected.insert(
        2729,
        Tile {
            id: 2729,
            image_data: vec![],
            image_dimension: 0,
            edge_values: vec![],
            oriented: true,
            affixed: true,
            top: Some(1951),
            right: Some(1427),
            bottom: Some(2971),
            left: None,
        },
    );
    expected.insert(
        1427,
        Tile {
            id: 1427,
            image_data: vec![],
            image_dimension: 0,
            edge_values: vec![],
            oriented: true,
            affixed: true,
            top: Some(2311),
            right: Some(2473),
            bottom: Some(1489),
            left: Some(2729),
        },
    );
    expected.insert(
        2473,
        Tile {
            id: 2473,
            image_data: vec![],
            image_dimension: 0,
            edge_values: vec![],
            oriented: true,
            affixed: true,
            top: Some(3079),
            right: None,
            bottom: Some(1171),
            left: Some(1427),
        },
    );
    expected.insert(
        2971,
        Tile {
            id: 2971,
            image_data: vec![],
            image_dimension: 0,
            edge_values: vec![],
            oriented: true,
            affixed: true,
            top: Some(2729),
            right: Some(1489),
            bottom: None,
            left: None,
        },
    );
    expected.insert(
        1489,
        Tile {
            id: 1489,
            image_data: vec![],
            image_dimension: 0,
            edge_values: vec![],
            oriented: true,
            affixed: true,
            top: Some(1427),
            right: Some(1171),
            bottom: None,
            left: Some(2971),
        },
    );
    expected.insert(
        1171,
        Tile {
            id: 1171,
            image_data: vec![],
            image_dimension: 0,
            edge_values: vec![],
            oriented: true,
            affixed: true,
            top: Some(2473),
            right: None,
            bottom: None,
            left: Some(1489),
        },
    );

    let data = test_data();
    let tiles = parse_tiles(&data)?;
    let mut arranged_tiles = arrange_tiles(tiles);

    for _ in 0..8 {
        let tile1951 = arranged_tiles.get(&1951).cloned().unwrap();

        if tile1951.right == Some(2311) && tile1951.bottom == Some(2729) {
            break;
        }

        match (tile1951.right, tile1951.bottom) {
            (Some(2729), Some(2311)) => flip_tiles(&mut arranged_tiles),
            _ => rotate_tiles(&mut arranged_tiles),
        };
    }

    for (id, expected_tile) in expected {
        let actual_tile = arranged_tiles.get(&id).cloned().unwrap();
        assert!(
            expected_tile.top == actual_tile.top
                && expected_tile.right == actual_tile.right
                && expected_tile.bottom == actual_tile.bottom
                && expected_tile.left == actual_tile.left
                && actual_tile.oriented
                && actual_tile.affixed,
            format!(
                "\nExpected: {:?}\n\nActual: {:?}",
                expected_tile, actual_tile
            )
        );
    }

    Ok(())
}

#[test]
fn day_20_finds_correct_water_roughness() -> Result<(), ParseIntError> {
    let data = test_data();
    let tiles = parse_tiles(&data)?;
    let arranged_tiles = arrange_tiles(tiles);
    let composite_image = compose_image(&arranged_tiles);
    let sea_monsters_removed = remove_sea_monsters(&composite_image);
    let roughness = calculate_roughness(&sea_monsters_removed);

    assert_eq!(273, roughness);

    Ok(())
}

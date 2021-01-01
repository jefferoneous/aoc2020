mod grid3d;
mod grid4d;

pub fn part_one(data: &[&str]) {
    use self::grid3d::Grid3d;

    let data = vec![data];
    let mut grid = Grid3d::new(&data);

    grid.run(6);

    println!("Active cubes after 6 cycles: {}", grid.active_cube_count());
}

pub fn part_two(data: &[&str]) {
    use self::grid4d::Grid4d;

    let data = vec![&data[..]];
    let data = vec![&data[..]];
    let mut grid = Grid4d::new(&data);

    grid.run(6);

    println!("Active cubes after 6 cycles: {}", grid.active_cube_count());
}

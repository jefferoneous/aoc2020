use std::ops::{Add, Sub};
use std::str::FromStr;

use super::DayRunner;

pub fn runner(data: Vec<String>) -> DayRunner {
    DayRunner::new(data, Some(part_one), Some(part_two))
}

fn part_one(data: &[String]) {
    let mut ship = Ship::new();
    navigate(&mut ship, &data);
    let mdist = manhattan_distance(ship.position);
    println!("Manhattan Distance: {}", mdist);
}

fn part_two(data: &[String]) {
    let mut ship = Ship::new();
    ship.use_waypoint = true;
    navigate(&mut ship, &data);
    let mdist = manhattan_distance(ship.position);
    println!("Manhattan Distance: {}", mdist);
}

fn navigate(ship: &mut Ship, data: &[String]) {
    for command in data {
        ship.follow_command(command);
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    North = 0,
    East = 90,
    South = 180,
    West = 270,
}

impl From<u32> for Direction {
    fn from(val: u32) -> Self {
        match val % 360 {
            0 => Self::North,
            90 => Self::East,
            180 => Self::South,
            270 => Self::West,
            _ => unreachable!(),
        }
    }
}

impl Add<u32> for Direction {
    type Output = Self;

    fn add(self, rhs: u32) -> Self::Output {
        (self as u32 + rhs).into()
    }
}

impl Sub<u32> for Direction {
    type Output = Self;

    fn sub(self, rhs: u32) -> Self::Output {
        (self as u32 + 360 - rhs).into()
    }
}

enum RotateDir {
    Right,
    Left,
}

enum TravelSpec {
    Directed(Direction, u32),
    Turn(RotateDir, u32),
    Forward(u32),
}

impl FromStr for TravelSpec {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        if let Some(first) = chars.next() {
            match first {
                'N' => Ok(TravelSpec::Directed(
                    Direction::North,
                    chars.collect::<String>().parse().unwrap_or_default(),
                )),
                'E' => Ok(TravelSpec::Directed(
                    Direction::East,
                    chars.collect::<String>().parse().unwrap_or_default(),
                )),
                'S' => Ok(TravelSpec::Directed(
                    Direction::South,
                    chars.collect::<String>().parse().unwrap_or_default(),
                )),
                'W' => Ok(TravelSpec::Directed(
                    Direction::West,
                    chars.collect::<String>().parse().unwrap_or_default(),
                )),
                'F' => Ok(TravelSpec::Forward(
                    chars.collect::<String>().parse().unwrap_or_default(),
                )),
                'R' => Ok(TravelSpec::Turn(
                    RotateDir::Right,
                    chars.collect::<String>().parse().unwrap_or_default(),
                )),
                'L' => Ok(TravelSpec::Turn(
                    RotateDir::Left,
                    chars.collect::<String>().parse().unwrap_or_default(),
                )),
                _ => Err("Unrecognized instruction".into()),
            }
        } else {
            Err("Something unexpected occurred".into())
        }
    }
}

type Position = (i32, i32);

struct Ship {
    direction: Direction,
    position: Position,
    use_waypoint: bool,
    waypoint_position: Position,
}

impl Ship {
    fn new() -> Self {
        Self {
            direction: Direction::East,
            position: (0, 0),
            use_waypoint: false,
            waypoint_position: (10, 1),
        }
    }

    fn travel(&mut self, spec: TravelSpec) {
        fn new_position(direction: &Direction, position: &Position, distance: u32) -> Position {
            let (lat, long) = *position;
            match direction {
                Direction::North => (lat, long + distance as i32),
                Direction::South => (lat, long - distance as i32),
                Direction::East => (lat + distance as i32, long),
                Direction::West => (lat - distance as i32, long),
            }
        }

        fn new_position_via_waypoint(
            waypoint_position: &Position,
            position: &Position,
            reps: u32,
        ) -> Position {
            (
                position.0 + waypoint_position.0 * (reps as i32),
                position.1 + waypoint_position.1 * (reps as i32),
            )
        }

        match spec {
            TravelSpec::Directed(dir, dist) => {
                if self.use_waypoint {
                    self.waypoint_position = new_position(&dir, &self.waypoint_position, dist);
                } else {
                    self.position = new_position(&dir, &self.position, dist);
                }
            }
            TravelSpec::Forward(amount) => {
                if self.use_waypoint {
                    self.position =
                        new_position_via_waypoint(&self.waypoint_position, &self.position, amount);
                } else {
                    self.position = new_position(&self.direction, &self.position, amount);
                }
            }
            TravelSpec::Turn(dir, angle) => match dir {
                RotateDir::Right => {
                    if self.use_waypoint {
                        match angle {
                            90 => {
                                self.waypoint_position =
                                    (self.waypoint_position.1, -self.waypoint_position.0);
                            }
                            180 => {
                                self.waypoint_position =
                                    (-self.waypoint_position.0, -self.waypoint_position.1);
                            }
                            270 => {
                                self.waypoint_position =
                                    (-self.waypoint_position.1, self.waypoint_position.0);
                            }
                            _ => {}
                        }
                    } else {
                        self.direction = self.direction + angle;
                    }
                }
                RotateDir::Left => {
                    if self.use_waypoint {
                        match angle {
                            90 => {
                                self.waypoint_position =
                                    (-self.waypoint_position.1, self.waypoint_position.0);
                            }
                            180 => {
                                self.waypoint_position =
                                    (-self.waypoint_position.0, -self.waypoint_position.1);
                            }
                            270 => {
                                self.waypoint_position =
                                    (self.waypoint_position.1, -self.waypoint_position.0);
                            }
                            _ => {}
                        }
                    } else {
                        self.direction = self.direction - angle;
                    }
                }
            },
        }
    }

    fn follow_command(&mut self, command: &str) {
        if let Ok(command) = command.parse::<TravelSpec>() {
            self.travel(command);
        }
    }
}

fn manhattan_distance(pos: Position) -> u32 {
    let (lat, long) = pos;
    lat.abs() as u32 + long.abs() as u32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day_12_moves_north() {
        let mut ship = Ship::new();
        ship.travel(TravelSpec::Directed(Direction::North, 10));
        assert_eq!((0, 10), ship.position);
    }

    #[test]
    fn day_12_moves_south() {
        let mut ship = Ship::new();
        ship.travel(TravelSpec::Directed(Direction::South, 10));
        assert_eq!((0, -10), ship.position);
    }

    #[test]
    fn day_12_moves_east() {
        let mut ship = Ship::new();
        ship.travel(TravelSpec::Directed(Direction::East, 10));
        assert_eq!((10, 0), ship.position);
    }

    #[test]
    fn day_12_moves_west() {
        let mut ship = Ship::new();
        ship.travel(TravelSpec::Directed(Direction::West, 10));
        assert_eq!((-10, 0), ship.position);
    }

    #[test]
    fn day_12_moves_forward() {
        let mut ship = Ship::new();
        ship.travel(TravelSpec::Forward(10));
        assert_eq!((10, 0), ship.position);
    }

    #[test]
    fn day_12_turns_right() {
        let mut ship = Ship::new();
        ship.travel(TravelSpec::Turn(RotateDir::Right, 90));
        assert_eq!(Direction::South, ship.direction);
        ship.travel(TravelSpec::Turn(RotateDir::Right, 90));
        assert_eq!(Direction::West, ship.direction);
        ship.travel(TravelSpec::Turn(RotateDir::Right, 90));
        assert_eq!(Direction::North, ship.direction);

        let mut ship = Ship::new();
        ship.travel(TravelSpec::Turn(RotateDir::Right, 180));
        assert_eq!(Direction::West, ship.direction);

        let mut ship = Ship::new();
        ship.travel(TravelSpec::Turn(RotateDir::Right, 270));
        assert_eq!(Direction::North, ship.direction);
        ship.travel(TravelSpec::Turn(RotateDir::Right, 270));
        assert_eq!(Direction::West, ship.direction);
        ship.travel(TravelSpec::Turn(RotateDir::Right, 270));
        assert_eq!(Direction::South, ship.direction);
    }

    #[test]
    fn day_12_turns_left() {
        let mut ship = Ship::new();
        ship.travel(TravelSpec::Turn(RotateDir::Left, 90));
        assert_eq!(Direction::North, ship.direction);
        ship.travel(TravelSpec::Turn(RotateDir::Left, 90));
        assert_eq!(Direction::West, ship.direction);
        ship.travel(TravelSpec::Turn(RotateDir::Left, 90));
        assert_eq!(Direction::South, ship.direction);

        let mut ship = Ship::new();
        ship.travel(TravelSpec::Turn(RotateDir::Left, 180));
        assert_eq!(Direction::West, ship.direction);

        let mut ship = Ship::new();
        ship.travel(TravelSpec::Turn(RotateDir::Left, 270));
        assert_eq!(Direction::South, ship.direction);
        ship.travel(TravelSpec::Turn(RotateDir::Left, 270));
        assert_eq!(Direction::West, ship.direction);
        ship.travel(TravelSpec::Turn(RotateDir::Left, 270));
        assert_eq!(Direction::North, ship.direction);
    }

    #[test]
    fn day_12_follows_commands() {
        let mut ship = Ship::new();
        ship.follow_command("F10");
        assert_eq!((10, 0), ship.position);
        ship.follow_command("N10");
        assert_eq!((10, 10), ship.position);
        ship.follow_command("R90");
        ship.follow_command("F5");
        assert_eq!((10, 5), ship.position);
    }

    #[test]
    fn day_12_follows_commands_using_waypoint() {
        let mut ship = Ship::new();
        ship.use_waypoint = true;

        ship.follow_command("F2");
        assert_eq!((20, 2), ship.position);
        ship.follow_command("W5");
        ship.follow_command("F1");
        assert_eq!((25, 3), ship.position);
        ship.follow_command("R90");
        ship.follow_command("F1");
        assert_eq!((26, -2), ship.position);
    }
}

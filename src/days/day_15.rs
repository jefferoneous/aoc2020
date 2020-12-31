use std::collections::HashMap;

pub fn part_one(data: &[&str]) {
    let list = parse_input(data[0]);
    let result = calculate_nth_number(&list, 2020);

    println!("The 2020th number: {}", result);
}

pub fn part_two(data: &[&str]) {
    let list = parse_input(data[0]);
    let result = calculate_nth_number(&list, 30000000);

    println!("The 30,000,000th number: {}", result);
}

fn parse_input(data: &str) -> Vec<u32> {
    data.split(",").filter_map(|s| s.parse().ok()).collect()
}

#[cfg(not(debug_assertions))]
const BOUNDS: u32 = 3 * 512 * 1024;

#[cfg(debug_assertions)]
const BOUNDS: u32 = 512 * 1024;

struct NumberRecorder {
    numbers_low: [u32; BOUNDS as usize],
    numbers_high: HashMap<u32, u32>,
}

impl NumberRecorder {
    fn new() -> Self {
        Self {
            numbers_low: [0; BOUNDS as usize],
            numbers_high: HashMap::with_capacity(1024 * 1024),
        }
    }

    fn record_number(&mut self, number: u32, turn: u32) {
        if number < BOUNDS {
            self.numbers_low[number as usize] = turn;
        } else {
            if let Some(v) = self.numbers_high.get_mut(&number) {
                *v = turn;
            } else {
                self.numbers_high.insert(number, turn);
            }
        }
    }

    fn get_number_turn(&self, number: u32) -> Option<u32> {
        if number < BOUNDS {
            let turn = self.numbers_low[number as usize];
            if turn != 0 {
                return Some(turn);
            } else {
                return None;
            }
        } else {
            return self.numbers_high.get(&number).copied();
        }
    }
}

fn calculate_nth_number(list: &[u32], n: u32) -> u32 {
    if (n as usize) < list.len() {
        return list[n as usize];
    }

    let mut recorder = NumberRecorder::new();

    for (turn, number) in list[..list.len() - 1].iter().enumerate() {
        recorder.record_number(*number, (turn + 1) as u32);
    }

    let mut current_number = list.last().copied().unwrap();

    for turn in list.len() as u32.. {
        if turn == n {
            return current_number;
        }

        let v = recorder.get_number_turn(current_number);
        recorder.record_number(current_number, turn as u32);

        if v.is_some() {
            current_number = turn as u32 - v.unwrap();
        } else {
            current_number = 0;
        }
    }

    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day_15_calculate_nth_number_01() {
        let list = parse_input("0,3,6");
        let result = calculate_nth_number(&list, 2020);
        assert_eq!(436, result);
    }

    #[test]
    fn day_15_calculate_nth_number_02() {
        let list = parse_input("1,3,2");
        let result = calculate_nth_number(&list, 2020);
        assert_eq!(1, result);
    }

    #[test]
    fn day_15_calculate_nth_number_03() {
        let list = parse_input("2,1,3");
        let result = calculate_nth_number(&list, 2020);
        assert_eq!(10, result);
    }

    #[test]
    fn day_15_calculate_nth_number_04() {
        let list = parse_input("1,2,3");
        let result = calculate_nth_number(&list, 2020);
        assert_eq!(27, result);
    }

    #[test]
    fn day_15_calculate_nth_number_05() {
        let list = parse_input("2,3,1");
        let result = calculate_nth_number(&list, 2020);
        assert_eq!(78, result);
    }

    #[test]
    fn day_15_calculate_nth_number_06() {
        let list = parse_input("3,2,1");
        let result = calculate_nth_number(&list, 2020);
        assert_eq!(438, result);
    }

    #[test]
    fn day_15_calculate_nth_number_07() {
        let list = parse_input("3,1,2");
        let result = calculate_nth_number(&list, 2020);
        assert_eq!(1836, result);
    }
}

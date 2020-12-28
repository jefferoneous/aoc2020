use super::DayRunner;

pub fn runner(data: Vec<String>) -> DayRunner {
    DayRunner::new(data, Some(part_one), Some(part_two))
}

fn part_one(data: &[String]) {
    let (timestamp, bus_list) = parse_input(&data);
    let (id, diff) = find_earliest_bus_and_time_diff(timestamp, &bus_list);

    println!(
        "Bus ID: {}\nTime diff: {}\nProduct: {}",
        id,
        diff,
        id * diff
    );
}

fn part_two(data: &[String]) {
    let (_, raw_bus_list) = parse_input(&data);
    let bus_list = parse_bus_list(&raw_bus_list);
    let timestamp = find_earliest_sequential_departure_timestamp(&bus_list);

    println!("Timestamp: {}", timestamp);
}

fn parse_input(data: &[String]) -> (usize, Vec<String>) {
    let timestamp: usize = data[0].parse().unwrap_or_default();
    let list: Vec<String> = data[1].split(',').map(|s| s.to_string()).collect();
    (timestamp, list)
}

fn parse_bus_list(data: &[String]) -> Vec<(usize, usize)> {
    data.iter()
        .enumerate()
        .filter_map(|(i, s)| {
            if let Ok(id) = s.parse::<usize>() {
                Some((i, id))
            } else {
                None
            }
        })
        .collect()
}

fn find_earliest_bus_and_time_diff(timestamp: usize, bus_list: &[String]) -> (usize, usize) {
    bus_list
        .iter()
        .filter_map(|s| s.parse::<usize>().ok())
        .map(|id| (id, id - timestamp % id))
        .min_by(|x, y| (x.1).cmp(&y.1))
        .unwrap_or_default()
}

fn find_earliest_sequential_departure_timestamp(list: &[(usize, usize)]) -> usize {
    let (result, _) = list.iter().fold((0, 1), |(mut timestamp, inc), (i, id)| {
        while (timestamp + i) % id != 0 {
            timestamp += inc
        }
        (timestamp, inc * id)
    });

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day_13_parse_input() {
        let data: Vec<String> = "939\n7,13,x,x,59,x,31,19"
            .lines()
            .map(|s| s.to_string())
            .collect();

        let expected_bus_list: Vec<String> = vec!["7", "13", "x", "x", "59", "x", "31", "19"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        assert_eq!((939, expected_bus_list), parse_input(&data));
    }

    #[test]
    fn day_13_find_earliest_bus_and_time() {
        let timestamp = 939;
        let bus_list: Vec<String> = vec!["7", "13", "x", "x", "59", "x", "31", "19"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        assert_eq!(
            (59, 5),
            find_earliest_bus_and_time_diff(timestamp, &bus_list)
        );
    }

    #[test]
    fn day_13_earliest_sequential_departure_01() {
        let raw_bus_list: Vec<String> = vec!["17", "x", "13", "19"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let bus_list = parse_bus_list(&raw_bus_list);

        assert_eq!(
            3417,
            find_earliest_sequential_departure_timestamp(&bus_list)
        );
    }

    #[test]
    fn day_13_earliest_sequential_departure_02() {
        let raw_bus_list: Vec<String> = vec!["67", "7", "59", "61"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let bus_list = parse_bus_list(&raw_bus_list);

        assert_eq!(
            754018,
            find_earliest_sequential_departure_timestamp(&bus_list)
        );
    }

    #[test]
    fn day_13_earliest_sequential_departure_03() {
        let raw_bus_list: Vec<String> = vec!["67", "x", "7", "59", "61"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let bus_list = parse_bus_list(&raw_bus_list);

        assert_eq!(
            779210,
            find_earliest_sequential_departure_timestamp(&bus_list)
        );
    }

    #[test]
    fn day_13_earliest_sequential_departure_04() {
        let raw_bus_list: Vec<String> = vec!["67", "7", "x", "59", "61"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let bus_list = parse_bus_list(&raw_bus_list);

        assert_eq!(
            1261476,
            find_earliest_sequential_departure_timestamp(&bus_list)
        );
    }

    #[test]
    fn day_13_earliest_sequential_departure_05() {
        let raw_bus_list: Vec<String> = vec!["1789", "37", "47", "1889"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let bus_list = parse_bus_list(&raw_bus_list);

        assert_eq!(
            1202161486,
            find_earliest_sequential_departure_timestamp(&bus_list)
        );
    }

    #[test]
    fn day_13_earliest_sequential_departure_06() {
        let data = vec!["0".to_string(), "17,x,x,x,x,x,x,x,x,x,x,37,x,x,x,x,x,409,x,29,x,x,x,x,x,x,x,x,x,x,13,x,x,x,x,x,x,x,x,x,23,x,x,x,x,x,x,x,373,x,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,19".to_string()];
        let (_, bus_list) = parse_input(&data);
        let bus_list = parse_bus_list(&bus_list);

        assert_eq!(
            1202161486,
            find_earliest_sequential_departure_timestamp(&bus_list)
        );
    }
}

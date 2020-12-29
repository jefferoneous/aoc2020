pub fn part_one(data: &[&str]) {
    let (timestamp, bus_list) = parse_input(&data);
    let (id, diff) = find_earliest_bus_and_time_diff(timestamp, &bus_list);

    println!(
        "Bus ID: {}\nTime diff: {}\nProduct: {}",
        id,
        diff,
        id * diff
    );
}

pub fn part_two(data: &[&str]) {
    let (_, raw_bus_list) = parse_input(&data);
    let bus_list = parse_bus_list(&raw_bus_list);
    let timestamp = find_earliest_sequential_departure_timestamp(&bus_list);

    println!("Timestamp: {}", timestamp);
}

fn parse_input<'a>(data: &[&'a str]) -> (usize, Vec<&'a str>) {
    let timestamp: usize = data[0].parse().unwrap_or_default();
    let list: Vec<&str> = data[1].split(',').map(|s| s).collect();
    (timestamp, list)
}

fn parse_bus_list(data: &[&str]) -> Vec<(usize, usize)> {
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

fn find_earliest_bus_and_time_diff(timestamp: usize, bus_list: &[&str]) -> (usize, usize) {
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
        let data: Vec<&str> = "939\n7,13,x,x,59,x,31,19".lines().collect();

        let expected_bus_list = vec!["7", "13", "x", "x", "59", "x", "31", "19"];

        assert_eq!((939, expected_bus_list), parse_input(&data));
    }

    #[test]
    fn day_13_find_earliest_bus_and_time() {
        let timestamp = 939;
        let bus_list: Vec<&str> = vec!["7", "13", "x", "x", "59", "x", "31", "19"];

        assert_eq!(
            (59, 5),
            find_earliest_bus_and_time_diff(timestamp, &bus_list)
        );
    }

    #[test]
    fn day_13_earliest_sequential_departure_01() {
        let raw_bus_list: Vec<&str> = vec!["17", "x", "13", "19"];
        let bus_list = parse_bus_list(&raw_bus_list);

        assert_eq!(
            3417,
            find_earliest_sequential_departure_timestamp(&bus_list)
        );
    }

    #[test]
    fn day_13_earliest_sequential_departure_02() {
        let raw_bus_list: Vec<&str> = vec!["67", "7", "59", "61"];
        let bus_list = parse_bus_list(&raw_bus_list);

        assert_eq!(
            754018,
            find_earliest_sequential_departure_timestamp(&bus_list)
        );
    }

    #[test]
    fn day_13_earliest_sequential_departure_03() {
        let raw_bus_list: Vec<&str> = vec!["67", "x", "7", "59", "61"];
        let bus_list = parse_bus_list(&raw_bus_list);

        assert_eq!(
            779210,
            find_earliest_sequential_departure_timestamp(&bus_list)
        );
    }

    #[test]
    fn day_13_earliest_sequential_departure_04() {
        let raw_bus_list: Vec<&str> = vec!["67", "7", "x", "59", "61"];
        let bus_list = parse_bus_list(&raw_bus_list);

        assert_eq!(
            1261476,
            find_earliest_sequential_departure_timestamp(&bus_list)
        );
    }

    #[test]
    fn day_13_earliest_sequential_departure_05() {
        let raw_bus_list: Vec<&str> = vec!["1789", "37", "47", "1889"];
        let bus_list = parse_bus_list(&raw_bus_list);

        assert_eq!(
            1202161486,
            find_earliest_sequential_departure_timestamp(&bus_list)
        );
    }

    #[test]
    #[ignore]
    fn day_13_earliest_sequential_departure_06() {
        let data = vec!["0", "17,x,x,x,x,x,x,x,x,x,x,37,x,x,x,x,x,409,x,29,x,x,x,x,x,x,x,x,x,x,13,x,x,x,x,x,x,x,x,x,23,x,x,x,x,x,x,x,373,x,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,19"];
        let (_, bus_list) = parse_input(&data);
        let bus_list = parse_bus_list(&bus_list);

        assert_eq!(
            1202161486,
            find_earliest_sequential_departure_timestamp(&bus_list)
        );
    }
}

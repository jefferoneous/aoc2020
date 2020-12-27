use super::DayRunner;

pub fn runner(data: Vec<String>) -> DayRunner {
    DayRunner::new(data, Some(part_one), None)
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

fn parse_input(data: &[String]) -> (u32, Vec<String>) {
    let timestamp: u32 = data[0].parse().unwrap_or_default();
    let list: Vec<String> = data[1].split(',').map(|s| s.to_string()).collect();
    (timestamp, list)
}

fn find_earliest_bus_and_time_diff(timestamp: u32, bus_list: &[String]) -> (u32, u32) {
    bus_list
        .iter()
        .filter_map(|s| s.parse::<u32>().ok())
        .map(|id| (id, id - timestamp % id))
        .min_by(|x, y| (x.1).cmp(&y.1))
        .unwrap_or_default()
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
}

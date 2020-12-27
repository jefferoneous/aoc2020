use super::DayRunner;

fn find_weak_number(data: &[i64], window_size: usize) -> i64 {
    for numbers in data.windows(window_size + 1) {
        let target = numbers[window_size];
        if !numbers[0..window_size]
            .iter()
            .any(|&n| numbers.contains(&(target - n)))
        {
            return numbers[window_size];
        }
    }

    0
}

fn find_encryption_weakness(data: &[i64], window_size: usize) -> i64 {
    let weak_number = find_weak_number(data, window_size);

    let mut index: usize = 0;

    loop {
        let mut sum: i64 = 0;
        let mut smallest: i64 = data[index];
        let mut largest: i64 = smallest;

        for number in data[index..].iter() {
            if *number < smallest {
                smallest = *number;
            } else if *number > largest {
                largest = *number;
            }

            sum += *number;

            if sum == weak_number {
                return smallest + largest;
            }
            if sum > weak_number {
                break;
            }
        }

        index += 1;

        if index > data.len() {
            break;
        }
    }

    0
}

fn parse_numbers(data: &[String]) -> Vec<i64> {
    data.iter()
        .map(|s| s.parse::<i64>().unwrap_or_default())
        .collect()
}

pub fn part_one(data: &[String]) {
    let numeric_data = parse_numbers(data);
    let result = find_weak_number(&numeric_data, 25);
    println!("Weak number: {}", result);
}

pub fn part_two(data: &[String]) {
    let numeric_data = parse_numbers(data);
    let result = find_encryption_weakness(&numeric_data, 25);
    println!("Encryption weakness: {}", result);
}

pub fn runner(data: Vec<String>) -> DayRunner {
    DayRunner::new(data, Some(part_one), Some(part_two))
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_sample_data() -> Vec<String> {
        vec![
            "35".to_string(),
            "20".to_string(),
            "15".to_string(),
            "25".to_string(),
            "47".to_string(),
            "40".to_string(),
            "62".to_string(),
            "55".to_string(),
            "65".to_string(),
            "95".to_string(),
            "102".to_string(),
            "117".to_string(),
            "150".to_string(),
            "182".to_string(),
            "127".to_string(),
            "219".to_string(),
            "299".to_string(),
            "277".to_string(),
            "309".to_string(),
            "576".to_string(),
        ]
    }

    #[test]
    fn day_09_finds_weak_number() {
        let sample_data = get_sample_data();
        let data = parse_numbers(&sample_data);

        assert_eq!(find_weak_number(&data, 5), 127);
    }

    #[test]
    fn day_09_finds_encryption_weakness() {
        let sample_data = get_sample_data();
        let data = parse_numbers(&sample_data);

        assert_eq!(find_encryption_weakness(&data, 5), 62);
    }
}

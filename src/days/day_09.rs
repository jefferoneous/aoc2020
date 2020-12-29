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

fn parse_numbers(data: &[&str]) -> Vec<i64> {
    data.iter()
        .map(|s| s.parse::<i64>().unwrap_or_default())
        .collect()
}

pub fn part_one(data: &[&str]) {
    let numeric_data = parse_numbers(data);
    let result = find_weak_number(&numeric_data, 25);
    println!("Weak number: {}", result);
}

pub fn part_two(data: &[&str]) {
    let numeric_data = parse_numbers(data);
    let result = find_encryption_weakness(&numeric_data, 25);
    println!("Encryption weakness: {}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day_09_finds_weak_number() {
        let sample_data = vec![
            "35", "20", "15", "25", "47", "40", "62", "55", "65", "95", "102", "117", "150", "182",
            "127", "219", "299", "277", "309", "576",
        ];
        let data = parse_numbers(&sample_data);

        assert_eq!(find_weak_number(&data, 5), 127);
    }

    #[test]
    fn day_09_finds_encryption_weakness() {
        let sample_data = vec![
            "35", "20", "15", "25", "47", "40", "62", "55", "65", "95", "102", "117", "150", "182",
            "127", "219", "299", "277", "309", "576",
        ];
        let data = parse_numbers(&sample_data);

        assert_eq!(find_encryption_weakness(&data, 5), 62);
    }
}

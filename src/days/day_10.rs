use std::collections::HashMap;

use super::DayRunner;

pub fn runner(data: Vec<String>) -> DayRunner {
    DayRunner::new(data, Some(part_one), Some(part_two))
}

fn calculate_differences(data: &[u64]) -> Vec<u64> {
    let mut v = vec![0];
    v.extend_from_slice(data);
    v.sort();

    let max = data.iter().max().unwrap();
    v.push(*max + 3);

    v.windows(2).map(|w| w[1] - w[0]).collect()
}

fn calculate_distribution(diffs: &[u64]) -> HashMap<u64, u64> {
    let mut distribs = HashMap::new();
    distribs.insert(1, 0);
    distribs.insert(2, 0);
    distribs.insert(3, 0);

    for n in diffs {
        if let Some(count) = distribs.get_mut(n) {
            *count += 1;
        }
    }

    distribs
}

fn calculate_product(data: &[String]) -> u64 {
    let numeric_data = parse_list(data);
    let diffs = calculate_differences(&numeric_data);
    let distrib = calculate_distribution(&diffs);

    distrib.get(&1).unwrap() * distrib.get(&3).unwrap()
}

fn calculate_combinations(data: &[String]) -> u64 {
    let numeric_data = parse_list(data);
    let diffs = calculate_differences(&numeric_data);
    count_combinations(&diffs)
}

fn count_combinations(data: &[u64]) -> u64 {
    let mut count: u64 = 0;
    let mut result: u64 = 1;

    for n in data {
        if *n == 1 {
            count += 1;
        } else {
            if count > 1 {
                result *= 1 + count * (count - 1) / 2;
            }
            count = 0;
        }
    }

    result
}

fn parse_list(data: &[String]) -> Vec<u64> {
    data.iter()
        .map(|s| s.parse::<u64>().unwrap_or_default())
        .collect()
}

fn part_one(data: &[String]) {
    let product = calculate_product(&data);
    println!("Product: {:?}", product);
}

fn part_two(data: &[String]) {
    let combinations = calculate_combinations(&data);
    println!("Combinations: {:?}", combinations);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day_10_produces_correct_differences() {
        let sample_data = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

        let expected = vec![1, 3, 1, 1, 1, 3, 1, 1, 3, 1, 3, 3];
        let result = calculate_differences(&sample_data);

        assert_eq!(expected, result);
    }

    #[test]
    fn day_10_produces_correct_differences_larger_sample() {
        let sample_data = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];

        let expected = vec![
            1, 1, 1, 1, 3, 1, 1, 1, 1, 3, 3, 1, 1, 1, 3, 1, 1, 3, 3, 1, 1, 1, 1, 3, 1, 3, 3, 1, 1,
            1, 1, 3,
        ];
        let result = calculate_differences(&sample_data);

        assert_eq!(expected, result);
    }

    #[test]
    fn day_10_calculdate_distributions() {
        let sample_data = vec![1, 3, 1, 1, 1, 3, 1, 1, 3, 1, 3, 3];
        let mut expected = HashMap::new();
        expected.insert(1, 7);
        expected.insert(2, 0);
        expected.insert(3, 5);

        let distrib = calculate_distribution(&sample_data);

        assert_eq!(expected, distrib);
    }

    #[test]
    fn day_10_combinatorial_values() {
        let sample_data = vec![1, 3, 1, 1, 1, 3, 1, 1, 3, 1, 3, 3];
        let result = count_combinations(&sample_data);

        assert_eq!(8, result);
    }

    #[test]
    fn day_10_combinatorial_values_larger_sample() {
        let sample_data = vec![
            1, 1, 1, 1, 3, 1, 1, 1, 1, 3, 3, 1, 1, 1, 3, 1, 1, 3, 3, 1, 1, 1, 1, 3, 1, 3, 3, 1, 1,
            1, 1, 3,
        ];
        let result = count_combinations(&sample_data);

        assert_eq!(19208, result);
    }

    #[test]
    fn day_10_parses_list() {
        let sample_data = vec![
            "16".to_string(),
            "10".to_string(),
            "15".to_string(),
            "5".to_string(),
            "1".to_string(),
            "11".to_string(),
            "7".to_string(),
            "19".to_string(),
            "6".to_string(),
            "12".to_string(),
            "4".to_string(),
        ];
        let expected = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

        assert_eq!(expected, parse_list(&sample_data));
    }

    #[test]
    fn day_10_product_of_distribution() {
        let sample_data = vec![
            "16".to_string(),
            "10".to_string(),
            "15".to_string(),
            "5".to_string(),
            "1".to_string(),
            "11".to_string(),
            "7".to_string(),
            "19".to_string(),
            "6".to_string(),
            "12".to_string(),
            "4".to_string(),
        ];

        let product = calculate_product(&sample_data);

        assert_eq!(35, product);
    }
}
/*

! for each contiguous set of 1's, let n = the # of 1's in the set
! such that n > 1
! then the combinatorial value, v, of the set is
!  v = 1 + n(n - 1) / 2
!
! the comb val of each 3 is 1
!
! then, multiply the v of each set to get the combined value
(0), 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, (22)
(0), 1, 4, 5,  , 7, 10, 11, 12, 15, 16, 19, (22)
(0), 1, 4,  , 6, 7, 10, 11, 12, 15, 16, 19, (22)
(0), 1, 4,  ,  , 7, 10, 11, 12, 15, 16, 19, (22)
(0), 1, 4, 5, 6, 7, 10,   , 12, 15, 16, 19, (22)
(0), 1, 4, 5,  , 7, 10,   , 12, 15, 16, 19, (22)
(0), 1, 4,  , 6, 7, 10,   , 12, 15, 16, 19, (22)
(0), 1, 4,     , 7, 10,   , 12, 15, 16, 19, (22)

(0)  1  4  5  6  7  8  11  12  15  16  19  (22)
(0)  1  4  5  6     8  11  12  15  16  19  (22)
(0)  1  4  5     7  8  11  12  15  16  19  (22)
(0)  1  4  5        8  11  12  15  16  19  (22)
(0)  1  4     6  7  8  11  12  15  16  19  (22)
(0)  1  4     6     8  11  12  15  16  19  (22)
(0)  1  4        7  8  11  12  15  16  19  (22)

(0)  1  4  5  6  7  8  9  12  15  16  19  (22)
(0)  1  4  5  6  7     9  12  15  16  19  (22)
(0)  1  4  5  6     8  9  12  15  16  19  (22)
(0)  1  4  5  6        9  12  15  16  19  (22)
(0)  1  4  5     7  8  9  12  15  16  19  (22)
(0)  1  4  5     7     9  12  15  16  19  (22)
(0)  1  4  5        8  9  12  15  16  19  (22)
(0)  1  4     6  7     9  12  15  16  19  (22)
(0)  1  4     6     8  9  12  15  16  19  (22)
(0)  1  4     6        9  12  15  16  19  (22)
(0)  1  4        7     9  12  15  16  19  (22)

(0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34, 35, 38, 39, 42, 45, 46, 47, 48, 49, (52)
(0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34, 35, 38, 39, 42, 45, 46, 47, 49, (52)
(0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34, 35, 38, 39, 42, 45, 46, 48, 49, (52)
(0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34, 35, 38, 39, 42, 45, 46, 49, (52)
(0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34, 35, 38, 39, 42, 45, 47, 48, 49, (52)
(0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45, 46, 48, 49, (52)
(0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45, 46, 49, (52)
(0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45, 47, 48, 49, (52)
(0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45, 47, 49, (52)
(0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45, 48, 49, (52)
 */

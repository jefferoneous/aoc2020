use std::collections::HashMap;

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

fn calculate_product(data: &[&str]) -> u64 {
    let numeric_data = parse_list(data);
    let diffs = calculate_differences(&numeric_data);
    let distrib = calculate_distribution(&diffs);

    distrib.get(&1).unwrap() * distrib.get(&3).unwrap()
}

fn calculate_combinations(data: &[&str]) -> u64 {
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

fn parse_list(data: &[&str]) -> Vec<u64> {
    data.iter()
        .map(|s| s.parse::<u64>().unwrap_or_default())
        .collect()
}

pub fn part_one(data: &[&str]) {
    let product = calculate_product(&data);
    println!("Product: {:?}", product);
}

pub fn part_two(data: &[&str]) {
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
        let sample_data = vec!["16", "10", "15", "5", "1", "11", "7", "19", "6", "12", "4"];
        let expected = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

        assert_eq!(expected, parse_list(&sample_data));
    }

    #[test]
    fn day_10_product_of_distribution() {
        let sample_data = vec!["16", "10", "15", "5", "1", "11", "7", "19", "6", "12", "4"];

        let product = calculate_product(&sample_data);

        assert_eq!(35, product);
    }
}

/*
Within the set of differences, find all the sets of adjacent 1's,
where the number of 1's in the set is n and n > 1.
The number of combinations, c, in each such set is:

      c = 1 + n(n - 1) / 2

Multiply the c of all such sets to get the number of combinations
of all the adapters.

                         c = 1 + 3 (3 - 1) / 2
                           = 1 + 3 (2) / 2
                           = 1 + 6 / 2
                           = 1 + 3
        3 adj. 1's ────> c = 4
        ----------
(0)  1  4  5  6  7  10  (11)
(0)  1  4  5     7  10  (11)
(0)  1  4     6  7  10  (11)
(0)  1  4        7  10  (11)


                         c = 1 + 4 (4 - 1) / 2
                           = 1 + 4 (3) / 2
                           = 1 + 12 / 2
                           = 1 + 6
         4 adj. 1's ───> c = 7
        -------------
(0)  1  4  5  6  7  8  (11)
(0)  1  4  5  6     8  (11)
(0)  1  4  5     7  8  (11)
(0)  1  4  5        8  (11)
(0)  1  4     6  7  8  (11)
(0)  1  4     6     8  (11)
(0)  1  4        7  8  (11)

                         c = 1 + 5 (5 - 1) / 2
                           = 1 + 5 (4) / 2
                           = 1 + 20 / 2
                           = 1 + 10
          5 adj. 1's ──> c = 11
        ----------------
(0)  1  4  5  6  7  8  9  (12)
(0)  1  4  5  6  7     9  (12)
(0)  1  4  5  6     8  9  (12)
(0)  1  4  5  6        9  (12)
(0)  1  4  5     7  8  9  (12)
(0)  1  4  5     7     9  (12)
(0)  1  4  5        8  9  (12)
(0)  1  4     6  7     9  (12)
(0)  1  4     6     8  9  (12)
(0)  1  4     6        9  (12)
(0)  1  4        7     9  (12)

 */

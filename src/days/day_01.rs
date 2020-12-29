const SUM: u32 = 2020;

fn find_pair(list: &[u32], sum: u32) -> Option<(u32, u32)> {
    for x in list {
        if *x > sum {
            continue;
        }

        if list[1..].contains(&(sum - *x)) {
            return Some((*x, sum - x));
        }
    }

    None
}

fn find_triple(list: &[u32], sum: u32) -> Option<(u32, u32, u32)> {
    for x in list {
        if *x > sum {
            continue;
        }

        if let Some((y, z)) = find_pair(&list[1..], sum - *x) {
            return Some((*x, y, z));
        }
    }

    None
}

fn convert_strings_to_numbers(contents: &[&str]) -> Vec<u32> {
    let mut result: Vec<u32> = vec![];

    for line in contents {
        if let Ok(x) = line.parse() {
            result.push(x);
        }
    }

    result
}

pub fn part_one(data: &[&str]) {
    let list = convert_strings_to_numbers(data);
    if let Some((a, b)) = find_pair(&list, SUM) {
        println!("a: {}, b: {}, a*b: {}", a, b, a * b);
    } else {
        println!("No solution found for part one");
    }
}

pub fn part_two(data: &[&str]) {
    let list = convert_strings_to_numbers(data);
    if let Some((a, b, c)) = find_triple(&list, SUM) {
        println!("a: {}, b: {}, c: {}, a*b*c: {}", a, b, c, a * b * c);
    } else {
        println!("No solution found for part two");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day_01_test_finds_correct_pair() {
        let list = vec![1721, 979, 366, 299, 675, 1456];
        let result = find_pair(&list, 2020);

        assert_eq!(result, Some((1721, 299)));
    }

    #[test]
    fn day_01_test_finds_correct_triple() {
        let list = vec![1721, 979, 366, 299, 675, 1456];
        let result = find_triple(&list, 2020);

        assert_eq!(result, Some((979, 366, 675)));
    }
}

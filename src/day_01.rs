use std::{fs, io};

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

fn part_one(list: &[u32], sum: u32) {
    println!("Part One\n========");
    if let Some((a, b)) = find_pair(list, sum) {
        println!("a: {}, b: {}, a*b: {}", a, b, a * b);
    } else {
        println!("No solution found for part one");
    }
}

fn part_two(list: &[u32], sum: u32) {
    println!("Part Two\n========");
    if let Some((a, b, c)) = find_triple(list, sum) {
        println!("a: {}, b: {}, c: {}, a*b*c: {}", a, b, c, a * b * c);
    } else {
        println!("No solution found for part two");
    }
}

fn load_list_from_file(filename: &str) -> Result<Vec<u32>, io::Error> {
    let mut result: Vec<u32> = vec![];

    let contents = fs::read_to_string(filename)?;

    for line in contents.lines() {
        if let Ok(x) = line.parse() {
            result.push(x);
        }
    }

    Ok(result)
}

pub fn run(filename: &str) {
    match load_list_from_file(filename) {
        Ok(list) => {
            part_one(&list, 2020);
            part_two(&list, 2020);
        }
        Err(e) => eprintln!("Error occurred while reading input file: {}", e),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_finds_correct_pair() {
        let list = vec![1721, 979, 366, 299, 675, 1456];
        let result = find_pair(&list, 2020);

        assert_eq!(result, Some((1721, 299)));
    }

    #[test]
    fn test_finds_correct_triple() {
        let list = vec![1721, 979, 366, 299, 675, 1456];
        let result = find_triple(&list, 2020);

        assert_eq!(result, Some((979, 366, 675)));
    }
}

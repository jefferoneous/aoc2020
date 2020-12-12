use std::{fs, io};

struct DayOne {
    list: Vec<u32>,
}

impl DayOne {
    fn with_list(list: Vec<u32>) -> Self {
        Self { list }
    }

    fn from_file(filename: &str) -> Self {
        let list = Self::load_list_from_file(filename).unwrap();
        Self { list }
    }

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

            if let Some((y, z)) = Self::find_pair(&list[1..], sum - *x) {
                return Some((*x, y, z));
            }
        }

        None
    }

    fn load_list_from_file(filename: &str) -> Result<Vec<u32>, io::Error> {
        let mut result: Vec<u32> = vec![];

        let contents = fs::read_to_string(filename)?;

        for line in contents.lines() {
            result.push(line.parse().unwrap());
        }

        Ok(result)
    }

    fn part_one(&self, sum: u32) {
        println!("Part One\n========");
        if let Some((a, b)) = Self::find_pair(&self.list, sum) {
            println!("a: {}, b: {}, a*b: {}", a, b, a * b);
        } else {
            println!("No solution found for part one");
        }
    }

    fn part_two(&self, sum: u32) {
        println!("Part Two\n========");
        if let Some((a, b, c)) = Self::find_triple(&self.list, sum) {
            println!("a: {}, b: {}, c: {}, a*b*c: {}", a, b, c, a * b * c);
        } else {
            println!("No solution found for part two");
        }
    }
}

pub fn run(filename: &str) {
    let day_one = DayOne::from_file(filename.into());

    day_one.part_one(2020);
    day_one.part_two(2020);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_finds_correct_pair() {
        let list = vec![1721, 979, 366, 299, 675, 1456];
        let (a, b) = DayOne::find_pair(&list, 2020).unwrap();

        assert_eq!(a, 1721);
        assert_eq!(b, 299);
    }

    #[test]
    fn test_finds_correct_triple() {
        let list = vec![1721, 979, 366, 299, 675, 1456];
        let (a, b, c) = DayOne::find_triple(&list, 2020).unwrap();

        assert_eq!(a, 979);
        assert_eq!(b, 366);
        assert_eq!(c, 675);
    }
}

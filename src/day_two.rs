use io::BufReader;
use std::{
    fs::File,
    io::{self, BufRead},
    str::FromStr,
};

struct PasswordSpec {
    min_count: u8,
    max_count: u8,
    character: char,
    password: String,
}

impl FromStr for PasswordSpec {
    type Err = ();

    fn from_str(spec: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = spec.split(|c| "- :".contains(c)).collect();
        let min_count = parts[0].parse().unwrap_or(1);
        let max_count = parts[1].parse().unwrap_or(u8::MAX);
        let character = parts[2].chars().next().unwrap_or('\x1B');
        // note we skip index 3 because it should be an empty string
        let password = parts[4].into();

        Ok(Self {
            min_count,
            max_count,
            character,
            password,
        })
    }
}

impl PasswordSpec {
    fn has_valid_password(&self) -> bool {
        let count = self
            .password
            .chars()
            .filter(|&c| c == self.character)
            .count() as u8;
        count >= self.min_count && count <= self.max_count
    }
}

fn count_invalid_passwords(list: &Vec<String>) -> usize {
    let mut result = 0;

    for item in list {
        let spec: PasswordSpec = item.parse().unwrap();
        if spec.has_valid_password() {
            result += 1;
        }
    }

    result
}

fn part_one(list: &Vec<String>) {
    println!("Part One\n========");
    let count = count_invalid_passwords(list);
    println!("Invalid passwords: {}", count);
}

// fn part_two(list: &[u32], sum: u32) {
//     println!("Part Two\n========");
//     if let Some((a, b, c)) = find_triple(list, sum) {
//         println!("a: {}, b: {}, c: {}, a*b*c: {}", a, b, c, a * b * c);
//     } else {
//         println!("No solution found for part two");
//     }
// }

fn load_list_from_file(filename: &str) -> Result<Vec<String>, io::Error> {
    let input = File::open(filename)?;
    let buf = BufReader::new(input);
    let result = buf.lines().map(|l| l.unwrap()).collect();

    Ok(result)
}

pub fn run(filename: &str) {
    match load_list_from_file(filename) {
        Ok(list) => {
            part_one(&list);
            // part_two(&list, 2020);
        }
        Err(e) => println!("Error occurred while reading input file: {}", e),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_spec() {
        let spec = PasswordSpec::from_str("1-3 a: abcde").unwrap();

        assert_eq!(spec.min_count, 1);
        assert_eq!(spec.max_count, 3);
        assert_eq!(spec.character, 'a');
        assert_eq!(spec.password, "abcde");
    }

    #[test]
    fn identifies_valid_password() {
        let spec = PasswordSpec::from_str("1-3 a: abcde").unwrap();

        assert!(spec.has_valid_password());
    }

    #[test]
    fn identifies_invalid_password() {
        let spec = PasswordSpec::from_str("1-3 b: cdefg").unwrap();

        assert_eq!(false, spec.has_valid_password());
    }
}

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs::File;
use std::io::{BufRead, BufReader, Error as IoError};
use std::str::FromStr;

#[derive(Debug)]
struct PasswordSpec {
    min_count: u8,
    max_count: u8,
    character: char,
    password: String,
}

#[derive(Debug, PartialEq, Eq)]
enum ParseError {
    InvalidSyntax,
    MinCountTooLow,
    MaxCountLowerThanMinCount,
    MissingRequiredCharacterSpec,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                ParseError::InvalidSyntax => "Specification doesn't follow syntax",
                ParseError::MinCountTooLow => "Minimum count can't be less than 1",
                ParseError::MaxCountLowerThanMinCount =>
                    "Maximum count can't be less than minimum count",
                ParseError::MissingRequiredCharacterSpec =>
                    "Required character specification is missing",
            }
        )
    }
}

impl FromStr for PasswordSpec {
    type Err = ParseError;

    fn from_str(spec: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = spec.split(|c| "- :".contains(c)).collect();

        if parts.len() != 5 {
            return Err(ParseError::InvalidSyntax);
        }

        let min_count = parts[0].parse().unwrap_or_default();
        let max_count = parts[1].parse().unwrap_or_default();
        let character = parts[2].chars().next().unwrap_or_default();
        // note we skip index 3 because it should be an empty string
        let password = parts[4].into();

        if min_count < 1 {
            return Err(ParseError::MinCountTooLow);
        }

        if max_count < min_count {
            return Err(ParseError::MaxCountLowerThanMinCount);
        }

        if character == char::default() {
            return Err(ParseError::MissingRequiredCharacterSpec);
        }

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
    let mut count = 0;

    for item in list {
        match item.parse::<PasswordSpec>() {
            Ok(spec) => {
                if spec.has_valid_password() {
                    count += 1;
                }
            }
            Err(e) => eprintln!("Could not parse \"{}\": {}", item, e),
        }
    }

    count
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

fn load_list_from_file(filename: &str) -> Result<Vec<String>, IoError> {
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
        Err(e) => eprintln!("Error occurred while reading input file: {}", e),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_valid_spec() {
        let spec: PasswordSpec = "1-3 a: abcde".parse().unwrap();

        assert_eq!(spec.min_count, 1);
        assert_eq!(spec.max_count, 3);
        assert_eq!(spec.character, 'a');
        assert_eq!(spec.password, "abcde");
    }

    #[test]
    fn cannot_parse_spec_out_of_syntax() {
        let result = "0-: abcde".parse::<PasswordSpec>().err();

        assert_eq!(result, Some(ParseError::InvalidSyntax));
    }

    #[test]
    fn cannot_parse_spec_with_min_less_than_one() {
        let result = "0-4 a: abcde".parse::<PasswordSpec>().err();

        assert_eq!(result, Some(ParseError::MinCountTooLow));
    }

    #[test]
    fn cannot_parse_spec_with_max_less_than_min() {
        let result = "4-1 a: abcde".parse::<PasswordSpec>().err();

        assert_eq!(result, Some(ParseError::MaxCountLowerThanMinCount));
    }

    #[test]
    fn cannot_parse_spec_with_missing_required_char() {
        let result = "1-3 : abcde".parse::<PasswordSpec>().err();

        assert_eq!(result, Some(ParseError::MissingRequiredCharacterSpec));
    }

    #[test]
    fn identifies_valid_password() {
        let spec: PasswordSpec = "1-3 a: abcde".parse().unwrap();

        assert!(spec.has_valid_password());
    }

    #[test]
    fn identifies_invalid_password() {
        let spec: PasswordSpec = "1-3 b: cdefg".parse().unwrap();

        assert_eq!(false, spec.has_valid_password());
    }
}

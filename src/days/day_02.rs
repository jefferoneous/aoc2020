use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

use super::DayRunner;

#[derive(Debug)]
struct PasswordSpec {
    a: u8,
    b: u8,
    character: char,
    password: String,
}

#[derive(Debug, PartialEq, Eq)]
enum ParseError {
    InvalidSyntax,
    ATooLow,
    BLowerThanA,
    MissingRequiredCharacterSpec,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                ParseError::InvalidSyntax => "Specification doesn't follow syntax",
                ParseError::ATooLow => "A can't be less than 1",
                ParseError::BLowerThanA => "B can't be less than A",
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

        let a = parts[0].parse().unwrap_or_default();
        let b = parts[1].parse().unwrap_or_default();
        let character = parts[2].chars().next().unwrap_or_default();
        // note we skip index 3 because it should be an empty string
        let password = parts[4].into();

        if a < 1 {
            return Err(ParseError::ATooLow);
        }

        if b < a {
            return Err(ParseError::BLowerThanA);
        }

        if character == char::default() {
            return Err(ParseError::MissingRequiredCharacterSpec);
        }

        Ok(Self {
            a: a,
            b: b,
            character,
            password,
        })
    }
}

impl PasswordSpec {
    fn has_valid_sled_password(&self) -> bool {
        let count = self
            .password
            .chars()
            .filter(|&c| c == self.character)
            .count() as u8;
        count >= self.a && count <= self.b
    }

    fn has_valid_toboggan_password(&self) -> bool {
        if ((self.a - 1) as usize) < self.password.len()
            || ((self.b - 1) as usize) < self.password.len()
        {
            let first_char = self
                .password
                .chars()
                .nth((self.a - 1) as usize)
                .unwrap_or_default();
            let second_char = self
                .password
                .chars()
                .nth((self.b - 1) as usize)
                .unwrap_or_default();

            first_char != second_char
                && (first_char == self.character || second_char == self.character)
        } else {
            false
        }
    }
}

fn count_invalid_passwords_sled_style(list: &[String]) -> usize {
    let mut count = 0;

    for item in list {
        match item.parse::<PasswordSpec>() {
            Ok(spec) => {
                if spec.has_valid_sled_password() {
                    count += 1;
                }
            }
            Err(e) => eprintln!("Could not parse \"{}\": {}", item, e),
        }
    }

    count
}

fn count_invalid_passwords_toboggan_style(list: &[String]) -> usize {
    let mut count = 0;

    for item in list {
        match item.parse::<PasswordSpec>() {
            Ok(spec) => {
                if spec.has_valid_toboggan_password() {
                    count += 1;
                }
            }
            Err(e) => eprintln!("Could not parse \"{}\": {}", item, e),
        }
    }

    count
}

pub fn part_one(data: &[String]) {
    let count = count_invalid_passwords_sled_style(data);
    println!("Valid passwords: {}", count);
}

pub fn part_two(data: &[String]) {
    let count = count_invalid_passwords_toboggan_style(data);
    println!("Valid passwords: {}", count);
}

pub fn runner(data: Vec<String>) -> DayRunner {
    DayRunner::new(data, Some(part_one), Some(part_two))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day_02_parses_valid_spec() {
        let spec: PasswordSpec = "1-3 a: abcde".parse().unwrap();

        assert_eq!(spec.a, 1);
        assert_eq!(spec.b, 3);
        assert_eq!(spec.character, 'a');
        assert_eq!(spec.password, "abcde");
    }

    #[test]
    fn day_02_cannot_parse_spec_out_of_syntax() {
        let result = "0-: abcde".parse::<PasswordSpec>().err();

        assert_eq!(result, Some(ParseError::InvalidSyntax));
    }

    #[test]
    fn day_02_cannot_parse_spec_with_min_less_than_one() {
        let result = "0-4 a: abcde".parse::<PasswordSpec>().err();

        assert_eq!(result, Some(ParseError::ATooLow));
    }

    #[test]
    fn day_02_cannot_parse_spec_with_max_less_than_min() {
        let result = "4-1 a: abcde".parse::<PasswordSpec>().err();

        assert_eq!(result, Some(ParseError::BLowerThanA));
    }

    #[test]
    fn day_02_cannot_parse_spec_with_missing_required_char() {
        let result = "1-3 : abcde".parse::<PasswordSpec>().err();

        assert_eq!(result, Some(ParseError::MissingRequiredCharacterSpec));
    }

    #[test]
    fn day_02_identifies_valid_sled_password() {
        let spec: PasswordSpec = "1-3 a: abcde".parse().unwrap();

        assert!(spec.has_valid_sled_password());
    }

    #[test]
    fn day_02_identifies_invalid_sled_password() {
        let spec: PasswordSpec = "1-3 b: cdefg".parse().unwrap();

        assert_eq!(false, spec.has_valid_sled_password());
    }

    #[test]
    fn day_02_identifies_valid_toboggan_password() {
        let spec: PasswordSpec = "1-3 a: abcde".parse().unwrap();

        assert!(spec.has_valid_toboggan_password());
    }

    #[test]
    fn day_02_identifies_invalid_toboggan_password_with_missing_char() {
        let spec: PasswordSpec = "1-3 b: cdefg".parse().unwrap();

        assert_eq!(false, spec.has_valid_toboggan_password());
    }

    #[test]
    fn day_02_identifies_invalid_toboggan_password_with_repeat_char() {
        let spec: PasswordSpec = "2-9 c: ccccccccc".parse().unwrap();

        assert_eq!(false, spec.has_valid_toboggan_password());
    }
}

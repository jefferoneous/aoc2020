use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Passport {
    fn new() -> Self {
        Passport {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_color: None,
            eye_color: None,
            passport_id: None,
            country_id: None,
        }
    }

    fn has_required_values(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }

    fn has_valid_values(&self) -> bool {
        self.has_valid_birth_year()
            && self.has_valid_issue_year()
            && self.has_valid_expiration_year()
            && self.has_valid_height()
            && self.has_valid_hair_color()
            && self.has_valid_eye_color()
            && self.has_valid_passport_id()
    }

    fn has_valid_birth_year(&self) -> bool {
        if let Some(year) = &self.birth_year {
            if let Ok(year) = year.parse::<u32>() {
                return year >= 1920 && year <= 2002;
            }
        }
        false
    }

    fn has_valid_issue_year(&self) -> bool {
        if let Some(year) = &self.issue_year {
            if let Ok(year) = year.parse::<u32>() {
                return year >= 2010 && year <= 2020;
            }
        }
        false
    }

    fn has_valid_expiration_year(&self) -> bool {
        if let Some(year) = &self.expiration_year {
            if let Ok(year) = year.parse::<u32>() {
                return year >= 2020 && year <= 2030;
            }
        }
        false
    }

    fn has_valid_height(&self) -> bool {
        if let Some(height) = &self.height {
            let units = if height.contains("cm") {
                Some("cm")
            } else if height.contains("in") {
                Some("in")
            } else {
                return false;
            };

            if let Ok(ht) = height
                .chars()
                .take_while(|c| c.is_numeric())
                .collect::<String>()
                .parse::<u32>()
            {
                match units {
                    Some("cm") => return ht >= 150 && ht <= 193,
                    Some("in") => return ht >= 59 && ht <= 76,
                    _ => return false,
                }
            }
        }

        false
    }

    fn has_valid_hair_color(&self) -> bool {
        if let Some(hair_color) = &self.hair_color {
            return hair_color.starts_with("#")
                && hair_color
                    .to_lowercase()
                    .chars()
                    .all(|c| "#abcdef0123456789".contains(c));
        }
        false
    }

    fn has_valid_eye_color(&self) -> bool {
        if let Some(eye_color) = &self.eye_color {
            let valid_colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

            return valid_colors.contains(&eye_color.as_ref());
        }

        false
    }

    fn has_valid_passport_id(&self) -> bool {
        if let Some(passport_id) = &self.passport_id {
            return passport_id.len() == 9 && passport_id.chars().all(|c| c.is_numeric());
        }
        false
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ParseError {
    InvalidSyntax,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                ParseError::InvalidSyntax => "Passport string doesn't follow syntax",
            }
        )
    }
}

impl FromStr for Passport {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut passport = Passport::new();

        for (key, value) in s
            .split_ascii_whitespace()
            .map(|c| c.split(':'))
            .map(|mut p| (p.next().unwrap_or_default(), p.next().unwrap_or_default()))
        {
            match key {
                "byr" => passport.birth_year = Some(value.to_string()),
                "iyr" => passport.issue_year = Some(value.to_string()),
                "eyr" => passport.expiration_year = Some(value.to_string()),
                "hgt" => passport.height = Some(value.to_string()),
                "hcl" => passport.hair_color = Some(value.to_string()),
                "ecl" => passport.eye_color = Some(value.to_string()),
                "pid" => passport.passport_id = Some(value.to_string()),
                "cid" => passport.country_id = Some(value.to_string()),
                _ => return Err(ParseError::InvalidSyntax),
            }
        }

        Ok(passport)
    }
}

fn parse_input_lines(lines: &[&str]) -> Result<Vec<Passport>, ParseError> {
    let mut passport_data = String::new();
    let mut result: Vec<Passport> = vec![];

    for line in lines {
        if line.is_empty() {
            result.push(passport_data.parse()?);
            passport_data = String::new();
        } else {
            if !passport_data.is_empty() {
                passport_data.push_str(" ");
            }
            passport_data.push_str(line);
        }
    }

    if !passport_data.is_empty() {
        result.push(passport_data.parse()?);
    }

    Ok(result)
}

fn count_passports_with_all_required_values(passports: &[Passport]) -> usize {
    passports.iter().filter(|p| p.has_required_values()).count()
}

fn count_passports_with_valid_values(passports: &[Passport]) -> usize {
    passports.iter().filter(|p| p.has_valid_values()).count()
}

pub fn part_one(data: &[&str]) {
    if let Ok(passports) = parse_input_lines(data) {
        let count = count_passports_with_all_required_values(&passports);
        println!("Valid passports: {}", count);
    } else {
        println!("No passports found");
    }
}

pub fn part_two(data: &[&str]) {
    if let Ok(passports) = parse_input_lines(data) {
        let count = count_passports_with_valid_values(&passports);
        println!("Valid passports: {}", count);
    } else {
        println!("No passports found");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day_04_parses_valid_syntax() -> Result<(), ParseError> {
        let sample_data =
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm"
                .to_string();

        let passport = sample_data.parse::<Passport>()?;
        assert_eq!(passport.birth_year, Some("1937".to_string()));
        assert_eq!(passport.issue_year, Some("2017".to_string()));
        assert_eq!(passport.expiration_year, Some("2020".to_string()));
        assert_eq!(passport.height, Some("183cm".to_string()));
        assert_eq!(passport.hair_color, Some("#fffffd".to_string()));
        assert_eq!(passport.eye_color, Some("gry".to_string()));
        assert_eq!(passport.passport_id, Some("860033327".to_string()));
        assert_eq!(passport.country_id, Some("147".to_string()));

        Ok(())
    }

    #[test]
    fn day_04_parses_valid_syntax_partial_passport() -> Result<(), ParseError> {
        let sample_data = "ecl:gry pid:860033327 eyr:2020".to_string();

        let passport = sample_data.parse::<Passport>()?;
        assert_eq!(passport.birth_year, None);
        assert_eq!(passport.issue_year, None);
        assert_eq!(passport.expiration_year, Some("2020".to_string()));
        assert_eq!(passport.height, None);
        assert_eq!(passport.hair_color, None);
        assert_eq!(passport.eye_color, Some("gry".to_string()));
        assert_eq!(passport.passport_id, Some("860033327".to_string()));
        assert_eq!(passport.country_id, None);

        Ok(())
    }

    #[test]
    fn day_04_does_not_parse_invalid_syntax() {
        let sample_data =
            "ecl:gry pid:860033327 gyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm"
                .to_string();

        let result = sample_data.parse::<Passport>();
        assert_eq!(result, Err(ParseError::InvalidSyntax));
    }

    #[test]
    fn day_04_identifies_passport_as_having_all_required_values() -> Result<(), ParseError> {
        let sample_data =
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm"
                .to_string();

        let passport = sample_data.parse::<Passport>()?;

        assert!(passport.has_required_values());

        Ok(())
    }

    #[test]
    fn day_04_identifies_valid_birth_year() -> Result<(), ParseError> {
        let sample_data =
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 hgt:183cm".to_string();

        let passport = sample_data.parse::<Passport>()?;

        assert!(passport.has_valid_birth_year());

        Ok(())
    }

    #[test]
    fn day_04_identifies_valid_issue_year() -> Result<(), ParseError> {
        let sample_data =
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 hgt:183cm".to_string();

        let passport = sample_data.parse::<Passport>()?;

        assert!(passport.has_valid_issue_year());

        Ok(())
    }

    #[test]
    fn day_04_identifies_valid_expiration_year() -> Result<(), ParseError> {
        let sample_data =
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 hgt:183cm".to_string();

        let passport = sample_data.parse::<Passport>()?;

        assert!(passport.has_valid_expiration_year());

        Ok(())
    }

    #[test]
    fn day_04_identifies_valid_height_in_cm() -> Result<(), ParseError> {
        let sample_data =
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 hgt:183cm".to_string();

        let passport = sample_data.parse::<Passport>()?;

        assert!(passport.has_valid_height());

        Ok(())
    }

    #[test]
    fn day_04_parses_valid_height_in_inches() -> Result<(), ParseError> {
        let sample_data =
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 hgt:71in".to_string();

        let passport = sample_data.parse::<Passport>()?;

        assert!(passport.has_valid_height());

        Ok(())
    }

    #[test]
    fn day_04_identifies_passport_without_country_id_as_having_all_required_values(
    ) -> Result<(), ParseError> {
        let sample_data =
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 hgt:183cm".to_string();

        let passport = sample_data.parse::<Passport>()?;

        assert!(passport.has_required_values());

        Ok(())
    }

    #[test]
    fn day_04_identifies_valid_hair_color() -> Result<(), ParseError> {
        let sample_data =
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 hgt:71in hcl:#ae17e1"
                .to_string();

        let passport = sample_data.parse::<Passport>()?;

        assert!(passport.has_valid_hair_color());

        Ok(())
    }

    #[test]
    fn day_04_identifies_valid_eye_color() -> Result<(), ParseError> {
        let sample_data =
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 hgt:71in hcl:#ae17e1"
                .to_string();

        let passport = sample_data.parse::<Passport>()?;

        assert!(passport.has_valid_eye_color());

        Ok(())
    }

    #[test]
    fn day_04_identifies_valid_passport_id() -> Result<(), ParseError> {
        let sample_data =
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 hgt:71in hcl:#ae17e1"
                .to_string();

        let passport = sample_data.parse::<Passport>()?;

        assert!(passport.has_valid_passport_id());

        Ok(())
    }

    #[test]
    fn day_04_parses_input_lines_as_passports() -> Result<(), ParseError> {
        let sample_lines = vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
            "byr:1937 iyr:2017 cid:147 hgt:183cm",
            "",
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
            "hcl:#cfa07d byr:1929",
            "",
            "hcl:#ae17e1 iyr:2013",
            "eyr:2024",
            "ecl:brn pid:760753108 byr:1931",
            "hgt:179cm",
            "",
            "hcl:#cfa07d eyr:2025 pid:166559648",
            "iyr:2011 ecl:brn hgt:59in",
        ];

        parse_input_lines(&sample_lines)?;

        Ok(())
    }

    #[test]
    fn day_04_counts_passports_with_required_values() -> Result<(), ParseError> {
        let sample_lines = vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
            "byr:1937 iyr:2017 cid:147 hgt:183cm",
            "",
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
            "hcl:#cfa07d byr:1929",
            "",
            "hcl:#ae17e1 iyr:2013",
            "eyr:2024",
            "ecl:brn pid:760753108 byr:1931",
            "hgt:179cm",
            "",
            "hcl:#cfa07d eyr:2025 pid:166559648",
            "iyr:2011 ecl:brn hgt:59in",
        ];

        let passports = parse_input_lines(&sample_lines)?;
        let count = count_passports_with_all_required_values(&passports);

        assert_eq!(count, 2);

        Ok(())
    }

    #[test]
    fn day_04_counts_passports_with_valid_values() -> Result<(), ParseError> {
        let sample_lines = vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
            "byr:1937 iyr:2017 cid:147 hgt:183cm",
            "",
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
            "hcl:#cfa07d byr:1929",
            "",
            "hcl:#ae17e1 iyr:2013",
            "eyr:2024",
            "ecl:brn pid:760753108 byr:1931",
            "hgt:179cm",
            "",
            "hcl:#cfa07d eyr:2025 pid:166559648",
            "iyr:2011 ecl:brn hgt:59in",
        ];

        let passports = parse_input_lines(&sample_lines)?;
        let count = count_passports_with_valid_values(&passports);

        assert_eq!(count, 2);

        Ok(())
    }
}

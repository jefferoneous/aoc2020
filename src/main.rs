use std::error::Error;
use std::fs;
use std::io::Error as IoError;
use std::path::PathBuf;

use clap::{App, Arg};

mod days;

fn load_data_from_file(day: u8) -> Result<Vec<String>, IoError> {
    let path: PathBuf = ["input", &format!("day_{:02}", day)].iter().collect();
    let contents = fs::read_to_string(path)?;
    let result = contents.lines().map(str::to_string).collect();

    Ok(result)
}

fn day_is_in_range(value: String) -> Result<(), String> {
    match value.parse::<u8>() {
        Ok(day) => {
            if day < 1 || day > days::days_implemented() {
                return Err(format!(
                    "Day must be between 1 and {}",
                    days::days_implemented()
                ));
            }
        }
        Err(_) => {
            return Err(String::from("Not a number"));
        }
    }

    Ok(())
}

fn process_args() -> u8 {
    let matches = App::new("Advent of Code 2020 Solution Runner")
        .version("0.1.0")
        .author("Jeff Mattfield")
        .about("Runs solutions to the problems posed during the Advent of Code 2020 (https://adventofcode.com/2020)")
        .arg(
            Arg::with_name("day")
                .help(&*format!("the day of the month (1-{})", days::days_implemented()))
                .index(1)
                .validator(day_is_in_range)
                .required(false)
        )
        .get_matches();

    let day: u8 = matches.value_of("day").unwrap_or("20").parse().unwrap();

    day
}

fn main() -> Result<(), Box<dyn Error>> {
    let day = process_args();
    let data = load_data_from_file(day)?;
    let data: Vec<_> = data.iter().map(String::as_str).collect();

    let runner = days::get_runner(day);
    days::run(runner, &data);

    Ok(())
}

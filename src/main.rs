#[macro_use]
extern crate lazy_static;

use std::error::Error;
use std::fs;
use std::io::Error as IoError;
use std::path::PathBuf;

use clap::{App, Arg};

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;

const LAST_DAY_IMPLEMENTED: u8 = 8;

fn load_data_from_file(path: PathBuf) -> Result<Vec<String>, IoError> {
    let contents = fs::read_to_string(path)?;
    let result = contents.lines().map(|l| l.to_string()).collect();

    Ok(result)
}

fn day_is_in_range(value: String) -> Result<(), String> {
    match value.parse::<u8>() {
        Ok(day) => {
            if day >= 1 && day <= LAST_DAY_IMPLEMENTED {
                Ok(())
            } else {
                Err(format!(
                    "Day must be between 1 and {}",
                    LAST_DAY_IMPLEMENTED
                ))
            }
        }
        Err(_) => Err(String::from("Not a number")),
    }
}

fn run(part_one: fn(&[String]), part_two: fn(&[String]), data: &[String]) {
    println!("\nPart One\n========");
    part_one(&data);
    println!("\nPart Two\n========");
    part_two(&data);
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Advent of Code 2020 Solution Runner")
        .version("0.1.0")
        .author("Jeff Mattfield")
        .about("Runs solutions to the problems posed during the Advent of Code 2020 (https://adventofcode.com/2020)")
        .arg(
            Arg::with_name("day")
                .help(&*format!("the day of the month (1-{})", LAST_DAY_IMPLEMENTED))
                .index(1)
                .validator(day_is_in_range)
                .required(false)
        )
        .get_matches();

    let day: u8 = matches.value_of("day").unwrap_or("1").parse().unwrap_or(1);

    let path: PathBuf = ["input", format!("day_{:02}", day).as_ref()]
        .iter()
        .collect();

    let data = load_data_from_file(path)?;

    match day {
        1 => run(day_01::part_one, day_01::part_two, &data),
        2 => run(day_02::part_one, day_02::part_two, &data),
        3 => run(day_03::part_one, day_03::part_two, &data),
        4 => run(day_04::part_one, day_04::part_two, &data),
        5 => run(day_05::part_one, day_05::part_two, &data),
        6 => run(day_06::part_one, day_06::part_two, &data),
        7 => run(day_07::part_one, day_07::part_two, &data),
        8 => run(day_08::part_one, day_08::part_two, &data),
        _ => println!("Day \"{}\" is not implemented or is not valid", day),
    };

    Ok(())
}

use std::path::PathBuf;

use clap::{App, Arg};

mod day_01;
mod day_02;
mod day_03;

const LAST_DAY_IMPLEMENTED: u8 = 3;

fn main() {
    let matches = App::new("Advent of Code 2020 Runner")
        .version("0.1.0")
        .author("Jeff Mattfield")
        .about("Runs solutions to the problems posed during the Advent of Code 2020 (https://adventofcode.com/2020)")
        .after_help(&*format!("Implemented days: 1-{}", LAST_DAY_IMPLEMENTED))
        .arg(
            Arg::with_name("day")
                .help("the day of the month")
                .index(1)
                .required(true)
        )
        .arg(
            Arg::with_name("input_dir")
                .help("the path to the directory containing input files")
                .short("d")
                .required(true)
                .takes_value(true)
        )
        .get_matches();

    let day: u8 = matches.value_of("day").unwrap_or("1").parse().unwrap_or(1);

    if day > LAST_DAY_IMPLEMENTED {
        eprintln!("Day must be between 1 and {}", LAST_DAY_IMPLEMENTED);
        return;
    }

    let input_dir = matches.value_of("input_dir").unwrap_or_default();
    let mut path = PathBuf::from(input_dir);
    path.push(format!("day_{:02}", day));

    match day {
        1 => day_01::run(path),
        2 => day_02::run(path),
        3 => day_03::run(path),
        _ => println!("Day \"{}\" is not implemented or is not valid", day),
    }
}

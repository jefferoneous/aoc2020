use clap::{App, Arg};

mod day_one;

const LAST_DAY_IMPLEMENTED: u8 = 1;

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
            Arg::with_name("filename")
                .help("the path to a file used by the day")
                .short("f")
                .required(false)
                .takes_value(true)
        )
        .get_matches();

    let day: u8 = matches.value_of("day").unwrap_or("1").parse().unwrap_or(1);

    match day {
        1 => {
            if let Some(filename) = matches.value_of("filename") {
                day_one::run(filename);
            } else {
                println!("Day 1 requires an input file");
            }
        }

        _ => println!("Day \"{}\" is not implemented or is not valid", day),
    }
}

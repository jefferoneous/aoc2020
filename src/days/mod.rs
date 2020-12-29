mod util;
pub use util::grid;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;

lazy_static! {
    static ref BUILDERS: Vec<DayRunner> = vec![
        (day_01::part_one, day_01::part_two),
        (day_02::part_one, day_02::part_two),
        (day_03::part_one, day_03::part_two),
        (day_04::part_one, day_04::part_two),
        (day_05::part_one, day_05::part_two),
        (day_06::part_one, day_06::part_two),
        (day_07::part_one, day_07::part_two),
        (day_08::part_one, day_08::part_two),
        (day_09::part_one, day_09::part_two),
        (day_10::part_one, day_10::part_two),
        (day_11::part_one, day_11::part_two),
        (day_12::part_one, day_12::part_two),
        (day_13::part_one, day_13::part_two),
        (day_14::part_one, day_14::part_two),
        (day_15::part_one, day_15::part_two),
    ];
}

type DayRunner = (fn(&[&str]), fn(&[&str]));

pub fn run(runner: DayRunner, data: &[&str]) {
    println!("\nPart One\n========");
    (runner.0)(&data);
    println!("\nPart Two\n========");
    (runner.1)(&data);
}

pub fn days_implemented() -> usize {
    BUILDERS.len()
}

pub fn get_runner(day: usize) -> DayRunner {
    BUILDERS[day - 1]
}

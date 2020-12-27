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

fn builders() -> Vec<fn(Vec<String>) -> DayRunner> {
    vec![
        day_01::runner,
        day_02::runner,
        day_03::runner,
        day_04::runner,
        day_05::runner,
        day_06::runner,
        day_07::runner,
        day_08::runner,
        day_09::runner,
        day_10::runner,
        day_11::runner,
    ]
}

pub struct DayRunner {
    data: Vec<String>,
    part_one: Option<fn(&[String])>,
    part_two: Option<fn(&[String])>,
}

impl DayRunner {
    pub fn new(
        data: Vec<String>,
        part_one: Option<fn(&[String])>,
        part_two: Option<fn(&[String])>,
    ) -> Self {
        Self {
            data,
            part_one,
            part_two,
        }
    }

    pub fn run(&self) {
        if let Some(part_one) = self.part_one {
            println!("\nPart One\n========");
            part_one(&self.data);
        }
        if let Some(part_two) = self.part_two {
            println!("\nPart Two\n========");
            part_two(&self.data);
        }
    }
}

pub fn days_implemented() -> usize {
    builders().len()
}

pub fn get_runner(day: usize, data: Vec<String>) -> Result<DayRunner, String> {
    if day > 0 && day <= days_implemented() {
        Ok((builders()[day - 1])(data))
    } else {
        Err(format!(
            "Day \"{}\" is not implemented or is not valid",
            day
        ))
    }
}

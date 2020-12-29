use super::DayRunner;

pub fn runner(data: Vec<String>) -> DayRunner {
    DayRunner::new(data, Some(part_one), Some(part_two))
}

fn part_one(data: &[String]) {
    todo!("do something and print the result");
}

fn part_two(data: &[String]) {
    todo!("do something and print the result");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day_xx_test() {}
}

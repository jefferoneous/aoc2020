use std::collections::HashMap;

pub fn part_one(data: &[&str]) {
    let list = parse_input(data[0]);
    let result = calculate_nth_number(&list, 2020);

    println!("The 2020th number: {}", result);
}

pub fn part_two(data: &[&str]) {
    let list = parse_input(data[0]);
    let result = calculate_nth_number(&list, 30000000);

    println!("The 30,000,000th number: {}", result);
}

fn parse_input(data: &str) -> Vec<usize> {
    data.split(",").filter_map(|s| s.parse().ok()).collect()
}

fn calculate_nth_number(list: &[usize], n: usize) -> usize {
    if n < list.len() {
        return list[n];
    }

    type SpokenNumbers = HashMap<usize, usize>;

    fn record_number(numbers: &mut SpokenNumbers, number: usize, turn: usize) {
        if let Some(v) = numbers.get_mut(&number) {
            *v = turn;
        } else {
            numbers.insert(number, turn);
        }
    }

    let mut spoken_numbers: SpokenNumbers = HashMap::new();

    for (turn, number) in list[..list.len() - 1].iter().enumerate() {
        record_number(&mut spoken_numbers, *number, turn + 1);
        // println!("{:4}: {}", turn, *number);
    }

    let mut current_number = list.last().copied().unwrap();

    for turn in list.len().. {
        if turn == n {
            return current_number;
        }

        let v = spoken_numbers.get(&current_number).copied();
        record_number(&mut spoken_numbers, current_number, turn);

        if v.is_some() {
            current_number = turn - v.unwrap();
        } else {
            current_number = 0;
        }
        // println!("{:4}: {}", turn, last_spoken_number);
    }

    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day_15_calculate_nth_number() {
        let list = parse_input("0,3,6");
        let result = calculate_nth_number(&list, 2020);
        assert_eq!(436, result);

        let list = parse_input("1,3,2");
        let result = calculate_nth_number(&list, 2020);
        assert_eq!(1, result);

        let list = parse_input("2,1,3");
        let result = calculate_nth_number(&list, 2020);
        assert_eq!(10, result);

        let list = parse_input("1,2,3");
        let result = calculate_nth_number(&list, 2020);
        assert_eq!(27, result);

        let list = parse_input("2,3,1");
        let result = calculate_nth_number(&list, 2020);
        assert_eq!(78, result);

        let list = parse_input("3,2,1");
        let result = calculate_nth_number(&list, 2020);
        assert_eq!(438, result);

        let list = parse_input("3,1,2");
        let result = calculate_nth_number(&list, 2020);
        assert_eq!(1836, result);
    }
}

fn parse_seat_spec(spec: &str) -> u32 {
    let mut row_start = 0;
    let mut row_end = 127;
    let mut column_start = 0;
    let mut column_end = 7;

    for c in spec.chars().filter(|c| "FB".contains(*c)) {
        match c {
            'F' => row_end -= (row_end - row_start + 1) / 2,
            'B' => row_start += (row_end - row_start + 1) / 2,
            _ => (),
        }

        if row_start == row_end {
            break;
        }
    }

    for c in spec.chars().filter(|c| "LR".contains(*c)) {
        match c {
            'L' => column_end -= (column_end - column_start + 1) / 2,
            'R' => column_start += (column_end - column_start + 1) / 2,
            _ => (),
        }

        if column_start == column_end {
            break;
        }
    }

    row_start * 8 + column_start
}

pub fn calculate_highest_seat_id(data: &[String]) -> u32 {
    data.iter().fold(0u32, |id, seat_spec| {
        let seat_id = parse_seat_spec(seat_spec);
        if seat_id > id {
            seat_id
        } else {
            id
        }
    })
}

pub fn part_one(data: &[String]) {
    let id = calculate_highest_seat_id(data);
    println!("Highest seat ID: {}", id);
}

pub fn part_two(data: &[String]) {
    //     if let Ok(passports) = parse_input_lines(data) {
    //         let count = count_passports_with_valid_values(&passports);
    //         println!("Valid passports: {}", count);
    //     } else {
    //         println!("No passports found");
    //     }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day_05_calculates_correct_seat_ids() {
        let sample_data = vec![
            "FBFBBFFRLR".to_string(),
            "BFFFBBFRRR".to_string(),
            "FFFBBBFRRR".to_string(),
            "BBFFBBFRLL".to_string(),
        ];

        let seat_ids = vec![357, 567, 119, 820];

        let mut calculated_seat_ids = vec![];

        for spec in sample_data {
            calculated_seat_ids.push(parse_seat_spec(&spec));
        }

        assert_eq!(calculated_seat_ids, seat_ids);
    }

    #[test]
    fn day_05_calculates_correct_highest_seat_id() {
        let sample_data = vec![
            "FBFBBFFRLR".to_string(),
            "BFFFBBFRRR".to_string(),
            "FFFBBBFRRR".to_string(),
            "BBFFBBFRLL".to_string(),
        ];

        let seat_id = calculate_highest_seat_id(&sample_data);

        assert_eq!(seat_id, 820);
    }
}

use std::{fs, io};

fn find_twenty_twenty_sum(list: &Vec<u32>) -> Option<(u32, u32)> {
    if let Some(a) = list.iter().find(|&&x| list.iter().any(|&y| x + y == 2020)) {
        Some((*a, 2020 - *a))
    } else {
        None
    }
}

fn load_list_from_file(filename: &str) -> Result<Vec<u32>, io::Error> {
    let mut result: Vec<u32> = vec![];

    let contents = fs::read_to_string(filename)?;

    for line in contents.lines() {
        println!("line: \"{}\"", line);
        result.push(line.parse().unwrap());
    }

    Ok(result)
}

pub fn run(filename: &str) {
    let list = load_list_from_file(filename).unwrap();
    if let Some((a, b)) = find_twenty_twenty_sum(&list) {
        println!("a: {}, b: {}, a*b: {}", a, b, a * b);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_finds_correct_pair() {
        let list = vec![1721, 979, 366, 299, 675, 1456];
        let (a, b) = find_twenty_twenty_sum(&list).unwrap();

        assert_eq!(a, 1721);
        assert_eq!(b, 299);
    }
}

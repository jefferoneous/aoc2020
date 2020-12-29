use std::collections::{HashMap, HashSet};

fn count_questions_everyone_in_group_answered(group: &[&str]) -> u32 {
    let mut questions_answered: HashMap<char, u32> = HashMap::new();

    for answers in group {
        for c in answers.chars() {
            if let Some(v) = questions_answered.get_mut(&c) {
                *v += 1;
            } else {
                questions_answered.insert(c, 1);
            }
        }
    }

    questions_answered
        .values()
        .filter(|&&v| v == group.len() as u32)
        .count() as u32
}

fn count_questions_everyone_answered(data: &[&str]) -> u32 {
    let mut count = 0;
    for group in separate_groups(data) {
        count += count_questions_everyone_in_group_answered(&group);
    }

    count
}

fn count_questions_anyone_in_group_answered(group: &[&str]) -> u32 {
    let mut questions_answered: HashSet<char> = HashSet::new();

    for answers in group {
        for c in answers.chars() {
            questions_answered.insert(c);
        }
    }

    questions_answered.len() as u32
}

fn count_questions_anyone_answered(data: &[&str]) -> u32 {
    let mut count = 0;
    for group in separate_groups(data) {
        count += count_questions_anyone_in_group_answered(&group);
    }

    count
}

fn separate_groups<'a>(data: &[&'a str]) -> Vec<Vec<&'a str>> {
    let mut groups = vec![];
    let mut group = vec![];

    for line in data {
        if line.is_empty() && !group.is_empty() {
            groups.push(group);
            group = vec![];
        } else {
            group.push(line.to_owned());
        }
    }

    if !group.is_empty() {
        groups.push(group);
    }

    groups
}

pub fn part_one(data: &[&str]) {
    let count = count_questions_anyone_answered(data);
    println!("Questions anyone answered: {}", count);
}

pub fn part_two(data: &[&str]) {
    let count = count_questions_everyone_answered(data);
    println!("Questions everyone answered: {}", count);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day_06_separates_groups() {
        let sample_data = vec![
            "abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b",
        ];

        assert_eq!(separate_groups(&sample_data).len(), 5);
    }

    #[test]
    fn day_06_counts_questions_anyone_answered_in_single_group() {
        let sample_group = vec!["abcx", "abcy", "abcz"];

        assert_eq!(count_questions_anyone_in_group_answered(&sample_group), 6);
    }

    #[test]
    fn day_06_counts_questions_anyone_answered() {
        let sample_data = vec![
            "abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b",
        ];

        assert_eq!(count_questions_anyone_answered(&sample_data), 11)
    }

    #[test]
    fn day_06_counts_questions_everyone_answered_in_single_group() {
        let sample_group = vec!["abcx", "abcy", "abcz"];

        assert_eq!(count_questions_everyone_in_group_answered(&sample_group), 3);
    }

    #[test]
    fn day_06_counts_questions_everyone_answered() {
        let sample_data = vec![
            "abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b",
        ];

        assert_eq!(count_questions_everyone_answered(&sample_data), 6)
    }
}

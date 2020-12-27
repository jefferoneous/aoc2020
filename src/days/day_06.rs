use std::collections::{HashMap, HashSet};

use super::DayRunner;

fn count_questions_everyone_in_group_answered(group: &[String]) -> u32 {
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

fn count_questions_everyone_answered(data: &[String]) -> u32 {
    let mut count = 0;
    for group in separate_groups(data) {
        count += count_questions_everyone_in_group_answered(&group);
    }

    count
}

fn count_questions_anyone_in_group_answered(group: &[String]) -> u32 {
    let mut questions_answered: HashSet<char> = HashSet::new();

    for answers in group {
        for c in answers.chars() {
            questions_answered.insert(c);
        }
    }

    questions_answered.len() as u32
}

fn count_questions_anyone_answered(data: &[String]) -> u32 {
    let mut count = 0;
    for group in separate_groups(data) {
        count += count_questions_anyone_in_group_answered(&group);
    }

    count
}

fn separate_groups(data: &[String]) -> Vec<Vec<String>> {
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

pub fn part_one(data: &[String]) {
    let count = count_questions_anyone_answered(data);
    println!("Questions anyone answered: {}", count);
}

pub fn part_two(data: &[String]) {
    let count = count_questions_everyone_answered(data);
    println!("Questions everyone answered: {}", count);
}

pub fn runner(data: Vec<String>) -> DayRunner {
    DayRunner::new(data, Some(part_one), Some(part_two))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn separates_groups() {
        let sample_data = vec![
            "abc".to_string(),
            "".to_string(),
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "".to_string(),
            "ab".to_string(),
            "ac".to_string(),
            "".to_string(),
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
            "".to_string(),
            "b".to_string(),
        ];

        assert_eq!(separate_groups(&sample_data).len(), 5);
    }

    #[test]
    fn day_06_counts_questions_anyone_answered_in_single_group() {
        let sample_group = vec!["abcx".to_string(), "abcy".to_string(), "abcz".to_string()];

        assert_eq!(count_questions_anyone_in_group_answered(&sample_group), 6);
    }

    #[test]
    fn day_06_counts_questions_anyone_answered() {
        let sample_data = vec![
            "abc".to_string(),
            "".to_string(),
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "".to_string(),
            "ab".to_string(),
            "ac".to_string(),
            "".to_string(),
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
            "".to_string(),
            "b".to_string(),
        ];

        assert_eq!(count_questions_anyone_answered(&sample_data), 11)
    }

    #[test]
    fn day_06_counts_questions_everyone_answered_in_single_group() {
        let sample_group = vec!["abcx".to_string(), "abcy".to_string(), "abcz".to_string()];

        assert_eq!(count_questions_everyone_in_group_answered(&sample_group), 3);
    }

    #[test]
    fn day_06_counts_questions_everyone_answered() {
        let sample_data = vec![
            "abc".to_string(),
            "".to_string(),
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "".to_string(),
            "ab".to_string(),
            "ac".to_string(),
            "".to_string(),
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
            "".to_string(),
            "b".to_string(),
        ];

        assert_eq!(count_questions_everyone_answered(&sample_data), 6)
    }
}

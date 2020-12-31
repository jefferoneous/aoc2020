use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::str::FromStr;

use regex::Regex;

pub fn part_one(data: &[&str]) {
    let (rules, _, nearby_tickets) = parse_input(&data);
    let invalid_ticket_values = find_invalid_ticket_values(&rules, &nearby_tickets);
    let sum: u32 = invalid_ticket_values.iter().sum();

    println!("Ticket scanning error rate: {}", sum);
}

pub fn part_two(data: &[&str]) {
    let (rules, your_ticket, nearby_tickets) = parse_input(&data);
    let fields = determine_fields(&rules, &your_ticket, &nearby_tickets);

    let product: u64 = fields
        .iter()
        .filter_map(|(k, v)| {
            if k.starts_with("departure") {
                Some(*v as u64)
            } else {
                None
            }
        })
        .product();

    println!("Product: {}", product);
}

type Ranges = ((u32, u32), (u32, u32));

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Rule {
    field: String,
    ranges: Ranges,
}

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}-{} or {}-{}",
            self.field, self.ranges.0 .0, self.ranges.0 .1, self.ranges.1 .0, self.ranges.1 .1,
        )
    }
}

impl Rule {
    fn is_valid(&self, value: u32) -> bool {
        (value >= self.ranges.0 .0 && value <= self.ranges.0 .1)
            || (value >= self.ranges.1 .0 && value <= self.ranges.1 .1)
    }
}

lazy_static::lazy_static! {
    static ref PARSE_RULE_REGEX: Regex = Regex::new(r"(?P<field>.*): (?P<low1>\d+)-(?P<high1>\d+) or (?P<low2>\d+)-(?P<high2>\d+)").unwrap();
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(caps) = PARSE_RULE_REGEX.captures(s) {
            Ok(Self {
                field: (&caps["field"]).to_string(),
                ranges: (
                    (
                        (&caps["low1"]).parse().unwrap_or_default(),
                        (&caps["high1"]).parse().unwrap_or_default(),
                    ),
                    (
                        (&caps["low2"]).parse().unwrap_or_default(),
                        (&caps["high2"]).parse().unwrap_or_default(),
                    ),
                ),
            })
        } else {
            Err(format!("Invalid rule: {}", s))
        }
    }
}

fn parse_input(data: &[&str]) -> (Vec<Rule>, Vec<u32>, Vec<Vec<u32>>) {
    let mut rules = vec![];
    let mut your_ticket = vec![];
    let mut nearby_tickets = vec![];

    let mut parse_your_ticket = false;
    let mut parse_nearby_tickets = false;

    for line in data {
        if PARSE_RULE_REGEX.is_match(line) {
            rules.push(line.parse().unwrap());
        } else if line.starts_with("your ticket") {
            parse_your_ticket = true;
        } else if parse_your_ticket {
            your_ticket = line
                .split(",")
                .map(|s| s.parse().unwrap_or_default())
                .collect::<Vec<u32>>();
            parse_your_ticket = false;
        } else if line.starts_with("nearby tickets") {
            parse_nearby_tickets = true;
        } else if parse_nearby_tickets {
            nearby_tickets.push(
                line.split(",")
                    .map(|s| s.parse().unwrap_or_default())
                    .collect(),
            )
        }
    }

    (rules, your_ticket, nearby_tickets)
}

fn find_invalid_ticket_values(rules: &Vec<Rule>, nearby_tickets: &Vec<Vec<u32>>) -> Vec<u32> {
    nearby_tickets
        .iter()
        .filter_map(|t| {
            t.iter()
                .find(|&&v| !rules.iter().any(|r| r.is_valid(v)))
                .copied()
        })
        .collect()
}

fn is_valid_ticket(ticket: &Vec<u32>, rules: &Vec<Rule>) -> bool {
    ticket.iter().all(|v| rules.iter().any(|r| r.is_valid(*v)))
}

fn determine_fields<'a>(
    rules: &'a Vec<Rule>,
    your_ticket: &Vec<u32>,
    nearby_tickets: &Vec<Vec<u32>>,
) -> HashMap<&'a str, u32> {
    let valid_tickets: Vec<&Vec<u32>> = nearby_tickets
        .iter()
        .filter(|t| is_valid_ticket(&t, &rules))
        .collect();

    let mut columns: Vec<Option<Vec<u32>>> = (0..valid_tickets[0].len())
        .map(|i| {
            Some(
                valid_tickets
                    .iter()
                    .enumerate()
                    .map(|(_, v)| v[i])
                    .collect::<Vec<u32>>(),
            )
        })
        .collect();
    let mut remaining_rules: Vec<&Rule> = rules.iter().collect();
    let mut result: HashMap<&str, u32> = HashMap::new();

    loop {
        let column_count = columns.len();

        if let Some((ri, ci)) = columns
            .iter()
            .enumerate()
            .filter(|(_, o)| o.is_some())
            .find_map(|(ci, o)| {
                if let Some(c) = o {
                    let rules = remaining_rules
                        .iter()
                        .enumerate()
                        .filter(|(_, r)| c.iter().all(|v| r.is_valid(*v)))
                        .collect::<Vec<(usize, &&Rule)>>();
                    if rules.len() == 1 {
                        return Some((rules[0].0, ci));
                    }
                }
                None
            })
        {
            result.insert(&remaining_rules[ri].field[..], your_ticket[ci]);
            columns[ci] = None;
            remaining_rules.remove(ri);
        }

        if remaining_rules.len() == column_count || remaining_rules.len() == 0 {
            break;
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day_16_parse_data() {
        let data = vec![
            "class: 1-3 or 5-7",
            "row: 6-11 or 33-44",
            "seat: 13-40 or 45-50",
            "",
            "your ticket:",
            "7,1,14",
            "",
            "nearby tickets:",
            "7,3,47",
            "40,4,50",
            "55,2,20",
            "38,6,12",
        ];
        let (rules, your_ticket, nearby_tickets) = parse_input(&data);

        assert_eq!(
            Rule {
                field: "class".into(),
                ranges: ((1, 3), (5, 7))
            },
            rules[0]
        );
        assert_eq!(
            Rule {
                field: "row".into(),
                ranges: ((6, 11), (33, 44))
            },
            rules[1]
        );
        assert_eq!(
            Rule {
                field: "seat".into(),
                ranges: ((13, 40), (45, 50))
            },
            rules[2]
        );

        assert_eq!(vec![7, 1, 14], your_ticket);

        assert_eq!(vec![7, 3, 47], nearby_tickets[0]);
        assert_eq!(vec![40, 4, 50], nearby_tickets[1]);
        assert_eq!(vec![55, 2, 20], nearby_tickets[2]);
        assert_eq!(vec![38, 6, 12], nearby_tickets[3]);
    }

    #[test]
    fn day_16_identify_invalid_nearby_tickets() {
        let data = vec![
            "class: 1-3 or 5-7",
            "row: 6-11 or 33-44",
            "seat: 13-40 or 45-50",
            "",
            "your ticket:",
            "7,1,14",
            "",
            "nearby tickets:",
            "7,3,47",
            "40,4,50",
            "55,2,20",
            "38,6,12",
        ];
        let (rules, _, nearby_tickets) = parse_input(&data);

        let invalid_ticket_values = find_invalid_ticket_values(&rules, &nearby_tickets);

        assert_eq!(71u32, invalid_ticket_values.iter().sum());
    }

    #[test]
    fn day_16_determine_fields() {
        let data = vec![
            "class: 0-1 or 4-19",
            "row: 0-5 or 8-19",
            "seat: 0-13 or 16-19",
            "",
            "your ticket:",
            "11,12,13",
            "",
            "nearby tickets:",
            "3,9,18",
            "15,1,5",
            "5,14,9",
        ];
        let (rules, your_ticket, nearby_tickets) = parse_input(&data);

        let fields = determine_fields(&rules, &your_ticket, &nearby_tickets);

        assert_eq!(12, fields["class"]);
        assert_eq!(11, fields["row"]);
        assert_eq!(13, fields["seat"]);
    }
}

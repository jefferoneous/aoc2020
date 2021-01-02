use std::collections::HashMap;
use std::str::FromStr;

use regex::Regex;

pub fn part_one(data: &[&str]) {
    if let Ok((rules, messages)) = parse_data(data) {
        let count = messages.iter().filter(|m| rules.matches(m)).count();
        println!("Count of valid messages: {}", count);
    }
}

pub fn part_two(data: &[&str]) {
    if let Ok((rules, messages)) = parse_data_with_looping_rules(data) {
        let count = messages.iter().filter(|m| rules.matches(m)).count();
        println!("Count of valid messages: {}", count);
    }
}

fn parse_data<'a>(data: &'a [&'a str]) -> Result<(Rules, &'a [&'a str]), String> {
    let mut iter = data.iter().enumerate();

    let mut rules = Rules::new(1);

    while let Some((i, s)) = iter.next() {
        if !s.is_empty() {
            rules.add_rule(s)?;
        } else {
            return Ok((rules, &data[i + 1..]));
        }
    }

    Err("Something unexpected occurred".into())
}

fn parse_data_with_looping_rules<'a>(
    data: &'a [&'a str],
) -> Result<(Rules, &'a [&'a str]), String> {
    let mut iter = data
        .iter()
        .map(|s| {
            if s.starts_with("8:") {
                "8: 42 | 42 8"
            } else if s.starts_with("11:") {
                "11: 42 31 | 42 11 31"
            } else {
                s
            }
        })
        .enumerate();

    let mut rules = Rules::new(5);

    while let Some((i, s)) = iter.next() {
        if !s.is_empty() {
            rules.add_rule(s)?;
        } else {
            return Ok((rules, &data[i + 1..]));
        }
    }

    Err("Something unexpected occurred".into())
}

lazy_static::lazy_static! {
    static ref PARSE_RULE_REGEX: Regex = Regex::new(r"(?P<id>\d+): *(?P<spec>.*)").unwrap();
    static ref PARSE_OR_REGEX: Regex = Regex::new(r"(?P<left>.*)\|(?P<right>.*)").unwrap();
    static ref PARSE_PATTERN_REGEX: Regex = Regex::new(r#""(?P<pattern>(.*))""#).unwrap();
}

enum RuleSpec {
    Compound(Vec<usize>),
    Or((Vec<usize>, Vec<usize>)),
    Pattern(String),
}

impl FromStr for RuleSpec {
    type Err = String;

    fn from_str(spec: &str) -> Result<Self, Self::Err> {
        fn parse_compound(s: &str) -> Vec<usize> {
            s.split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect()
        }

        if let Some(or_caps) = PARSE_OR_REGEX.captures(spec) {
            let left = parse_compound(&or_caps["left"]);
            let right = parse_compound(&or_caps["right"]);

            return Ok(RuleSpec::Or((left, right)));
        } else if let Some(pattern_caps) = PARSE_PATTERN_REGEX.captures(spec) {
            let pattern = &pattern_caps["pattern"];

            return Ok(RuleSpec::Pattern(pattern.into()));
        }

        let compound = parse_compound(spec);
        if compound.len() > 0 {
            return Ok(RuleSpec::Compound(compound));
        }

        Err("Could not parse rule specification".into())
    }
}

struct Rules {
    rules: HashMap<usize, RuleSpec>,
    depth: i32,
}

impl Rules {
    fn new(depth: i32) -> Self {
        Self {
            rules: HashMap::new(),
            depth,
        }
    }

    fn matches(&self, candidate: &str) -> bool {
        if let Some(rule) = self.rules.get(&0) {
            let pattern = self.compile_rule(0, rule, self.depth);

            let pattern = format!("^{}$", pattern);
            let regex = Regex::new(&pattern).unwrap();
            regex.is_match(candidate)
        } else {
            false
        }
    }

    fn compile_rule(&self, id: usize, rulespec: &RuleSpec, depth: i32) -> String {
        match rulespec {
            RuleSpec::Pattern(pattern) => pattern.clone(),
            RuleSpec::Compound(ids) => {
                if ids.contains(&id) && depth == 0 {
                    String::new()
                } else {
                    ids.iter()
                        .map(|i| {
                            if let Some(r) = self.rules.get(&i) {
                                self.compile_rule(*i, r, depth - 1)
                            } else {
                                String::new()
                            }
                        })
                        .collect()
                }
            }

            RuleSpec::Or((left, right)) => {
                let left_pattern: String =
                    self.compile_rule(id, &RuleSpec::Compound(left.clone()), depth);
                let right_pattern: String =
                    self.compile_rule(id, &RuleSpec::Compound(right.clone()), depth);

                if left_pattern.is_empty() {
                    right_pattern
                } else if right_pattern.is_empty() {
                    left_pattern
                } else {
                    format!("({}|{})", left_pattern, right_pattern)
                }
            }
        }
    }

    fn add_rule(&mut self, raw_rule: &str) -> Result<(), String> {
        if let Some(caps) = PARSE_RULE_REGEX.captures(raw_rule) {
            let id: usize = (&caps["id"]).parse().unwrap_or_default();
            let spec = &caps["spec"];

            let spec: RuleSpec = spec.parse()?;
            self.rules.insert(id, spec);

            return Ok(());
        }

        Err("Could not parse rule".into())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day_19_matches_simple_rule() -> Result<(), String> {
        let mut rules = Rules::new(1);
        rules.add_rule("0: \"a\"")?;
        let message = "a";

        assert!(rules.matches(message));

        Ok(())
    }

    #[test]
    fn day_19_does_not_match_simple_rule() -> Result<(), String> {
        let mut rules = Rules::new(1);
        rules.add_rule("0: \"a\"")?;
        let message = "bb";

        assert!(!rules.matches(message));

        Ok(())
    }

    #[test]
    fn day_19_matches_compound_rule() -> Result<(), String> {
        let mut rules = Rules::new(1);
        rules.add_rule("0: 1 2")?;
        rules.add_rule("1: \"a\"")?;
        rules.add_rule("2: \"b\"")?;
        let message = "ab";

        assert!(rules.matches(message));

        Ok(())
    }

    #[test]
    fn day_19_matches_or_rule() -> Result<(), String> {
        let mut rules = Rules::new(1);
        rules.add_rule("0: 1 2 | 3 4")?;
        rules.add_rule("1: \"a\"")?;
        rules.add_rule("2: \"b\"")?;
        rules.add_rule("3: \"c\"")?;
        rules.add_rule("4: \"d\"")?;

        assert!(rules.matches("ab"));
        assert!(rules.matches("cd"));

        Ok(())
    }

    #[test]
    fn day_19_counts_valid_messages() {
        let data = vec![
            "0: 4 1 5",
            "1: 2 3 | 3 2",
            "2: 4 4 | 5 5",
            "3: 4 5 | 5 4",
            "4: \"a\"",
            "5: \"b\"",
            "",
            "ababbb",
            "bababa",
            "abbbab",
            "aaabbb",
            "aaaabbb",
        ];

        if let Ok((rules, messages)) = parse_data(&data) {
            let count = messages.iter().filter(|m| rules.matches(m)).count();
            assert_eq!(2, count);
        }
    }

    #[test]
    fn day_19_counts_valid_messages_with_looping_rules() {
        let data = vec![
            "42: 9 14 | 10 1",
            "9: 14 27 | 1 26",
            "10: 23 14 | 28 1",
            "1: \"a\"",
            "11: 42 31",
            // "11: 42 31 | 42 11 31",
            "5: 1 14 | 15 1",
            "19: 14 1 | 14 14",
            "12: 24 14 | 19 1",
            "16: 15 1 | 14 14",
            "31: 14 17 | 1 13",
            "6: 14 14 | 1 14",
            "2: 1 24 | 14 4",
            "0: 8 11",
            "13: 14 3 | 1 12",
            "15: 1 | 14",
            "17: 14 2 | 1 7",
            "23: 25 1 | 22 14",
            "28: 16 1",
            "4: 1 1",
            "20: 14 14 | 1 15",
            "3: 5 14 | 16 1",
            "27: 1 6 | 14 18",
            "14: \"b\"",
            "21: 14 1 | 1 14",
            "25: 1 1 | 1 14",
            "22: 14 14",
            "8: 42",
            // "8: 42 | 42 8",
            "26: 14 22 | 1 20",
            "18: 15 15",
            "7: 14 5 | 1 21",
            "24: 14 1",
            "",
            "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa",
            "bbabbbbaabaabba",
            "babbbbaabbbbbabbbbbbaabaaabaaa",
            "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
            "bbbbbbbaaaabbbbaaabbabaaa",
            "bbbababbbbaaaaaaaabbababaaababaabab",
            "ababaaaaaabaaab",
            "ababaaaaabbbaba",
            "baabbaaaabbaaaababbaababb",
            "abbbbabbbbaaaababbbbbbaaaababb",
            "aaaaabbaabaaaaababaa",
            "aaaabbaaaabbaaa",
            "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
            "babaaabbbaaabaababbaabababaaab",
            "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
        ];

        if let Ok((rules, messages)) = parse_data_with_looping_rules(&data) {
            let count = messages.iter().filter(|m| rules.matches(m)).count();
            assert_eq!(12, count);
        }
    }
}

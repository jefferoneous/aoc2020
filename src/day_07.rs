use regex::Regex;
use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref PARSE_RULE_REGEX: Regex = Regex::new(r"(?P<amount>\d+) (?P<color>.*)").unwrap();
    static ref SPLIT_RULE_REGEX: Regex = Regex::new(r" bags?[.,] ?").unwrap();
}

struct RuleMap {
    rules: HashMap<String, HashMap<String, u32>>,
}

impl RuleMap {
    fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    fn count_required_bags(&self, color: &str) -> u32 {
        self.rules
            .get(color)
            .unwrap()
            .iter()
            .fold(0, |total, (k, count)| {
                total + count + count * self.count_required_bags(k)
            })
    }

    fn must_contain_bag(&self, key: &str, color: &str) -> bool {
        self.rules
            .get(key)
            .unwrap()
            .keys()
            .any(|k| k == color || self.must_contain_bag(k, color))
    }

    fn find_containers(&self, color: &str) -> HashSet<String> {
        self.rules
            .keys()
            .filter(|k| self.must_contain_bag(k, color))
            .map(|k| k.to_owned())
            .collect()
    }

    fn parse_rule(&mut self, raw_rule: &str) {
        let parts: Vec<&str> = raw_rule.split(" bags contain ").collect();

        let container = parts[0].to_string();

        if parts[1].contains("no other bags") {
            self.rules.insert(container, HashMap::new());
        } else {
            let mut map: HashMap<String, u32> = HashMap::new();

            for raw_contained in SPLIT_RULE_REGEX.split(parts[1]) {
                for caps in PARSE_RULE_REGEX.captures_iter(raw_contained) {
                    map.insert(
                        (&caps["color"]).to_string(),
                        (&caps["amount"]).parse().unwrap_or_default(),
                    );
                }
            }

            if map.len() > 0 {
                self.rules.insert(container, map);
            }
        }
    }
}

pub fn part_one(data: &[String]) {
    let mut rule_map = RuleMap::new();
    for line in data {
        rule_map.parse_rule(line);
    }
    let count = rule_map.find_containers("shiny gold").len();
    println!(
        "Bags that can contain at least one shiny gold bag: {}",
        count
    );
}

pub fn part_two(data: &[String]) {
    let mut rule_map = RuleMap::new();
    for line in data {
        rule_map.parse_rule(line);
    }
    let count = rule_map.count_required_bags("shiny gold");
    println!("Bags that shiny gold bag requires: {}", count);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day_07_parses_rule_with_empty_bag() {
        let raw_rule = "dotted black bags contain no other bags.";
        let mut rule_map = RuleMap::new();

        rule_map.parse_rule(raw_rule);

        assert!(rule_map.rules.contains_key("dotted black"));
        assert_eq!(rule_map.rules.get("dotted black").unwrap().len(), 0);
    }

    #[test]
    fn day_07_parses_rule_with_one_contained_bag() {
        let raw_rule = "bright white bags contain 1 shiny gold bag.";
        let mut rule_map = RuleMap::new();

        rule_map.parse_rule(raw_rule);

        let mut result_map: HashMap<String, u32> = HashMap::new();
        result_map.insert("shiny gold".to_string(), 1);

        assert!(rule_map.rules.contains_key("bright white"));
        assert_eq!(rule_map.rules.get("bright white").unwrap().len(), 1);
        assert_eq!(*rule_map.rules.get("bright white").unwrap(), result_map);
    }

    #[test]
    fn day_07_parses_rule_with_multiple_contained_bags() {
        let raw_rule = "dark orange bags contain 3 bright white bags, 4 muted yellow bags.";
        let mut rule_map = RuleMap::new();

        rule_map.parse_rule(raw_rule);
        let mut result_map: HashMap<String, u32> = HashMap::new();
        result_map.insert("bright white".to_string(), 3);
        result_map.insert("muted yellow".to_string(), 4);

        assert!(rule_map.rules.contains_key("dark orange"));
        assert_eq!(rule_map.rules.get("dark orange").unwrap().len(), 2);
        assert_eq!(*rule_map.rules.get("dark orange").unwrap(), result_map);
    }

    fn generate_test_rule_map_01() -> RuleMap {
        let raw_rules = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags.",
        ];

        let mut rule_map = RuleMap::new();

        for raw_rule in raw_rules {
            rule_map.parse_rule(raw_rule);
        }

        rule_map
    }

    #[test]
    fn day_07_identifies_containers() {
        let rule_map = generate_test_rule_map_01();

        let mut expected: HashSet<String> = HashSet::new();
        expected.insert("bright white".to_string());
        expected.insert("muted yellow".to_string());
        expected.insert("dark orange".to_string());
        expected.insert("light red".to_string());

        assert_eq!(rule_map.find_containers("shiny gold"), expected);
    }

    fn generate_test_rule_map_02() -> RuleMap {
        let raw_rules = vec![
            "shiny gold bags contain 2 dark red bags.",
            "dark red bags contain 2 dark orange bags.",
            "dark orange bags contain 2 dark yellow bags.",
            "dark yellow bags contain 2 dark green bags.",
            "dark green bags contain 2 dark blue bags.",
            "dark blue bags contain 2 dark violet bags.",
            "dark violet bags contain no other bags.",
        ];

        let mut rule_map = RuleMap::new();

        for raw_rule in raw_rules {
            rule_map.parse_rule(raw_rule);
        }

        rule_map
    }

    #[test]
    fn day_07_counts_bags_required_for_specified_bag() {
        let rule_map = generate_test_rule_map_02();

        assert_eq!(rule_map.count_required_bags("shiny gold"), 126);
    }
}

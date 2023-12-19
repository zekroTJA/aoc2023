use lib::*;
use std::{collections::HashMap, ops::RangeInclusive};

#[derive(Clone, Debug)]
struct Part {
    ratings: HashMap<char, usize>,
}

impl Part {
    fn parse(line: &str) -> Self {
        let ratings = line[1..line.len() - 1]
            .split(',')
            .map(|r| (r.chars().next().unwrap(), r[2..].parse().unwrap()))
            .collect();
        Self { ratings }
    }

    fn value(&self) -> usize {
        self.ratings.values().sum()
    }
}

#[derive(Debug)]
enum Filter {
    Lower(usize),
    Higher(usize),
}

#[derive(Debug)]
struct Rule {
    field: char,
    filter: Filter,
    next: String,
}

impl Rule {
    fn matches(&self, p: &Part) -> Option<&str> {
        let Some(&rating) = p.ratings.get(&self.field) else {
            return None;
        };

        let ok = match self.filter {
            Filter::Higher(v) => rating > v,
            Filter::Lower(v) => rating < v,
        };

        if ok {
            Some(self.next.as_str())
        } else {
            None
        }
    }
}

impl Rule {
    fn parse(v: &str) -> Self {
        let mut chars = v.chars();
        let field = chars.next().unwrap();

        let (val, next) = v[2..].split_once(':').unwrap();

        let filter = match chars.next().unwrap() {
            '>' => Filter::Higher(val.parse().unwrap()),
            '<' => Filter::Lower(val.parse().unwrap()),
            x => panic!("invalid filter character: {x}"),
        };

        Self {
            field,
            filter,
            next: next.into(),
        }
    }
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
    fallback: String,
}

impl Workflow {
    fn parse(line: &str) -> (String, Self) {
        let (key, rules) = line[..line.len() - 1].split_once('{').unwrap();

        let split: Vec<_> = rules.split(',').collect();

        let rules = split[..split.len() - 1]
            .iter()
            .map(|&r| Rule::parse(r))
            .collect();

        let fallback = split.last().unwrap().to_string();

        let w = Self { rules, fallback };
        (key.into(), w)
    }

    fn process(&self, part: &Part) -> &str {
        for r in &self.rules {
            if let Some(next) = r.matches(part) {
                return next;
            }
        }

        self.fallback.as_str()
    }
}

fn count(
    workflows: &HashMap<String, Workflow>,
    ranges: &HashMap<char, RangeInclusive<usize>>,
    curr: &str,
) -> usize {
    match curr {
        "R" => 0,
        "A" => ranges.values().map(|r| r.clone().count()).product(),
        curr => {
            let workflow = workflows.get(curr).unwrap();
            let mut total = 0;

            let mut current_ranges = ranges.clone();

            for rule in &workflow.rules {
                let r = current_ranges.get(&rule.field).unwrap();
                let (incl, excl) = match rule.filter {
                    Filter::Lower(n) => (*r.start()..=n - 1, n..=*r.end()),
                    Filter::Higher(n) => (n + 1..=*r.end(), *r.start()..=n),
                };

                if !incl.is_empty() {
                    let mut new_ranges = current_ranges.clone();
                    new_ranges.insert(rule.field, incl);
                    total += count(workflows, &new_ranges, &rule.next);
                }

                if excl.is_empty() {
                    break;
                }

                current_ranges.insert(rule.field, excl);
            }

            if current_ranges.values().any(|r| !r.is_empty()) {
                total += count(workflows, &current_ranges, &workflow.fallback);
            }

            total
        }
    }
}

fn main() {
    let input: String = lib::read_input!();

    let (workflows, parts) = input.split_once("\n\n").unwrap();

    let parts: Vec<_> = parts.split('\n').map(Part::parse).collect();
    let workflows: HashMap<_, _> = workflows.split('\n').map(Workflow::parse).collect();
    let start_workflow = workflows.get("in").unwrap();

    let mut accepted = vec![];

    for part in parts {
        let mut next = start_workflow.process(&part);
        while next != "R" && next != "A" {
            next = workflows.get(next).unwrap().process(&part);
        }
        if next == "A" {
            accepted.push(part);
        }
    }

    let p1: usize = accepted.iter().map(|p| p.value()).sum();
    p1!(p1);

    // --------------------------------------------------------------------------------------

    let p2 = count(
        &workflows,
        &[
            ('x', 1..=4000),
            ('m', 1..=4000),
            ('a', 1..=4000),
            ('s', 1..=4000),
        ]
        .into(),
        "in",
    );
    p2!(p2);
}

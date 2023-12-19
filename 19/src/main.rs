use std::{fs::read_to_string, collections::BTreeMap, ops::{Range, RangeInclusive}};

enum Operator {
    GreaterThan,
    LessThan
}

struct Rule {
    field: String,
    operator: Operator,
    value: i32,
    target: String
}

impl Rule {
    fn from_str(s: &str) -> Self {
        let (rule, target) = s.split_once(":").unwrap();
        if let Some((field, value)) = rule.split_once("<") {
            Self {
                field: field.to_string(),
                operator: Operator::LessThan,
                value: value.parse().unwrap(),
                target: target.to_string()
            }
        } else {
            if let Some((field, value)) = rule.split_once(">") {
                Self {
                    field: field.to_string(),
                    operator: Operator::GreaterThan,
                    value: value.parse().unwrap(),
                    target: target.to_string()
                }
            } else {
                panic!("Invalid rule: {}", s);
            }
        }
    }
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
    catchall_target: String
}

impl Workflow {
    fn from_str(s: &str) -> Self {
        let mut rules = Vec::new();
        let (name,rules_str) = s.split_once("{").unwrap();
        let raw_rules = rules_str.split(",").collect::<Vec<_>>();
        for rule in raw_rules.iter().take(raw_rules.len() - 1) {
            let rule = Rule::from_str(rule);
            rules.push(rule);
        }
        Self {
            name: name.to_string(),
            rules,
            catchall_target: raw_rules.last().unwrap().strip_suffix("}").unwrap().to_owned()
        }
    }

    fn find_next_target(&self, input: &BTreeMap<String, i32>) -> String {
        for rule in self.rules.iter() {
            let value = input.get(&rule.field).unwrap();
            match rule.operator {
                Operator::GreaterThan => {
                    if value > &rule.value {
                        return rule.target.clone();
                    }
                },
                Operator::LessThan => {
                    if value < &rule.value {
                        return rule.target.clone();
                    }
                }
            }
        }
        self.catchall_target.clone()
    }
}

fn find_num_accepted(workflows: &BTreeMap<String, Workflow>, current_workflow: &str, ranges: BTreeMap<String, Range<i32>>) -> u64 {
    if current_workflow == "A" {
        let result = ranges.iter().fold(1, |acc, (_, f)| acc * (f.len() + 1) as u64);
        return result;
    }
    if current_workflow == "R" {
        return 0;
    }
    let mut result = 0;
    let current_workflow = workflows.get(current_workflow).unwrap();
    let mut remaining_ranges = ranges.clone();
    for rule in current_workflow.rules.iter() {
        let mut new_ranges = remaining_ranges.clone();
        match rule.operator {
            Operator::GreaterThan => {
                let range = &remaining_ranges[&rule.field];
                if range.start > rule.value {
                    return result + find_num_accepted(workflows, &rule.target, remaining_ranges);
                } else {
                    if range.end > rule.value {
                        new_ranges.insert(rule.field.clone(), rule.value+1..range.end);
                        remaining_ranges.insert(rule.field.clone(), range.start..rule.value);
                    }
                }
            },
            Operator::LessThan => {
                let range = &remaining_ranges[&rule.field];
                if range.end < rule.value {
                    return result + find_num_accepted(workflows, &rule.target, remaining_ranges);
                } else {
                    if range.start < rule.value {
                        new_ranges.insert(rule.field.clone(), range.start..rule.value-1);
                        remaining_ranges.insert(rule.field.clone(), rule.value..range.end);
                    }
                }
            }
        }
        result += find_num_accepted(workflows, &rule.target, new_ranges);
    }
    result += find_num_accepted(workflows, &current_workflow.catchall_target, remaining_ranges);

    result
}

fn main() {
    let input_str = read_to_string("input.txt").unwrap();

    let (workflows_str, input_str) = input_str.split_once("\n\n").unwrap();

    let workflows = workflows_str.split("\n")
                .map(|s| {let w = Workflow::from_str(s); (w.name.clone(), w)})
                .collect::<BTreeMap<String, Workflow>>();


    let mut result = 0;

    for mut in_line in input_str.lines() {
        let mut input = BTreeMap::new();
        in_line = in_line.strip_prefix("{").unwrap();
        in_line = in_line.strip_suffix("}").unwrap();
        in_line.split(",").for_each(|s| {
            let (key, value) = s.split_once("=").unwrap();
            input.insert(key.to_string(), value.parse::<i32>().unwrap());
        });

        let mut current_workflow = workflows.get("in").unwrap();

        loop {
            let next_target = current_workflow.find_next_target(&input);
            if next_target == "A" || next_target == "R" {
                if next_target == "A" {
                    result += input.values().sum::<i32>();
                }
                break;
            }
            current_workflow = workflows.get(&next_target).unwrap();
        }
    }

    println!("Result 1: {}", result);

    let mut ranges = BTreeMap::new();
    for key in ["x", "a", "m", "s"] {
        ranges.insert(key.to_string(), 1..4000);
    }

    let result = find_num_accepted(&workflows, "in", ranges);

    println!("Result 2: {}", result);
    
}
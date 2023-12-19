use std::collections::{HashMap, VecDeque};

#[derive(Debug, Copy, Clone)]
struct Range {
    start: i64,
    end: i64, // inclusive
}

impl Range {
    fn get_interval(&self) -> i64 {
        self.end - self.start + 1
    }
}

#[derive(Debug, Copy, Clone)]
struct Part(Range, Range, Range, Range); // x, m, a, s

impl Part {
    fn get_field_idx(field: &str) -> usize {
        match field {
            "x" => 0,
            "m" => 1,
            "a" => 2,
            "s" => 3,
            _ => panic!("oh no"),
        }
    }

    fn get(&self, index: usize) -> Range {
        match index {
            0 => self.0,
            1 => self.1,
            2 => self.2,
            3 => self.3,
            _ => panic!("oh no"),
        }
    }

    fn set(&mut self, index: usize, r: Range) {
        match index {
            0 => self.0 = r,
            1 => self.1 = r,
            2 => self.2 = r,
            3 => self.3 = r,
            _ => panic!("oh no"),
        }
    }

    fn get_product(&self) -> i64 {
        self.0.get_interval() * self.1.get_interval() * self.2.get_interval() * self.3.get_interval()
    }
}

#[derive(Debug)]
struct Rule {
    target: String,
    sinkhole: bool,
    field_idx: usize,
    predicate: String,
}


impl Rule {
    fn process_predicate(&self, p: Part) -> (Option<Part>, Option<Part>) { // (accepted part, continue part)
        if self.sinkhole {
            return (Some(p), None);
        }

        let value = p.get(self.field_idx);
        let operator = self.predicate.chars().next().unwrap();
        let predicate_value = self.predicate[1..].parse::<i64>().unwrap();
        match operator {
            '<' => {
                if value.end < predicate_value {
                    // match the entire thing
                    (Some(p), None)
                } else if value.start < predicate_value && predicate_value <= value.end {
                    // in the middle
                    let new_range_accepted = Range {
                        start: value.start,
                        end: predicate_value - 1,
                    };

                    let mut p1 = p;
                    p1.set(self.field_idx, new_range_accepted);

                    let new_range_continue = Range {
                        start: predicate_value,
                        end: value.end,
                    };

                    let mut p2 = p;
                    p2.set(self.field_idx, new_range_continue);
                    (Some(p1), Some(p2))
                } else {
                    // match on the left
                    (None, Some(p))
                }
            }
            '>' => {
                if value.start > predicate_value {
                    // match the entire thing
                    (Some(p), None)
                } else if value.start <= predicate_value && predicate_value < value.end {
                    // in the middle
                    let new_range_accepted = Range {
                        start: predicate_value + 1,
                        end: value.end,
                    };

                    let mut p1 = p;
                    p1.set(self.field_idx, new_range_accepted);

                    let new_range_continue = Range {
                        start: value.start,
                        end: predicate_value,
                    };

                    let mut p2 = p;
                    p2.set(self.field_idx, new_range_continue);
                    (Some(p1), Some(p2))
                } else {
                    // match on the right
                    (None, Some(p))
                }
            }
            _ => panic!("oh no"),
        }
    }
}

#[derive(Debug)]
struct Workflow {
    // name is part of the key
    rules: Vec<Rule>,
}

impl Workflow {
    fn match_part(&self, part: Part) -> Vec<(Part, String)> {
        let mut curr = vec![part];
        let mut next = Vec::new();
        let mut done = Vec::new();
        for rule in &self.rules {
            next.clear();
            for p in &curr {
                let (accepted_range, continue_range) = rule.process_predicate(*p);
                if let Some(x) = accepted_range {
                    done.push((x, rule.target.clone()));
                }

                if let Some(x) = continue_range {
                    next.push(x);
                }
            }

            curr = next.clone();
        }

        done
    }
}

fn parse(workflow_map: &mut HashMap<String, Workflow>, line: &str) {
    let (workflow_name, rule_list) = line.split_once('{').unwrap();

    let mut rules = Vec::new();
    for r in rule_list[..rule_list.len() - 1].split(',') {
        if let Some((predicate, target)) = r.split_once(':') {
            let field = &predicate[0..1];
            let p = &predicate[1..];
            let new_rule = Rule {
                target: target.to_string(),
                sinkhole: false,
                field_idx: Part::get_field_idx(field),
                predicate: p.to_string(),
            };
            rules.push(new_rule);
        } else {
            let new_rule = Rule {
                target: r.to_string(),
                sinkhole: true,
                field_idx: 0,
                predicate: "".to_string(),
            };
            rules.push(new_rule);
        }
    }

    let new_workflow = Workflow {
        rules
    };

    workflow_map.insert(workflow_name.to_string(), new_workflow);
}

fn main() {
    let input = include_str!("input.txt");
    let (workflows, _) = input.split_once("\n\n").unwrap();

    let mut workflow_map: HashMap<String, Workflow> = HashMap::new();
    let _: () = workflows.lines().map(|s| parse(&mut workflow_map, s)).collect();
    let initial_part = (Part(
        Range {
            start: 1,
            end: 4000,
        },
        Range {
            start: 1,
            end: 4000,
        },
        Range {
            start: 1,
            end: 4000,
        },
        Range {
            start: 1,
            end: 4000,
        },
    ), "in".to_string());

    let mut accepted: Vec<Part> = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(initial_part);
    while let Some((part, current_workflow)) = queue.pop_front() {
        let workflow = workflow_map.get(&current_workflow).unwrap();
        let result = workflow.match_part(part);
        for ref n @ (part, ref next_workflow) in result {
            if next_workflow == &"A".to_string() {
                accepted.push(part);
            } else if next_workflow == &"R".to_string() {
                continue; // ignore this
            } else {
                queue.push_back(n.clone());
            }
        }
    }

    let ans = accepted.iter().fold(0, |acc, &p| acc + p.get_product());
    println!("{:?}", ans);
}
use std::collections::HashMap;

#[derive(Debug)]
struct Part(i64, i64, i64, i64); // x, m, a, s

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

    fn get(&self, index: usize) -> i64 {
        match index {
            0 => self.0,
            1 => self.1,
            2 => self.2,
            3 => self.3,
            _ => panic!("oh no"),
        }
    }

    fn sum(&self) -> i64 {
        self.0 + self.1 + self.2 + self.3
    }
}

#[derive(Debug)]
struct Rule {
    target: String,
    sinkhole: bool,
    field_idx: usize,
    // I am too lazy to use option here
    predicate: String,
}

impl Rule {
    fn check_predicate(&self, p: &Part) -> bool {
        if self.sinkhole {
            return true;
        }

        let value = p.get(self.field_idx);
        let operator = self.predicate.chars().next().unwrap();
        let predicate_value = self.predicate[1..].parse::<i64>().unwrap();
        match operator {
            '<' => value < predicate_value,
            '>' => value > predicate_value,
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
    fn match_part(&self, part: &Part) -> Option<String> {
        for rule in &self.rules {
            if rule.check_predicate(part) {
                return Some(rule.target.to_string());
            }
        }

        None
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

fn parse_part(line: &str) -> Part {
    let l: Vec<i64> = (&line[1..line.len() - 1])
        .split(',')
        .map(|s| s.split('=').last().unwrap().parse::<i64>().unwrap())
        .collect();

    Part(l[0], l[1], l[2], l[3])
}

fn main() {
    let input = include_str!("input.txt");
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let parts: Vec<Part> = parts.lines().map(parse_part).collect();

    let mut workflow_map: HashMap<String, Workflow> = HashMap::new();
    let _: () = workflows.lines().map(|s| parse(&mut workflow_map, s)).collect();

    // part 1
    let mut accepted: Vec<&Part> = Vec::new();
    for p in &parts {
        let mut current_part = (p, "in".to_string());
        let result = loop {
            let workflow = workflow_map.get(&current_part.1).unwrap();
            let result = workflow.match_part(current_part.0).unwrap();
            current_part = (current_part.0, result);

            if current_part.1 == "A".to_string() {
                break Some(current_part.0);
            } else if current_part.1 == "R".to_string() {
                break None;
            }
        };

        if let Some(p) = result {
            accepted.push(p);
        }
    }

    let ans = accepted.iter().fold(0, |acc, &p| acc + p.sum());
    println!("{:?}", ans);
}
use std::collections::{HashMap, VecDeque};

use crate::Module::{Broadcaster, Conjunction, FlipFlop};
use crate::Pulse::{High, Low};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone)]
enum Module {
    Broadcaster {
        targets: Vec<String>,
    },
    Conjunction {
        memories: HashMap<String, Pulse>,
        targets: Vec<String>,
    },
    FlipFlop {
        state: bool,
        targets: Vec<String>,
    },
}

impl Module {
    fn process(&mut self, source: String, s: Pulse) -> Vec<(String, Pulse)> {
        match self {
            Broadcaster { targets } => {
                targets.clone().into_iter().map(|id| (id, s)).collect()
            }
            Conjunction { memories, targets } => {
                memories.insert(source, s);
                let is_all_high = memories.iter().map(|(_, pulse)| pulse).all(|&s| s == High);
                let send_pulse = if is_all_high { Low } else { High };
                targets.clone().into_iter().map(|id| (id, send_pulse)).collect()
            }
            FlipFlop { targets, state } => {
                match s {
                    Low => {
                        if *state {
                            *state = false;
                            // send low pulse
                            targets.clone().into_iter().map(|id| (id, Low)).collect()
                        } else {
                            *state = true;
                            targets.clone().into_iter().map(|id| (id, High)).collect()
                        }
                    }
                    High => Vec::new(),
                }
            }
        }
    }
}

fn parse(line: &str, map: &mut HashMap<String, Module>) {
    let (module, targets) = line.split_once("->").unwrap();
    let t: Vec<String> = targets.split(',').map(|s| s.trim().to_string()).collect();
    let (id, m) = match module.chars().next().unwrap() {
        '%' => {
            let id = module.trim()[1..].to_string();
            (id, FlipFlop {
                state: false,
                targets: t,
            })
        }
        '&' => {
            let id = module.trim()[1..].to_string();
            (id, Conjunction {
                memories: HashMap::new(), // to be initialized later
                targets: t,
            })
        }
        _ => (module.trim().to_string(), Broadcaster {
            targets: t,
        }),
    };

    map.insert(id, m);
}

fn main() {
    let input = include_str!("input.txt");
    let mut map = HashMap::new();
    let _: () = input.lines().map(|l| parse(l, &mut map)).collect();

    // create the map for every conjunction target
    let all_conjunctions: Vec<String> = map.iter().filter(|(_, m)| {
        if let Conjunction { .. } = m {
            true
        } else {
            false
        }
    })
        .map(|(id, _)| id.clone())
        .collect();

    map.clone().iter().for_each(|(id, m)| {
        match m {
            FlipFlop { targets, .. } | Conjunction { targets, .. } => {
                for t in targets {
                    if all_conjunctions.contains(&t) {
                        let c = map.get_mut(t).unwrap();
                        if let Conjunction { ref mut memories, .. } = c {
                            memories.insert(id.clone(), Low);
                        }
                    }
                }
            }
            _ => (),
        }
    });

    let mut q = VecDeque::new();

    // first try brute force without any config encoding + cycle detection
    // let mut low_counter: i64 = 0;
    // let mut high_counter: i64 = 0;

    let mut button_press_counter: i64 = 0;

    // send once
    loop {
        q.push_back(("button".to_string(), "broadcaster".to_string(), Low));
        button_press_counter += 1;
        while let Some((source, destination, signal)) = q.pop_front() {
            // part 2; wtf cannot be brute forced
            // "ds" "zp" 3733
            // "sb" "zp" 3797
            // "hf" "zp" 3877
            // "nd" "zp" 3917
            // check the investigation.txt how do I get the result
            if destination == "zp".to_string() && signal == High {
                println!("{:?} {:?} {:?}", source, destination, button_press_counter);
                // break 'outer;
            }
            // match signal {
            //     Low => low_counter += 1,
            //     High => high_counter += 1,
            // }

            let sent_module = map.get_mut(&destination);
            if let Some(m) = sent_module {
                let next_pulse = m.process(source.clone(), signal);
                for (next, pulse) in next_pulse {
                    q.push_back((destination.clone(), next, pulse));
                }
            }
            // println!("{:?}", q);
        }
    }
    // println!("{:?} {:?} {:?}", low_counter, high_counter, low_counter * high_counter);
}
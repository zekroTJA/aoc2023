use lib::*;
use num::Integer;
use std::{cell::RefCell, collections::HashMap};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Pulse {
    High,
    Low,
}

struct Broadcaster {
    outputs: Vec<String>,
}

struct FlipFlop {
    on: bool,
    outputs: Vec<String>,
}

struct Conjunction {
    states: HashMap<String, Pulse>,
    outputs: Vec<String>,
}

trait Module {
    fn receive(&mut self, from: &str, pulse: Pulse) -> Option<Pulse>;
    fn outputs(&self) -> Vec<String>;
    fn set_inputs(&mut self, _inputs: &[String]) {}
    fn is_conjunction(&self) -> bool {
        false
    }
}

impl Module for Broadcaster {
    fn receive(&mut self, _: &str, pulse: Pulse) -> Option<Pulse> {
        Some(pulse)
    }

    fn outputs(&self) -> Vec<String> {
        self.outputs.to_vec()
    }
}

impl Module for FlipFlop {
    fn receive(&mut self, _: &str, pulse: Pulse) -> Option<Pulse> {
        match pulse {
            Pulse::High => None,
            Pulse::Low => {
                self.on = !self.on;
                Some(if self.on { Pulse::High } else { Pulse::Low })
            }
        }
    }

    fn outputs(&self) -> Vec<String> {
        self.outputs.to_vec()
    }
}

impl Module for Conjunction {
    fn receive(&mut self, from: &str, pulse: Pulse) -> Option<Pulse> {
        self.states.insert(from.to_string(), pulse);
        if self.states.values().all(|v| v == &Pulse::High) {
            Some(Pulse::Low)
        } else {
            Some(Pulse::High)
        }
    }

    fn outputs(&self) -> Vec<String> {
        self.outputs.to_vec()
    }

    fn set_inputs(&mut self, inputs: &[String]) {
        for i in inputs {
            self.states.insert(i.clone(), Pulse::Low);
        }
    }

    fn is_conjunction(&self) -> bool {
        true
    }
}

fn parse_module(line: &str) -> (String, RefCell<Box<dyn Module>>) {
    let (ident, outputs) = line.split_once(" -> ").unwrap();

    let outputs = outputs.split(", ").map(|s| s.to_string()).collect();

    match ident.chars().next().unwrap() {
        'b' => (
            ident.to_string(),
            RefCell::new(Box::new(Broadcaster { outputs })),
        ),
        '%' => (
            ident[1..].to_string(),
            RefCell::new(Box::new(FlipFlop { outputs, on: false })),
        ),
        '&' => (
            ident[1..].to_string(),
            RefCell::new(Box::new(Conjunction {
                outputs,
                states: HashMap::new(),
            })),
        ),
        i => panic!("unexpected ident: {i}"),
    }
}

fn construct_modules_map(input: &str) -> HashMap<String, RefCell<Box<dyn Module>>> {
    let modules: HashMap<_, _> = input.split('\n').map(parse_module).collect();

    for (k, v) in modules.iter().filter(|(_, v)| v.borrow().is_conjunction()) {
        let inputs: Vec<_> = modules
            .iter()
            .filter(|(_, v)| v.borrow().outputs().contains(k))
            .map(|(s, _)| s.to_string())
            .collect();
        v.borrow_mut().set_inputs(&inputs);
    }

    modules
}

fn main() {
    let input: String = lib::read_input!();

    let modules = construct_modules_map(&input);

    let mut low = 0;
    let mut high = 0;

    for _ in 0..1000 {
        let mut queue = vec![("button".to_string(), "broadcaster".to_string(), Pulse::Low)];

        while let Some((from, to, pulse)) = queue.pop() {
            match pulse {
                Pulse::High => high += 1,
                Pulse::Low => low += 1,
            }

            let Some(next) = modules.get(&to) else {
                continue;
            };

            let res = { next.borrow_mut().receive(&from, pulse) };
            if let Some(res) = res {
                let outs = { next.borrow().outputs() };
                for o in outs {
                    queue.insert(0, (to.clone(), o, res));
                }
            }
        }
    }

    p1!(low * high);

    // -------------------------------------------------------------

    let modules = construct_modules_map(&input);

    let feed = modules
        .iter()
        .find(|(_, v)| v.borrow().outputs().contains(&"rx".into()))
        .map(|(k, _)| k)
        .unwrap();

    let mut seen: HashMap<_, _> = modules
        .iter()
        .filter(|(_, v)| v.borrow().outputs().contains(feed))
        .map(|(k, _)| (k.clone(), 0usize))
        .collect();

    let mut cycles = HashMap::new();

    'outer: for i in 1..usize::MAX {
        let mut queue = vec![("button".to_string(), "broadcaster".to_string(), Pulse::Low)];

        while let Some((from, to, pulse)) = queue.pop() {
            let Some(next) = modules.get(&to) else {
                continue;
            };

            if &to == feed && pulse == Pulse::High {
                *seen.get_mut(&from).unwrap() += 1;

                if !cycles.contains_key(&from) {
                    cycles.insert(from.clone(), i);
                }

                if seen.values().all(|v| *v >= 1) {
                    let p2 = cycles.values().fold(1usize, |g, c| g.lcm(c));
                    p2!(p2);
                    break 'outer;
                }
            }

            let res = { next.borrow_mut().receive(&from, pulse) };
            if let Some(res) = res {
                let outs = { next.borrow().outputs() };
                for o in outs {
                    queue.insert(0, (to.clone(), o, res));
                }
            }
        }
    }
}

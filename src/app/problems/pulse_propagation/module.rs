use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction
}

#[derive(Debug)]
pub struct ModuleConfiguration<'a> {
    type_map: HashMap<&'a str, ModuleType>,
    adjacency: HashMap<&'a str, Vec<&'a str>>,
    adjacency_reverse: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> ModuleConfiguration<'a> {
    pub fn from_str(s: &'a str) -> ModuleConfiguration<'a> {
        let mut module_configuration = ModuleConfiguration::new();
        for line in s.lines() {
            let (from, destinations) = line.split_once(" -> ").unwrap();
            let (module_type, from) = if from == "broadcaster" {
                (ModuleType::Broadcaster, from)
            } else {
                let (module_type, from) = from.split_at(1);
                let module_type = match module_type {
                    "%" => ModuleType::FlipFlop,
                    "&" => ModuleType::Conjunction,
                    _ => panic!("Invalid module type \"{module_type}\""),
                };
                (module_type, from)
            };
            let destinations: Vec<_> = destinations.split(", ").collect();

            module_configuration.insert_line(module_type, from, destinations);
        }

        module_configuration
    }

    pub fn into_modules(mut self) -> ModuleCollection<'a> {
        let mut module_collection = ModuleCollection::new();

        for (node, destinations) in self.adjacency {
            match self.type_map.get(node).unwrap() {
                ModuleType::Broadcaster => module_collection.insert_broadcaster(node, destinations),
                ModuleType::FlipFlop => module_collection.insert_flip_flop(node, destinations),
                ModuleType::Conjunction => module_collection.insert_conjunction(node, destinations, self.adjacency_reverse.remove(node).unwrap())
            }
        }

        module_collection
    }

    fn new() -> ModuleConfiguration<'a> {
        ModuleConfiguration {
            type_map: HashMap::new(),
            adjacency: HashMap::new(),
            adjacency_reverse: HashMap::new(),
        }
    }

    fn insert_line(&mut self, module_type: ModuleType, from: &'a str, destinations: Vec<&'a str>) {
        self.type_map.insert(from, module_type);
        for to in destinations.iter() {
            self.insert_reverse_adjacency(from, to);
        }
        self.adjacency.insert(from, destinations);
    }

    fn insert_reverse_adjacency(&mut self, from: &'a str, to: &'a str) {
        if let Some(map) = self.adjacency_reverse.get_mut(to) {
            map.push(from);
        } else {
            self.adjacency_reverse.insert(to, vec![from]);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pulse<'a> {
    High { from: &'a str, to: &'a str },
    Low { from: &'a str, to: &'a str },
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Module<'a> {
    Broadcaster { name: &'a str, destinations: Vec<&'a str> },
    FlipFlop { name: &'a str, destinations: Vec<&'a str>, active: bool },
    Conjunction { name: &'a str, destinations: Vec<&'a str>, inputs: HashMap<&'a str, bool> },
}

impl<'a> Module<'a> {
    fn receive_high_pulse(&mut self, from: &'a str) -> Vec<Pulse<'a>> {
        match self {
            Module::Broadcaster { name, destinations} => {
                destinations.iter()
                    .map(|to| Pulse::High { from: name, to })
                    .collect()
            },
            Module::FlipFlop { name: _, destinations: _, active: _ } => {
                vec![]
            },
            Module::Conjunction { name, destinations, inputs } => {
                *inputs.get_mut(from).unwrap() = true;

                if inputs.values().all(|x| *x) {
                    destinations.iter()
                        .map(|to| Pulse::Low { from: name, to })
                        .collect()
                } else {
                    destinations.iter()
                        .map(|to| Pulse::High { from: name, to })
                        .collect()
                }
            }
        }
    }

    fn receive_low_pulse(&mut self, from: &'a str) -> Vec<Pulse<'a>> {
        match self {
            Module::Broadcaster { name, destinations } => {
                destinations.iter()
                    .map(|to| Pulse::Low { from: name, to })
                    .collect()
            },
            Module::FlipFlop { name, destinations, active } => {
                if *active {
                    *active = false;
                    destinations.iter()
                        .map(|to| Pulse::Low { from: name, to })
                        .collect()
                } else {
                    *active = true;
                    destinations.iter()
                        .map(|to| Pulse::High { from: name, to })
                        .collect()
                }
            },
            Module::Conjunction { name, destinations, inputs } => {
                *inputs.get_mut(from).unwrap() = false;

                if inputs.values().all(|x| *x) {
                    destinations.iter()
                        .map(|to| Pulse::Low { from: name, to })
                        .collect()
                } else {
                    destinations.iter()
                        .map(|to| Pulse::High { from: name, to })
                        .collect()
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct ModuleCollection<'a> {
    module_map: HashMap<&'a str, Module<'a>>
}

impl<'a> ModuleCollection<'a> {
    pub fn click_button(&mut self, index: usize) -> (usize, usize) {
        let mut high_pulse_count = 0usize;
        let mut low_pulse_count = 0usize;
        let mut pulse_queue: VecDeque<Pulse<'a>> = VecDeque::from([Pulse::Low { from: "button", to: "broadcaster" }]);

        while let Some(pulse) = pulse_queue.pop_front() {
            let pulses = match pulse {
                Pulse::Low { from, to } => {
                    low_pulse_count += 1;

                    // Some heavy handed cheating here >_>
                    if to == "ln" {
                        leptos::logging::log!("ln hit at click {}", index);
                    }
                    if to == "db" {
                        leptos::logging::log!("db hit at click {}", index);
                    }
                    if to == "vq" {
                        leptos::logging::log!("vq hit at click {}", index);
                    }
                    if to == "tf" {
                        leptos::logging::log!("tf hit at click {}", index);
                    }

                    if let Some(module) = self.module_map.get_mut(to) {
                        module.receive_low_pulse(from)
                    } else {
                        vec![]
                    }
                },
                Pulse::High { from, to } => {
                    high_pulse_count += 1;
                    if let Some(module) = self.module_map.get_mut(to) {
                        module.receive_high_pulse(from)
                    } else {
                        vec![]
                    }
                }
            };

            for pulse in pulses {
                pulse_queue.push_back(pulse);
            }
        }

        (high_pulse_count, low_pulse_count)
    }

    fn new() -> ModuleCollection<'a> {
        ModuleCollection { module_map: HashMap::new() }
    }

    fn insert_broadcaster(&mut self, name: &'a str, destinations: Vec<&'a str>) {
        self.module_map.insert(name, Module::Broadcaster { name, destinations });
    }

    fn insert_flip_flop(&mut self, name: &'a str, destinations: Vec<&'a str>) {
        self.module_map.insert(name, Module::FlipFlop { name, destinations, active: false });
    }

    fn insert_conjunction(&mut self, name: &'a str, destinations: Vec<&'a str>, inputs: Vec<&'a str>) {
        self.module_map.insert(name, Module::Conjunction { name, destinations, inputs: inputs.into_iter().map(|i| (i, false)).collect() });
    }
}

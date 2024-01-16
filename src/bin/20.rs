use std::any::Any;
use std::collections::{vec_deque, HashMap, VecDeque};

advent_of_code::solution!(20);

trait Module {
    fn receive_pulse(&mut self, high: bool, sender: usize) -> Option<bool>;
    fn add_output(&mut self, other: usize);
    fn add_input(&mut self, other: usize);
    fn get_output_iter(&self) -> std::slice::Iter<'_, usize>;
    fn get_name(&self) -> String;
    fn as_any(&mut self) -> &mut dyn Any;
}

struct FlipFlop {
    id: usize,
    name: String,
    outputs: Vec<usize>,
    on: bool,
}

impl FlipFlop {
    fn new(id: usize, name: String) -> Self {
        Self {
            id,
            name,
            outputs: vec![],
            on: false,
        }
    }
}

impl Module for FlipFlop {
    fn receive_pulse(&mut self, high: bool, _sender: usize) -> Option<bool> {
        if high {
            return None;
        } else if self.on {
            self.on = false;
            return Some(false);
        } else {
            self.on = true;
            return Some(true);
        }
    }

    fn add_output(&mut self, other: usize) {
        self.outputs.push(other);
    }

    fn add_input(&mut self, _other: usize) {
        //do nothing
    }

    fn get_output_iter(&self) -> std::slice::Iter<'_, usize> {
        return self.outputs.iter();
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn as_any(&mut self) -> &mut dyn Any {
        return self;
    }
}

struct Conjunction {
    id: usize,
    name: String,
    inputs: Vec<usize>,
    outputs: Vec<usize>,
    memory: Vec<bool>,
}

impl Conjunction {
    fn new(id: usize, name: String) -> Self {
        Self {
            id,
            name,
            inputs: vec![],
            outputs: vec![],
            memory: vec![],
        }
    }
}

impl Module for Conjunction {
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn receive_pulse(&mut self, high: bool, sender: usize) -> Option<bool> {
        for (i, input) in self.inputs.iter().enumerate() {
            if *input == sender {
                self.memory[i] = high;
                break;
            }
        }
        if !high {
            return Some(true);
        }
        for value in self.memory.iter() {
            if !*value {
                return Some(true);
            }
        }

        return Some(false);
    }

    fn add_output(&mut self, other: usize) {
        self.outputs.push(other);
    }

    fn add_input(&mut self, other: usize) {
        self.inputs.push(other);
        self.memory.push(false);
    }
    fn get_output_iter(&self) -> std::slice::Iter<'_, usize> {
        return self.outputs.iter();
    }

    fn as_any(&mut self) -> &mut dyn Any {
        return self;
    }
}

struct Broadcast {
    id: usize,
    name: String,
    outputs: Vec<usize>,
}
impl Broadcast {
    fn new(id: usize, name: String) -> Self {
        Broadcast {
            id,
            name,
            outputs: Vec::new(),
        }
    }
}

impl Module for Broadcast {
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn receive_pulse(&mut self, high: bool, sender: usize) -> Option<bool> {
        return Some(high);
    }

    fn add_output(&mut self, other: usize) {
        self.outputs.push(other);
    }

    fn add_input(&mut self, other: usize) {
        //do nothing
    }
    fn get_output_iter(&self) -> std::slice::Iter<'_, usize> {
        return self.outputs.iter();
    }
    fn as_any(&mut self) -> &mut dyn Any {
        return self;
    }
}

struct Sink {
    id: usize,
    name: String,
    outputs: Vec<usize>,
}
impl Sink {
    fn new(id: usize, name: String) -> Self {
        Sink {
            id,
            name,
            outputs: Vec::new(),
        }
    }
}

impl Module for Sink {
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn receive_pulse(&mut self, high: bool, sender: usize) -> Option<bool> {
        return None;
    }

    fn add_output(&mut self, other: usize) {

    }

    fn add_input(&mut self, other: usize) {
        //do nothing
    }
    fn get_output_iter(&self) -> std::slice::Iter<'_, usize> {
        return self.outputs.iter();
    }
    fn as_any(&mut self) -> &mut dyn Any {
        return self;
    }
}

fn parse(input: &str) -> (usize, Vec<Box<(dyn Module + 'static)>>) {
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut connections = Vec::new();
    let mut modules: Vec<Box<dyn Module>> = Vec::new();
    let mut broadcaster_id = 0;
    for (i, line) in input.trim().lines().enumerate() {
        if let Some((name, outputs)) = line.split_once(" -> ") {
            let parsed_name: String;
            match name {
                "broadcaster" => {
                    parsed_name = name.to_string();
                    broadcaster_id = i;
                    modules.push(Box::new(Broadcast::new(i, parsed_name.clone())));
                }
                s if s.starts_with("%") => {
                    parsed_name = name[1..].to_string();
                    modules.push(Box::new(FlipFlop::new(i, parsed_name.clone())));
                }
                s if s.starts_with("&") => {
                    parsed_name = name[1..].to_string();
                    modules.push(Box::new(Conjunction::new(i, parsed_name.clone())));
                }
                _ => parsed_name = name.to_string(),
            }
            map.insert(parsed_name, i);
            connections.push(outputs);
        }
    }
    for outputs in connections.iter(){
        for output in outputs.split(", ") {
            if map.contains_key(&output.to_string()){
                continue;
            }
            map.insert(output.to_string(), modules.len());
            modules.push(
                Box::new(
                    Sink::new(modules.len(), output.to_string())
                )
            )
        }
    }
    let mut adj_matrix: Vec<Vec<bool>> = vec![vec![false; modules.len()];modules.len()];
    for (i, outputs) in connections.iter().enumerate() {
        for output in outputs.split(", ") {
            adj_matrix[i][map[output]] = true;
        }
    }
    for (i, module) in modules.iter_mut().enumerate() {
        for (other, value) in adj_matrix[i].iter().enumerate() {
            if *value {
                module.add_output(other)
            }
        }
        if let Some(conjunction) = module.as_any().downcast_mut::<Conjunction>() {
            for other in 0..adj_matrix.len() {
                if adj_matrix[other][i] {
                    module.add_input(other);
                }
            }
        }
    }

    return (broadcaster_id, modules);
}

pub fn part_one(input: &str) -> Option<u128> {
    let (broadcaster_id, mut modules) = parse(input);
    let mut pulses: VecDeque<(bool, usize, usize)> = VecDeque::new();
    let mut low_pulses : usize = 0;
    let mut high_pulses : usize = 0;
    for _ in 0..1000 {
        pulses.push_back((false, broadcaster_id, broadcaster_id));
        while !pulses.is_empty() {
            if let Some((strength, sender, destination)) = pulses.pop_front() {
                if strength {
                    high_pulses += 1;
                } else {
                    low_pulses += 1;
                }
                // if strength{
                //     println!("{} -high-> {}", modules[sender].get_name(), modules[destination].get_name());
                // } else {
                //     println!("{} -low-> {}", modules[sender].get_name(), modules[destination].get_name());
                // }
                if let Some(output_strength) = modules[destination].receive_pulse(strength, sender)
                {
                    for new_destination in modules[destination].get_output_iter() {
                        pulses.push_back((output_strength, destination, *new_destination))
                    }
                }
            }
        }
    }
    println!("low: {} high: {} multiplied: {}", low_pulses,high_pulses, low_pulses * high_pulses);
    return Some((low_pulses * high_pulses) as u128);
}

pub fn part_two(input: &str) -> Option<u128> {
    let (broadcaster_id, mut modules) = parse(input);
    let mut pulses: VecDeque<(bool, usize, usize)> = VecDeque::new();
    let mut low_pulses : usize = 0;
    let mut high_pulses : usize = 0;
    let mut loops = 0;
    let mut xj = None;
    let mut qs = None;
    let mut kz = None;
    let mut km = None;
    loop{
        loops += 1;
        pulses.push_back((false, broadcaster_id, broadcaster_id));
        while !pulses.is_empty() {
            if let Some((strength, sender, destination)) = pulses.pop_front() {
                if strength {
                    high_pulses += 1;
                } else {
                    low_pulses += 1;
                    match &*modules[destination].get_name() {
                        "xj" => {
                            if xj == None{
                                xj = Some(loops);
                            }
                        }
                        "qs" => {
                            if qs == None{
                                qs = Some(loops);
                            }
                        }
                        "kz" => {
                            if kz == None{
                                kz = Some(loops);
                            }
                        }
                        "km" => {
                            if km == None{
                                km = Some(loops);
                            }
                        }
                        _ => {}
                    }

                    match (xj, qs, kz, km){
                        (Some(a), Some(b), Some(c), Some(d)) => {
                            return Some((a as u128) * (b as u128) * (c as u128) * (d as u128));
                        }
                        _ => {}
                    }
                }
                // if strength{
                //     println!("{} -high-> {}", modules[sender].get_name(), modules[destination].get_name());
                // } else {
                //     println!("{} -low-> {}", modules[sender].get_name(), modules[destination].get_name());
                // }
                if let Some(output_strength) = modules[destination].receive_pulse(strength, sender)
                {
                    for new_destination in modules[destination].get_output_iter() {
                        pulses.push_back((output_strength, destination, *new_destination))
                    }
                }
            }
        }
    }
    // return Some(loops);
    // return Some((low_pulses * high_pulses) as u128);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

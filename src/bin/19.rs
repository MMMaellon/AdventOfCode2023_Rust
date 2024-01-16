use std::collections::HashMap;

advent_of_code::solution!(19);

#[derive(Clone)]
struct Part {
    x: i128,
    m: i128,
    a: i128,
    s: i128,
}

#[derive(Debug, Clone)]
enum Comparison {
    GreaterThan,
    LessThan,
    Always,
}

#[derive(Debug, Clone)]
enum Destination {
    Other(String),
    Accept,
    Reject,
    Pass,
}

#[derive(Debug, Clone)]
struct Rule {
    accessor: fn(part: &Part) -> i128,
    check: Comparison,
    check_value: i128,
    destination: Destination,
}

impl Rule {
    const X: fn(part: &Part) -> i128 = |part| part.x;
    const M: fn(part: &Part) -> i128 = |part| part.m;
    const A: fn(part: &Part) -> i128 = |part| part.a;
    const S: fn(part: &Part) -> i128 = |part| part.s;
    fn from_letter(s: &str) -> fn(part: &Part) -> i128 {
        match s {
            "x" => {
                return Rule::X;
            }
            "m" => {
                return Rule::M;
            }
            "a" => {
                return Rule::A;
            }
            _ => {
                return Rule::S;
            }
        }
    }
    fn run(&self, part: &Part) -> Destination {
        match self.check {
            Comparison::GreaterThan => {
                if (self.accessor)(part) > self.check_value {
                    return self.destination.clone();
                };
            }
            Comparison::LessThan => {
                if (self.accessor)(part) < self.check_value {
                    return self.destination.clone();
                };
            }
            Comparison::Always => {
                return self.destination.clone();
            }
        }
        return Destination::Pass;
    }
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

fn parse(input: &str) -> (Vec<Workflow>, Vec<Part>) {
    let mut output = Vec::new();
    let mut parts = Vec::new();
    let mut finished_rules = false;
    for line in input.lines() {
        if finished_rules {
            let mut part = Part {
                x: 0,
                m: 0,
                a: 0,
                s: 0,
            };
            for category_str in line[1..(line.len() - 1)].split(',') {
                if let Some((category, value_str)) = category_str.split_once('=') {
                    let value = value_str.parse::<i128>().unwrap();
                    match category {
                        "x" => part.x = value,
                        "m" => part.m = value,
                        "a" => part.a = value,
                        _ => part.s = value,
                    }
                }
            }
            parts.push(part);
            continue;
        } else {
            finished_rules = line.trim().is_empty();
            if finished_rules {
                continue;
            }
        }
        if let Some((name, rules)) = line.split_once('{') {
            let mut new_workflow = Workflow {
                name: name.to_string(),
                rules: Vec::new(),
            };
            for rule in rules[0..(rules.len() - 1)].split(',') {
                let mut new_rule = Rule {
                    check: Comparison::Always,
                    check_value: 0,
                    accessor: Rule::X,
                    destination: Destination::Pass,
                };
                if let Some((left_side, other_name)) = rule.split_once(':') {
                    if let Some((category, remainder)) = left_side.split_once(['<']) {
                        new_rule.accessor = Rule::from_letter(category);
                        new_rule.check_value = remainder.parse::<i128>().unwrap();
                        new_rule.check = Comparison::LessThan;
                    } else if let Some((category, remainder)) = left_side.split_once(['>']) {
                        new_rule.accessor = Rule::from_letter(category);
                        new_rule.check_value = remainder.parse::<i128>().unwrap();
                        new_rule.check = Comparison::GreaterThan;
                    }
                    match other_name {
                        "A" => new_rule.destination = Destination::Accept,
                        "R" => new_rule.destination = Destination::Reject,
                        _ => {
                            new_rule.destination = Destination::Other(other_name.to_string());
                        }
                    }
                } else if rule == "A" {
                    new_rule.check = Comparison::Always;
                    new_rule.destination = Destination::Accept;
                } else if rule == "R" {
                    new_rule.check = Comparison::Always;
                    new_rule.destination = Destination::Reject;
                } else {
                    new_rule.destination = Destination::Other(rule.to_string());
                }
                new_workflow.rules.push(new_rule);
            }
            output.push(new_workflow);
        }
    }
    return (output, parts);
}

fn map_from_vec(workflows: &Vec<Workflow>) -> HashMap<&String, usize> {
    let mut map = HashMap::new();
    for (i, workflow) in workflows.into_iter().enumerate() {
        map.insert(&workflow.name, i);
    }
    return map;
}

pub fn part_one(input: &str) -> Option<i128> {
    let (workflows, parts) = parse(input);
    let workflow_map = map_from_vec(&workflows);
    let mut sum = 0;
    for part in parts.iter() {
        let mut index = workflow_map[&("in".to_string())];
        let mut workflow = &workflows[index];
        loop {
            let mut chain_ended = false;
            for rule in workflow.rules.iter() {
                match rule.run(part) {
                    Destination::Other(other) => {
                        if !workflow_map.contains_key(&other) {
                            return None;
                        }
                        index = workflow_map[&other];
                        workflow = &workflows[index];
                        break;
                    }
                    Destination::Accept => {
                        //accepted
                        sum += part.x + part.m + part.a + part.s;
                        chain_ended = true;
                        break;
                    }
                    Destination::Reject => {
                        //rejected
                        chain_ended = true;
                        break;
                    }
                    _ => {}
                }
            }
            if chain_ended {
                break;
            }
        }
    }
    return Some(sum);
}

fn evaluate_part(
    part: &Part,
    workflows: &Vec<Workflow>,
    workflow_map: &HashMap<&String, usize>,
) -> bool {
    let mut index = workflow_map[&("in".to_string())];
    let mut workflow = &workflows[index];
    loop {
        for rule in workflow.rules.iter() {
            match rule.run(part) {
                Destination::Other(other) => {
                    index = workflow_map[&other];
                    workflow = &workflows[index];
                    break;
                }
                Destination::Accept => {
                    //accepted
                    return true;
                }
                Destination::Reject => {
                    //rejected
                    return false;
                }
                _ => {}
            }
        }
    }
}

fn djikstras(
    lower_bound: Part,
    upper_bound: Part,
    current_workflow_index: usize,
    current_rule_index: usize,
    workflows: &Vec<Workflow>,
    workflow_map: &HashMap<&String, usize>,
) -> u128 {
    if lower_bound.x > upper_bound.x
        || lower_bound.m > upper_bound.m
        || lower_bound.a > upper_bound.a
        || lower_bound.s > upper_bound.s
    {
        return 0;
    }
    let mut sum = 0;
    let current_workflow = &workflows[current_workflow_index];
    for rule_index in current_rule_index..current_workflow.rules.len() {
        println!("split on {}[{}]", current_workflow.name, rule_index);
        let rule = &current_workflow.rules[rule_index];
        match rule.check {
            Comparison::GreaterThan => {
                if (rule.accessor)(&upper_bound) <= rule.check_value {
                    //would always fail
                    continue;
                }
                //otherwise splits into two scenarios, what happens when you're above and what happens when you're below
                let mut new_bound_lower = upper_bound.clone();
                let mut new_bound_upper = lower_bound.clone();
                match rule.accessor {
                    Rule::X => {
                        new_bound_lower.x = rule.check_value;
                        new_bound_upper.x = rule.check_value + 1;
                    }
                    Rule::M => {
                        new_bound_lower.m = rule.check_value;
                        new_bound_upper.m = rule.check_value + 1;
                    }
                    Rule::A => {
                        new_bound_lower.a = rule.check_value;
                        new_bound_upper.a = rule.check_value + 1;
                    }
                    Rule::S => {
                        new_bound_lower.s = rule.check_value;
                        new_bound_upper.s = rule.check_value + 1;
                    }
                    _ => {}
                }

                match &rule.destination {
                    Destination::Other(name) => {
                        let next_workflow_index = workflow_map[&name];
                        sum += djikstras(
                            new_bound_upper,
                            upper_bound.clone(),
                            next_workflow_index,
                            0,
                            workflows,
                            workflow_map,
                        );
                    }
                    Destination::Accept => {
                        println!("new bounds {}..{} {}..{} {}..{} {}..{}", &new_bound_upper.x, &upper_bound.x, &new_bound_upper.m, &upper_bound.m, &new_bound_upper.a, &upper_bound.a, &new_bound_upper.s, &upper_bound.s);
                        sum += get_bounded_size(&new_bound_upper, &upper_bound);
                        println!("sum {}", sum);
                    }
                    Destination::Reject => {
                        sum += 0;
                    }
                    Destination::Pass => {
                        //there really shouldn't be any in here
                        println!("fuck");
                        return 0;
                    }
                }
                sum += djikstras(
                    lower_bound.clone(),
                    new_bound_lower,
                    current_workflow_index,
                    rule_index + 1,
                    workflows,
                    workflow_map,
                );
                return sum;
            }
            Comparison::LessThan => {
                println!("less than");
                if (rule.accessor)(&lower_bound) >= rule.check_value {
                    //would always fail
                    continue;
                }
                let mut new_bound_lower = upper_bound.clone();
                let mut new_bound_upper = lower_bound.clone();
                match rule.accessor {
                    Rule::X => {
                        new_bound_lower.x = rule.check_value - 1;
                        new_bound_upper.x = rule.check_value;
                    }
                    Rule::M => {
                        new_bound_lower.m = rule.check_value - 1;
                        new_bound_upper.m = rule.check_value;
                    }
                    Rule::A => {
                        new_bound_lower.a = rule.check_value - 1;
                        new_bound_upper.a = rule.check_value;
                    }
                    Rule::S => {
                        new_bound_lower.s = rule.check_value - 1;
                        new_bound_upper.s = rule.check_value;
                    }
                    _ => {}
                }
                match &rule.destination {
                    Destination::Other(name) => {
                        let next_workflow_index = workflow_map[&name];
                        sum += djikstras(
                            lower_bound.clone(),
                            new_bound_lower,
                            next_workflow_index,
                            0,
                            workflows,
                            workflow_map,
                        );
                    }
                    Destination::Accept => {
                        sum += get_bounded_size(&lower_bound, &new_bound_lower);
                    }
                    Destination::Reject => {
                        sum += 0;
                    }
                    Destination::Pass => {
                        //there really shouldn't be any in here
                        println!("fuck");
                        return 0;
                    }
                }
                sum += djikstras(
                    new_bound_upper,
                    upper_bound.clone(),
                    current_workflow_index,
                    rule_index + 1,
                    workflows,
                    workflow_map,
                );
                return sum;
            }
            Comparison::Always => {
                match &rule.destination {
                    Destination::Other(name) => {
                        sum += djikstras(
                            lower_bound.clone(),
                            upper_bound.clone(),
                            workflow_map[&name],
                            0,
                            workflows,
                            workflow_map,
                        );
                        return sum;
                    }
                    Destination::Accept => {
                        sum += get_bounded_size(&lower_bound, &upper_bound);
                        return sum;
                    }
                    Destination::Reject => {
                        return sum;
                    }
                    Destination::Pass => {
                        //there really shouldn't be any in here
                        println!("fuck");
                        return 0;
                    }
                }
            }
        }
    }
    return sum;
}

fn get_bounded_size(lower: &Part, upper: &Part) -> u128 {
    let mut sum = upper.x - lower.x + 1;
    sum *= upper.m - lower.m + 1;
    sum *= upper.a - lower.a + 1;
    sum *= upper.s - lower.s + 1;
    return sum as u128;
}

fn find_slices(workflows: &Vec<Workflow>) -> (Vec<i128>, Vec<i128>, Vec<i128>, Vec<i128>) {
    let mut x = vec![4000];
    let mut m = vec![4000];
    let mut a = vec![4000];
    let mut s = vec![4000];

    for workflow in workflows.iter() {
        for rule in workflow.rules.iter() {
            if rule.check_value >= 4000 {
                continue;
            }
            let check_value;
            match rule.check {
                Comparison::GreaterThan => check_value = rule.check_value,
                Comparison::LessThan => check_value = rule.check_value - 1,
                Comparison::Always => continue,
            }
            match rule.accessor {
                Rule::X => {
                    x.push(check_value);
                }
                Rule::M => {
                    m.push(check_value);
                }
                Rule::A => {
                    a.push(check_value);
                }
                Rule::S => {
                    s.push(check_value);
                }
                _ => {}
            }
        }
    }

    x.sort();
    m.sort();
    a.sort();
    s.sort();

    return (x, m, a, s);
}

pub fn part_two(input: &str) -> Option<u128> {
    let (workflows, _parts) = parse(input);
    let workflow_map = map_from_vec(&workflows);
    // let mut sum = 0;
    // let slices = find_slices(&workflows);
    // // let slices = (vec![4000], vec![4000], vec![4000], vec![2000, 4000]);
    //
    // let mut prev_x: i128 = 0;
    // let mut prev_m: i128;
    // let mut prev_a: i128;
    // let mut prev_s: i128;
    // for &x in slices.0.iter() {
    //     prev_m = 0;
    //     let section_size = (x - prev_x) as u128;
    //     for &m in slices.1.iter() {
    //         prev_a = 0;
    //         let section_size = section_size * (m - prev_m) as u128;
    //         for &a in slices.2.iter() {
    //             prev_s = 0;
    //             let section_size = section_size * (a - prev_a) as u128;
    //             for &s in slices.3.iter() {
    //                 let part = Part { x, m, a, s };
    //                 if true || evaluate_part(&part, &workflows, &workflow_map) {
    //                     let section_size = section_size * (s - prev_s) as u128;
    //                     sum += section_size;
    //                 }
    //                 prev_s = s;
    //             }
    //             prev_a = a;
    //         }
    //         prev_m = m;
    //     }
    //     prev_x = x;
    // }
    // Some(sum)

    let lower_bound = Part {
        x: 1,
        m: 1,
        a: 1,
        s: 1,
    };
    let upper_bound = Part {
        x: 4000,
        m: 4000,
        a: 4000,
        s: 4000,
    };
    // let lower = Part {
    //     x: 1,
    //     m: 1549,
    //     a: 1,
    //     s: 1,
    // };
    // let upper = Part {
    //     x: 4000,
    //     m: 4000,
    //     a: 4000,
    //     s: 4000,
    // };
    // println!("bounded_size: {}", get_bounded_size(&lower, &upper));
    let start_workflow_index = workflow_map[&("in".to_string())];
    Some(djikstras(
        lower_bound,
        upper_bound,
        start_workflow_index,
        0,
        &workflows,
        &workflow_map,
    ) as u128)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}

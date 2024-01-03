advent_of_code::solution!(15);

fn hash(input: &str) -> u32 {
    let mut current_value : u32 = 0;
    for c in input.chars(){
        let full_num : u128 = 17 * (current_value as u128 + c as u128);
        current_value = (full_num % 256) as u32;
    }
    return current_value;
}

fn print_boxes(boxes: &Vec<Vec<(String, u32)>>){
    for (i, b) in boxes.into_iter().enumerate(){
        if b.is_empty(){
            continue;
        }
        print!("Box {}:", i);
        for (label, lens) in b.into_iter(){
            print!(" [{} {}]", label, lens);
        }
        println!("");
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for step in input.split(','){
        sum += hash(step.trim());
    }
    return Some(sum);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut boxes : Vec<Vec<(String, u32)>> = vec![Vec::new();256];
    for step in input.trim().split(','){
        let label : &String = &step.trim_end_matches(|x| {x == '=' || x == '-' || char::is_numeric(x)}).to_string();
        let i = hash(label) as usize;
        let operation = &step.chars().nth(label.len());
        match operation{
            Some('=') => {
                let lens = &step.chars().last().unwrap().to_string().parse::<u32>().unwrap();
                let mut matched = false;
                for slot in boxes[i].iter_mut(){
                    if slot.0 == *label{
                        matched = true;
                        slot.1 = *lens;
                        break;
                    }
                }
                if !matched{
                    boxes[i].push(((*label).clone(), *lens));
                }
            }
            Some('-') => {
                let new_box = boxes[i].clone();
                boxes[i].clear();
                for slot in new_box.into_iter(){
                    if slot.0 != *label{
                        boxes[i].push(slot);
                    }
                }
            }
            _ => {}
        }
    }
    print_boxes(&boxes);
    let mut sum = 0;
    for (i, b) in boxes.into_iter().enumerate(){
        for (j, (_,lens_power)) in b.into_iter().enumerate(){
            sum += (i as u32 + 1) * (j as u32 + 1) * lens_power;
            // println!("= {}", (i as u32 + 1) + (j as u32 + 1) + lens_power);
        }
    }
    return Some(sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}

use std::fmt;
advent_of_code::solution!(18);

struct Step {
    x: i32,
    y: i32,
    color: u32,
}

impl fmt::Debug for Step {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{({}, {}), {:06x}}}", self.x, self.y, self.color)
    }
}

fn parse(input: &str) -> Vec<Step> {
    let mut output = Vec::new();
    for line in input.lines() {
        if let Some((direction, remainder)) = line.split_once(' ') {
            if let Some((count_str, remainder)) = remainder.split_once(' ') {
                if let Ok(count) = count_str.parse::<i32>() {
                    if let Ok(color) =
                        u32::from_str_radix(remainder.trim_matches(|x : char| !x.is_alphanumeric()), 16)
                    {
                        let mut step = Step {
                            x: count,
                            y: 0,
                            color,
                        };
                        match direction {
                            "U" => {
                                step.y = -count;
                                step.x = 0;
                            }
                            "D" => {
                                step.y = count;
                                step.x = 0;
                            }
                            "L" => {
                                step.x = -count;
                            }
                            _ => {}
                        }
                        output.push(step);
                    }

        } else {
            println!("bad parse {}", line);
                }
        } else {
            println!("bad count {}", line);
            }
        } else {
            println!("bad direction {}", line);
        }
    }
    return output;
}

fn calc_area(steps: &Vec<Step>) -> usize{
    let mut height = 0;
    let mut count: i32 = 0;
    let mut last_horizontal = false;
    for step in steps.into_iter(){
        if step.y != 0{
            height += step.y;
        } else if last_horizontal{
            if step.x > 0{
                count += height * step.x;
            } else {
                count += (height - 1) * step.x;
            }
        } else if step.x > 0{
            count += height * (step.x - 1);
        } else {
            count += (height - 1) * (step.x + 1);
        }
        last_horizontal = step.y == 0;
        println!("height {} count {}", height, count);
    }
    return count.abs() as usize;
}
fn calc_perimeter(steps: &Vec<Step>) -> usize{
    let mut count = 0;
    for step in steps.into_iter(){
        if step.y != 0{
            count += step.y.abs();
        } else {
            count += step.x.abs();
        }
    }
    return count as usize;
}
pub fn part_one(input: &str) -> Option<u32> {
    let steps = parse(input);
    println!("perimeter {}", calc_perimeter(&steps));
    println!("area {}", calc_area(&steps));
    println!("both {}", calc_perimeter(&steps) + calc_area(&steps));
    return Some(calc_perimeter(&steps) as u32 + calc_area(&steps) as u32);
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

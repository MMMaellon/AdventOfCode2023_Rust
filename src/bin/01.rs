// use std::str::pattern::Pattern;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut num_str: String;
    let mut sum: u32 = 0;
    for line in input.lines() {
        num_str = "".to_string();
        for c in line.chars() {
            if c.is_numeric() {
                if num_str.len() == 2 {
                    num_str.replace_range(1.., &c.to_string())
                } else {
                    num_str.push(c);
                    num_str.push(c);
                }
            }
        }
        match num_str.parse::<u32>() {
            Ok(value) => {
                sum += value;
            }
            _ => {}
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut num_str: String;
    let mut word_str: String;
    let mut sum: u32 = 0;
    for line in input.lines() {
        num_str = "".to_string();
        word_str = "".to_string();
        for c in line.chars() {
            if c.is_numeric() {
                add_char(&mut num_str, c)
            } else {
                word_str.push(c);
                match c {
                    'e' => {
                        if word_str.ends_with("one") {
                            add_char(&mut num_str, '1');
                        } else if word_str.ends_with("three") {
                            add_char(&mut num_str, '3');
                        } else if word_str.ends_with("five") {
                            add_char(&mut num_str, '5');
                        } else if word_str.ends_with("nine") {
                            add_char(&mut num_str, '9');
                        }
                    }
                    'o' => {
                        if word_str.ends_with("two") {
                            add_char(&mut num_str, '2');
                        }
                    }
                    'r' => {
                        if word_str.ends_with("four") {
                            add_char(&mut num_str, '4');
                        }
                    }
                    'x' => {
                        if word_str.ends_with("six") {
                            add_char(&mut num_str, '6');
                        }
                    }
                    'n' => {
                        if word_str.ends_with("seven") {
                            add_char(&mut num_str, '7');
                        }
                    }
                    't' => {
                        if word_str.ends_with("eight") {
                            add_char(&mut num_str, '8');
                        }
                    }
                    _ => {}
                }
            }
        }
        match num_str.parse::<u32>() {
            Ok(value) => {
                sum += value;
            }
            _ => {}
        }
    }
    Some(sum)
}

fn add_char(num: &mut String, c: char) {
    if num.len() == 2 {
        num.replace_range(1.., &c.to_string())
    } else {
        num.push(c);
        num.push(c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}

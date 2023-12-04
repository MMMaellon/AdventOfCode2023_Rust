use std::str::pattern::Pattern;

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
        sum += num_str.parse::<u32>().unwrap();
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
                if num_str.len() == 2 {
                    num_str.replace_range(1.., &c.to_string())
                } else {
                    num_str.push(c);
                    num_str.push(c);
                }
            } else {
                word_str.push(c);
                match c {
                    'e' => {
                        if num_str.ends_with("one") {
                        } else if num_str.ends_with("three") {
                        } else if num_str.ends_with("five") {
                        } else if num_str.ends_with("nine") {
                        }
                    }
                    'o' => {
                        if num_str.ends_with("two") {

                        }
                    }
                    'r' => {
                        if word_str.ends_with("four"){

                        }
                    }
                    'x' => {
                        if word_str.ends_with("six"){

                        }
                    }
                    'n' => {
                        if word_str.ends_with("seve"){

                        }
                    }
                }
            }
        }
        sum += num_str.parse::<u32>().unwrap();
    }
    Some(sum)
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
        assert_eq!(result, None);
    }
}

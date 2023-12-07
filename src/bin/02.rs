advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;
    let (mut id, mut count, mut red, mut green, mut blue): (u32, u32, u32, u32, u32);
    let mut parts;
    let mut hand_parts;
    let mut passed: bool;
    let condition = (12, 13, 14);
    for line in input.lines() {
        parts = line.split_once(": ");
        id = parts.unwrap().0[5..].parse::<u32>().unwrap();
        passed = true;
        for game in parts.unwrap().1.split("; ") {
            if !passed {
                break;
            }
            (red, green, blue) = (0, 0, 0);
            for hand in game.split(", ") {
                hand_parts = hand.split_once(' ').unwrap();
                count = hand_parts.0.parse::<u32>().unwrap();
                match hand_parts.1 {
                    "red" => {
                        red += count;
                        if red > condition.0 {
                            passed = false;
                            break;
                        }
                    }
                    "green" => {
                        green += count;
                        if green > condition.1 {
                            passed = false;
                            break;
                        }
                    }
                    "blue" => {
                        blue += count;
                        if blue > condition.2 {
                            passed = false;
                            break;
                        }
                    }
                    _ => {
                        panic!("invalid color");
                    }
                }
            }
        }
        if passed {
            sum += id;
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;
    let (mut count, mut red, mut green, mut blue): (u32, u32, u32, u32);
    let mut parts;
    let mut hand_parts;
    let (mut min_red, mut min_green, mut min_blue): (u32, u32, u32) = (999, 999, 999);
    for line in input.lines() {
        parts = line.split_once(": ");
        (min_red, min_green, min_blue) = (0, 0, 0);
        for game in parts.unwrap().1.split("; ") {
            (red, green, blue) = (0, 0, 0);
            for hand in game.split(", ") {
                hand_parts = hand.split_once(' ').unwrap();
                count = hand_parts.0.parse::<u32>().unwrap();
                match hand_parts.1 {
                    "red" => {
                        red += count;
                    }
                    "green" => {
                        green += count;
                    }
                    "blue" => {
                        blue += count;
                    }
                    _ => {
                        panic!("invalid color");
                    }
                }
            }
            if red > min_red {
                min_red = red;
            }
            if green > min_green {
                min_green = green;
            }
            if blue > min_blue {
                min_blue = blue;
            }
        }
        sum += min_red * min_green * min_blue;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
